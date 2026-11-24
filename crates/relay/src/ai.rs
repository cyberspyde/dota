//! Post-game review written by a local language model (Ollama). It only sees
//! the computed report: names, heroes, metrics and moments. The host's private
//! coaching notes are never part of the prompt.

use crate::analysis::MatchReport;
use serde_json::json;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct AiConfig {
    /// Ollama base URL, e.g. http://127.0.0.1:11434 ; empty = disabled.
    pub url: String,
    pub model: String,
    pub language: String,
}

fn compact(r: &MatchReport) -> serde_json::Value {
    json!({
        "result": r.summary,
        "duration": crate::analysis::clock(r.duration),
        "fights": r.fights.iter().map(|f| json!({
            "at": f.clock, "result": f.result, "our_kills": f.our_kills, "our_deaths": f.our_deaths,
            "present": f.present, "absent": f.absent,
        })).collect::<Vec<_>>(),
        "players": r.players.iter().map(|p| json!({
            "name": p.name, "hero": p.hero_name, "kda": format!("{}/{}/{}", p.k, p.d, p.a),
            "usefulness": p.usefulness, "grade": p.grade, "title": p.title,
            "fights": format!("{}/{}", p.metrics.fights_present, p.metrics.fights_total),
            "missed_fights_with_tp_ready": p.metrics.fights_missed_tp_ready,
            "help_calls_answered": format!("{}/{}", p.metrics.calls_answered, p.metrics.calls_total),
            "deaths_alone": p.metrics.deaths_alone,
            "deaths_with_items_ready": p.metrics.deaths_with_items_ready,
            "deaths_after_retreat_call": p.metrics.deaths_after_retreat_call,
            "seconds_hoarding_2k_gold": p.metrics.hoard_s,
            "time_alone_pct": p.metrics.alone_pct.round(),
            "observers_placed": p.metrics.obs_placed,
            "worst_moments": p.worst.iter().map(|m| format!("{} {}", m.clock, m.text)).collect::<Vec<_>>(),
            "best_moments": p.best.iter().map(|m| format!("{} {}", m.clock, m.text)).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
    })
}

pub fn prompt(r: &MatchReport, language: &str) -> String {
    format!(
        "You are the brutally honest but friendly coach of a 5-friend Dota 2 team. \
         Their main problem is team coordination. Below is a computed report of their last game \
         (only facts measured by their tool; Dota's own scoreboard stats are deliberately left out).\n\n\
         Write the post-game review in {language}. Format:\n\
         1. One headline sentence about how the team played together.\n\
         2. For each player, in the given order: their name in bold, 2 short sentences on how useful \
         they were to the team, their single dumbest moment quoted with its game time, and one concrete \
         fix for the next game.\n\
         3. One team-wide lesson.\n\
         Be specific, use the numbers, tease a little like friends do, never insult. No preamble, \
         under 350 words, plain text with **bold** names only.\n\nREPORT:\n{}",
        serde_json::to_string_pretty(&compact(r)).unwrap_or_default()
    )
}

/// Asks the model; returns the review text.
pub async fn review(cfg: &AiConfig, r: &MatchReport) -> Result<String, String> {
    if cfg.url.is_empty() {
        return Err("AI review is switched off".into());
    }
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(600))
        .build()
        .map_err(|e| e.to_string())?;
    let body = json!({
        "model": cfg.model,
        "stream": false,
        "think": false,
        "options": { "temperature": 0.7, "num_ctx": 16384 },
        "messages": [{ "role": "user", "content": prompt(r, &cfg.language) }],
    });
    let res = client
        .post(format!("{}/api/chat", cfg.url.trim_end_matches('/')))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("model unreachable: {e}"))?;
    let status = res.status();
    let v: serde_json::Value = res.json().await.map_err(|e| format!("bad model reply: {e}"))?;
    if !status.is_success() {
        return Err(format!("model error {status}: {}", v["error"].as_str().unwrap_or("")));
    }
    let text = v["message"]["content"].as_str().unwrap_or("").trim().to_string();
    // Some models still emit a thinking block; keep only the answer.
    let text = match text.rfind("</think>") {
        Some(i) => text[i + 8..].trim().to_string(),
        None => text,
    };
    if text.is_empty() {
        return Err("model returned nothing".into());
    }
    Ok(text)
}
