//! Each player's post-game card: the few lines of their report worth seeing
//! right after the game, pushed straight to their app. The full report stays
//! on the site.

use crate::{
    analysis::{MatchReport, PlayerReport},
    AppState,
};
use protocol::{now_ms, PostGameCard, ServerMsg};

/// Strips **bold** marks and leading list symbols from a line of the review.
fn plain(s: &str) -> String {
    s.replace("**", "").trim_start_matches(['-', '*', ' ', '•']).trim().to_string()
}

/// The coach's line for `name` from the EVERYONE part of the review, and the
/// TEAM LESSON. The review is free text, so this is forgiving: a missing
/// line just leaves the card without it.
pub fn coach_lines(review: &str, name: &str) -> (Option<String>, Option<String>) {
    let lines: Vec<&str> = review.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
    let upper = |l: &str| l.replace("**", "").trim().to_uppercase();
    let section = |head: &str| lines.iter().position(|l| upper(l).starts_with(head));
    let name_l = name.to_lowercase();

    let coach = section("EVERYONE").and_then(|start| {
        lines[start..]
            .iter()
            .skip(1)
            .take_while(|l| !upper(l).starts_with("TEAM LESSON"))
            .find(|l| l.to_lowercase().contains(&name_l))
            .map(|l| {
                let t = plain(l);
                // "Name: text" or "Name - text" -> "text"
                let low = t.to_lowercase();
                match low.find(&name_l) {
                    Some(0) => t.get(name.len()..).unwrap_or(&t).trim_start_matches([':', '-', '–', '—', ' ']).trim().to_string(),
                    _ => t,
                }
            })
            .filter(|t| !t.is_empty())
    });
    let lesson = section("TEAM LESSON").and_then(|i| {
        let head = plain(lines[i]);
        // Either on the heading line after a colon, or on the next line.
        let after = head.split_once(':').map(|(_, r)| r.trim().to_string()).filter(|r| !r.is_empty());
        after.or_else(|| lines.get(i + 1).map(|l| plain(l)))
    });
    (coach, lesson)
}

fn worst_first(p: &PlayerReport) -> Vec<(String, String)> {
    let mut v: Vec<(String, String)> = Vec::new();
    for tone in ["bad", "good", "neutral"] {
        for c in p.cards.iter().filter(|c| c.tone == tone) {
            let mut t = c.text.clone();
            if let (Some(u), Some(tr)) = (&c.usual, &c.trend) {
                if tr != "same" {
                    t.push_str(&format!(" ({tr} than usual: {u})"));
                }
            }
            v.push((c.tone.clone(), t));
        }
    }
    // Up to three: the biggest problems, then something done well.
    let bad: Vec<_> = v.iter().filter(|(t, _)| t == "bad").take(2).cloned().collect();
    let mut out = bad;
    out.extend(v.iter().filter(|(t, _)| t == "good").take(3 - out.len().min(3)).cloned());
    if out.len() < 3 {
        out.extend(v.iter().filter(|(t, _)| t == "neutral").take(3 - out.len()).cloned());
    }
    // Nothing good or neutral: show one more problem rather than a short card.
    if out.len() < 3 {
        out.extend(v.iter().filter(|(t, _)| t == "bad").skip(2).take(3 - out.len()).cloned());
    }
    out
}

/// "6 team fights: 4 won, 1 lost", or, when nobody else ran the app and so
/// no fights could be found, a plain note instead of a misleading "0".
fn team_line(r: &MatchReport, fw: usize, fl: usize) -> String {
    let apps = r.players.iter().filter(|p| p.app).count();
    if r.fights.is_empty() && apps < 2 {
        return "Only you ran the app: team fights show when two or more of you do".into();
    }
    format!("{} team fights: {fw} won, {fl} lost", r.fights.len())
}

