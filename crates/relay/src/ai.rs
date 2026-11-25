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
        "enemy_heroes": r.enemies,
        "enemies_who_killed_us_most": r.killers.iter().take(3).map(|(h, n)| format!("{h} ({n})")).collect::<Vec<_>>(),
        "fights": r.fights.iter().map(|f| json!({
            "at": f.clock, "result": f.result, "our_kills": f.our_kills, "our_deaths": f.our_deaths,
            "present": f.present, "absent": f.absent,
        })).collect::<Vec<_>>(),
        "players": r.players.iter().map(|p| json!({
            "name": p.name, "hero": p.hero_name, "nickname": p.title,
            "ran_the_app": p.app,
            // Each fact with where it came from and how it compares with this player's usual.
            "facts": p.cards.iter().map(|c| {
                let mut s = format!("{} [{}; {}]", c.text, c.tone, c.source);
                if let (Some(u), Some(t)) = (&c.usual, &c.trend) {
                    s.push_str(&format!(" ({u}; {t} than usual)"));
                }
                s
            }).collect::<Vec<_>>(),
            "worst_moments": p.worst.iter().map(|m| format!("{} {}", m.clock, m.text)).collect::<Vec<_>>(),
            "best_moments": p.best.iter().map(|m| format!("{} {}", m.clock, m.text)).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
    })
}

pub fn prompt(r: &MatchReport, language: &str) -> String {
    format!(
        "You are the brutally honest but friendly coach of a 5-friend Dota 2 team. \
         Their main problem is team coordination, and they read this together to learn as a team. \
         Below are facts from their last game: how each player helped the team, not Dota's scoreboard. \
         'replay' facts come from the real replay, 'measured' from their own tracking app, \
         'estimate' are rough. Where a fact says better/worse than usual, that compares with the \
         player's own earlier games.\n\n\
         Write in {language}, plain text, **bold** names only, under 380 words, no preamble:\n\
         HELPED MOST: who did the most for the team and the exact moments that show it.\n\
         WORST MOVES: the 2-3 worst actions of the game, each with the player, the game time and why it hurt the team.\n\
         EVERYONE: for each player in the given order, one line: what to repeat or what to fix next game; \
         mention it when they were clearly better or worse than their usual.\n\
         TEAM LESSON: one thing all five should do differently next game.\n\
         Be specific, use the numbers, tease a little like friends do, never insult. Never invent \
         facts that are not in the report. Only compare with a player's usual where a fact says \
         'better than usual' or 'worse than usual'; without that, say nothing about their usual.\n\nREPORT:\n{}",
        serde_json::to_string_pretty(&compact(r)).unwrap_or_default()
    )
}

/// Asks the model; returns the review text.
/// The model to use: the configured one if Ollama has it; otherwise the
/// server's "-agent" model, otherwise its largest. Models on the server get
/// upgraded (qwen3.6-agent became qwen3.8-agent), and the coach should
/// keep working without a config change.
async fn pick_model(client: &reqwest::Client, cfg: &AiConfig) -> String {
    #[derive(serde::Deserialize)]
    struct Tags {
        models: Vec<Tag>,
    }
    #[derive(serde::Deserialize)]
    struct Tag {
        name: String,
        #[serde(default)]
        size: u64,
    }
    let url = format!("{}/api/tags", cfg.url.trim_end_matches('/'));
    let Ok(res) = client.get(url).timeout(Duration::from_secs(5)).send().await else { return cfg.model.clone() };
    let Ok(tags) = res.json::<Tags>().await else { return cfg.model.clone() };
    choose(&cfg.model, &tags.models.iter().map(|t| (t.name.as_str(), t.size)).collect::<Vec<_>>())
}

pub fn choose(wanted: &str, models: &[(&str, u64)]) -> String {
    let bare = |n: &str| n.trim_end_matches(":latest").to_string();
    if models.iter().any(|(n, _)| bare(n) == bare(wanted)) || models.is_empty() {
        return wanted.to_string();
    }
    let pick = models
        .iter()
        .filter(|(n, _)| bare(n).ends_with("-agent"))
        .max_by_key(|(_, s)| *s)
        .or_else(|| models.iter().max_by_key(|(_, s)| *s))
        .map(|(n, _)| n.to_string())
        .unwrap_or_else(|| wanted.to_string());
    tracing::warn!("AI model {wanted:?} not on the server; using {pick:?}");
    pick
}

pub async fn review(cfg: &AiConfig, r: &MatchReport) -> Result<String, String> {
    if cfg.url.is_empty() {
        return Err("AI review is switched off".into());
    }
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(600))
        .build()
        .map_err(|e| e.to_string())?;
    let model = pick_model(&client, cfg).await;
    let body = json!({
        "model": model,
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

#[cfg(test)]
mod tests {
    use super::choose;

    #[test]
    fn falls_back_to_the_servers_agent_model() {
        let now = [("qwen3.8-agent:latest", 17), ("llava:latest", 4), ("qwen2.5:14b", 8)];
        assert_eq!(choose("qwen3.6-agent", &now), "qwen3.8-agent:latest");
        assert_eq!(choose("qwen2.5:14b", &now), "qwen2.5:14b");
        assert_eq!(choose("qwen3.8-agent", &now), "qwen3.8-agent");
        assert_eq!(choose("x", &[("a", 1), ("b", 9)]), "b");
        assert_eq!(choose("x", &[]), "x");
    }
}