/// The card for one player in this report, if they played in it.
pub fn card_for(r: &MatchReport, name: &str, ai_on: bool) -> Option<PostGameCard> {
    let p = r.players.iter().find(|p| p.name.eq_ignore_ascii_case(name.trim()))?;
    let (coach, lesson) = r.ai_review.as_deref().map(|rv| coach_lines(rv, &p.name)).unwrap_or((None, None));
    let fw = r.fights.iter().filter(|f| f.result == "won").count();
    let fl = r.fights.iter().filter(|f| f.result == "lost").count();
    Some(PostGameCard {
        match_id: r.match_id.clone(),
        won: r.won,
        duration: r.duration,
        team_line: team_line(r, fw, fl),
        facts: worst_first(p),
        worst: p.worst.first().map(|m| format!("{} {}", m.clock, m.text)),
        best: p.best.first().map(|m| format!("{} {}", m.clock, m.text)),
        coach,
        lesson,
        coach_pending: ai_on && r.ai_status == "writing",
        replay_facts: r.od.state == "done",
    })
}

/// Sends every connected player who was in this game their card.
pub fn push(app: &AppState, r: &MatchReport) {
    let ai_on = app.ai.enabled();
    for (id, typed) in app.hub.players() {
        let name = crate::profiles::canonical(app.team.as_deref(), &typed);
        if let Some(card) = card_for(r, &name, ai_on) {
            let frame = serde_json::to_string(&ServerMsg::PostGame { card }).expect("serialize").into();
            app.hub.send_to(id, &frame);
        }
    }
}

/// On connect: the card from a game that ended in the last half hour, so a
/// player who restarted the app after the game still sees it.
pub fn recent_card(app: &AppState, name: &str) -> Option<PostGameCard> {
    let r = app.store.history(1).into_iter().next()?;
    let ended_ms = r.started_ms + r.duration.max(0) as u64 * 1000;
    if now_ms().saturating_sub(ended_ms) > 45 * 60_000 {
        return None;
    }
    card_for(&r, name, app.ai.enabled())
}

#[cfg(test)]
mod tests {
    use super::*;

    const REVIEW: &str = "HELPED MOST: **Azizbek** kept the team alive.\n\n\
        WORST MOVES:\n- **Azimjon** at 18:20 died alone.\n\n\
        EVERYONE:\n**Azizbek**: Place wards earlier.\n- **Azimjon** - Click TP when the team fights.\n\n\
        TEAM LESSON: Group before Roshan.";

    #[test]
    fn finds_the_players_line_and_lesson() {
        let (c, l) = coach_lines(REVIEW, "Azimjon");
        assert_eq!(c.as_deref(), Some("Click TP when the team fights."));
        assert_eq!(l.as_deref(), Some("Group before Roshan."));
        let (c, _) = coach_lines(REVIEW, "azizbek");
        assert_eq!(c.as_deref(), Some("Place wards earlier."));
        assert_eq!(coach_lines(REVIEW, "Otabek").0, None);
    }

    #[test]
    fn em_dash_after_the_name_is_stripped() {
        let review = "EVERYONE:\n**Ilhom** — Keep placing wards.\n\nTEAM LESSON: Group up.";
        assert_eq!(coach_lines(review, "Ilhom").0.as_deref(), Some("Keep placing wards."));
    }

    #[test]
    fn solo_card_says_why_there_are_no_fights() {
        let mut rec = crate::analysis::tests::scenario();
        rec.tracks.retain(|k, _| k == "C");
        let r = crate::analysis::analyze(&rec);
        let c = card_for(&r, "C", false).unwrap();
        assert!(c.team_line.starts_with("Only you ran the app"), "{}", c.team_line);
        assert!(!c.facts.iter().any(|(_, t)| t.contains("teammate") || t.contains("team fights")), "{:?}", c.facts);
        assert!(!c.facts.is_empty());
    }

    #[test]
    fn card_has_worst_first() {
        let r = crate::analysis::analyze(&crate::analysis::tests::scenario());
        let c = card_for(&r, "c", false).expect("C played");
        assert!(c.facts.len() <= 3 && !c.facts.is_empty());
        assert_eq!(c.facts[0].0, "bad");
        assert!(c.worst.is_some());
        assert!(card_for(&r, "nobody", false).is_none());
    }
}
