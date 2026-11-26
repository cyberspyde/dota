//! Plain-language facts about how each player helped the team, the kind
//! Dota's own end screen doesn't show. No scores and no grades: each fact is
//! one sentence a friend understands at a glance, coloured good or bad,
//! with where it came from and how it compares with that player's usual.
//!
//! Sources, most trusted first:
//!   replay   - OpenDota's parse of the real replay (all ten players)
//!   measured - our own per-second recording (players running the app)
//!   estimate - inferred from the recording (marked as such on the site)

use crate::{
    analysis::{clock, dur, Fight, MatchReport, Metrics, Moment, PlayerReport},
    opendota,
};
use protocol::{hero_display, hero_short, is_key_item, item_short};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Card {
    /// Stable id, used to compare with the player's earlier games.
    pub id: String,
    /// One short sentence: "Showed up to 3 of 7 team fights".
    pub text: String,
    /// "good", "bad" or "neutral".
    pub tone: String,
    /// "replay", "measured" or "estimate".
    pub source: String,
    /// The number behind the sentence, for trends.
    pub num: f32,
    /// Higher is better for this fact.
    pub up_good: bool,
    /// "usually 4.0" etc., once there is history.
    #[serde(default)]
    pub usual: Option<String>,
    /// "better", "worse" or "same" than usual.
    #[serde(default)]
    pub trend: Option<String>,
}

/// What OpenDota's replay parse told us about one of our players.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OdFacts {
    pub fights_in: u32,
    pub fights_total: u32,
    /// Deaths outside every team fight: picked off.
    pub pickoffs: u32,
    pub obs_placed: u32,
    /// Average observer lifetime, seconds.
    pub obs_life_s: i32,
    pub sen_placed: u32,
    /// (item, bought at, times pressed, minutes owned)
    pub actives: Vec<(String, i32, u32, i32)>,
    pub stuns: f32,
    /// Enemy hero that killed this player most, and how often.
    pub nemesis: Option<(String, u32)>,
    pub buybacks: u32,
}

fn card(id: &str, text: String, tone: &str, source: &str, num: f32, up_good: bool) -> Card {
    Card { id: id.into(), text, tone: tone.into(), source: source.into(), num, up_good, usual: None, trend: None }
}

fn plural(n: u32, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

/// Facts for one player, from whatever sources this report has.
/// `app_mates` = how many *other* players ran the app this game. Facts that
/// measure distance to teammates, fights from positions or help calls need
/// at least one; with none they would always read "0" and mislead.
pub fn cards(p: &PlayerReport, fights_total: u32, app_mates: usize) -> Vec<Card> {
    let m: &Metrics = &p.metrics;
    let od = p.od.as_ref();
    let mut out = Vec::new();
    let team_aware = p.app && app_mates > 0;

    // 1. Showing up to fights.
    let (inn, tot, src) = match od {
        Some(o) if o.fights_total > 0 => (o.fights_in, o.fights_total, "replay"),
        _ if team_aware && fights_total > 0 => (m.fights_present, m.fights_total.max(m.fights_present), "measured"),
        _ => (0, 0, ""),
    };
    if tot > 0 {
        let pct = inn as f32 / tot as f32 * 100.0;
        let tone = if pct >= 70.0 { "good" } else if pct < 50.0 { "bad" } else { "neutral" };
        out.push(card("fights", format!("Showed up to {inn} of {tot} team fights"), tone, src, pct, true));
    }

    // 2. Dying where nobody could help.
    if let Some(o) = od {
        let tone = match o.pickoffs { 0 => "good", 1 => "neutral", _ => "bad" };
        out.push(card("pickoffs", format!("Died outside the big team fights {}", plural(o.pickoffs, "time", "times")), tone, "replay", o.pickoffs as f32, false));
    }
    if team_aware {
        let tone = match m.deaths_alone { 0 => "good", 1 => "neutral", _ => "bad" };
        out.push(card("alone_deaths", format!("Died with no teammate nearby {}", plural(m.deaths_alone, "time", "times")), tone, "measured", m.deaths_alone as f32, false));
    }

    // 3. Coming when called.
    if team_aware && m.calls_total > 0 {
        let tone = if m.calls_answered == m.calls_total { "good" } else if m.calls_answered * 2 < m.calls_total { "bad" } else { "neutral" };
        let speed = if m.calls_answered > 0 { format!(", in {:.0}s on average", m.avg_answer_s) } else { String::new() };
        out.push(card("calls", format!("Came to {} of {} help calls{speed}", m.calls_answered, m.calls_total), tone, "measured", m.calls_answered as f32 / m.calls_total as f32 * 100.0, true));
    }

    // 4. Staying out of fights while able to TP in.
    if team_aware && fights_total > 0 {
        let n = m.fights_missed_tp_ready;
        let tone = if n == 0 { "good" } else if n >= 2 { "bad" } else { "neutral" };
        out.push(card("skipped_tp", format!("Stayed out of {} with TP ready", plural(n, "fight", "fights")), tone, "measured", n as f32, false));
    }

    // 5. Items bought to be pressed, and how often they were.
    if let Some(o) = od {
        // The least-used active item owned for at least 5 minutes.
        if let Some((item, bought, uses, mins)) = o.actives.iter().filter(|a| a.3 >= 5).min_by(|a, b| (a.2 as f32 / a.3 as f32).total_cmp(&(b.2 as f32 / b.3 as f32))) {
            let tone = if *uses == 0 { "bad" } else if (*uses as f32) < (*mins as f32 / 8.0) { "neutral" } else { "good" };
            out.push(card(
                "item_uses",
                format!("Pressed {} {} in {} min of owning it (bought {})", item_short(item), plural(*uses, "time", "times"), mins, clock(*bought)),
                tone,
                "replay",
                *uses as f32 / (*mins).max(1) as f32 * 10.0,
                true,
            ));
        }
    }

    // 6. Vision.
    match od {
        Some(o) if o.obs_placed > 0 => {
            let life = if o.obs_life_s > 0 { format!("; they lived {} on average", dur(o.obs_life_s)) } else { String::new() };
            let tone = if o.obs_life_s > 0 && o.obs_life_s < 120 { "bad" } else { "neutral" };
            out.push(card("wards", format!("Placed {}{life}", plural(o.obs_placed, "observer", "observers")), tone, "replay", o.obs_placed as f32, true));
        }
        _ if p.app && m.obs_placed > 0 => {
            out.push(card("wards", format!("Placed about {}", plural(m.obs_placed, "observer", "observers")), "neutral", "estimate", m.obs_placed as f32, true));
        }
        _ => {}
    }
    if p.app && m.ward_hold_s >= 180 {
        out.push(card("ward_hold", format!("Carried observers for {} before placing", dur(m.ward_hold_s)), "bad", "measured", m.ward_hold_s as f32 / 60.0, false));
    }

    // 7. Gold doing nothing.
    if p.app {
        let mins = m.hoard_s / 60;
        let tone = if mins >= 4 { "bad" } else if mins == 0 { "good" } else { "neutral" };
        out.push(card("lazy_gold", format!("Sat on 2,000+ unspent gold for {mins} min"), tone, "measured", mins as f32, false));
        if m.gold_lost > 0 {
            out.push(card("gold_lost", format!("Lost {} gold by dying", m.gold_lost), if m.gold_lost > 2000 { "bad" } else { "neutral" }, "measured", m.gold_lost as f32, false));
        }
    }

    // 8. Fights: first to fall, retreat calls.
    if team_aware && m.first_to_die > 0 {
        out.push(card("first_death", format!("First to die in {}", plural(m.first_to_die, "fight", "fights")), if m.first_to_die >= 2 { "bad" } else { "neutral" }, "measured", m.first_to_die as f32, false));
    }
    if p.app && m.deaths_after_retreat_call > 0 {
        out.push(card("ignored_retreat", format!("Died right after a RETREAT call {}", plural(m.deaths_after_retreat_call, "time", "times")), "bad", "measured", m.deaths_after_retreat_call as f32, false));
    }

    // 9. Distance from the team.
    if team_aware && m.avg_team_distance > 0.0 {
        let tone = if m.alone_pct > 45.0 { "bad" } else if m.alone_pct < 20.0 { "good" } else { "neutral" };
        out.push(card("far", format!("Far from every teammate {:.0}% of the time", m.alone_pct), tone, "measured", m.alone_pct, false));
    }

    // 10. Who keeps killing you.
    if let Some((hero, n)) = od.and_then(|o| o.nemesis.clone()) {
        if n >= 3 {
            out.push(card("nemesis", format!("{hero} killed you {n} times"), "neutral", "replay", n as f32, false));
        }
    }
    out
}

fn short_hero(id: u32, heroes: &HashMap<u32, String>) -> String {
    heroes.get(&id).cloned().unwrap_or_default()
}

/// Merges OpenDota's parse into a report: real team fights, facts for our
/// players (including teammates without the app), the enemy line-up.
pub fn apply_opendota(r: &mut MatchReport, m: &opendota::Match, heroes: &HashMap<u32, String>) {
    let radiant = match r.team.as_str() {
        "radiant" => true,
        "dire" => false,
        // No side recorded: pick the side our heroes are on.
        _ => m.players.iter().filter(|p| r.players.iter().any(|q| q.hero == short_hero(p.hero_id, heroes))).all(|p| p.radiant()),
    };
    let ours: Vec<usize> = (0..m.players.len()).filter(|&i| m.players[i].radiant() == radiant).collect();
    let theirs: Vec<usize> = (0..m.players.len()).filter(|&i| m.players[i].radiant() != radiant).collect();
    r.enemies = theirs.iter().map(|&i| hero_display(&short_hero(m.players[i].hero_id, heroes))).collect();
    if r.won.is_none() {
        r.won = m.radiant_win.map(|w| w == radiant);
    }

    let fights = m.teamfights.clone().unwrap_or_default();
    let window: Vec<(i32, i32)> = fights.iter().map(|f| (f.start - 5, f.end + 5)).collect();

    // Who killed us: summed from each of our players' "killed by" list.
    let mut killers: HashMap<String, u32> = HashMap::new();
    for &o in &ours {
        for (killer, n) in m.players[o].killed_by.clone().unwrap_or_default() {
            // Only enemy heroes; creeps, towers and neutrals are not a nemesis.
            if killer.starts_with("npc_dota_hero_") {
                *killers.entry(hero_display(hero_short(&killer))).or_default() += n;
            }
        }
    }
    let mut killers: Vec<(String, u32)> = killers.into_iter().collect();
    killers.sort_by(|a, b| b.1.cmp(&a.1));
    r.killers = killers;

    for &i in &ours {
        let op = &m.players[i];
        let hero = short_hero(op.hero_id, heroes);
        let mut f = OdFacts { fights_total: fights.len() as u32, ..Default::default() };
        f.fights_in = fights.iter().filter(|t| t.players.get(i).is_some_and(|fp| fp.involved())).count() as u32;
        // Our deaths, timed from the enemies' kill logs.
        let deaths: Vec<i32> = theirs
            .iter()
            .flat_map(|&e| m.players[e].kills_log.clone().unwrap_or_default())
            .filter(|k| hero_short(&k.key) == hero)
            .map(|k| k.time)
            .collect();
        f.pickoffs = deaths.iter().filter(|&&t| !window.iter().any(|(a, b)| t >= *a && t <= *b)).count() as u32;
        let obs = op.obs_log.clone().unwrap_or_default();
        f.obs_placed = obs.len() as u32;
        let left = op.obs_left_log.clone().unwrap_or_default();
        let lives: Vec<i32> = obs
            .iter()
            .map(|w| {
                left.iter()
                    .find(|l| l.ehandle.is_some() && l.ehandle == w.ehandle)
                    .map(|l| (l.time - w.time).clamp(0, 360))
                    .unwrap_or(360)
            })
            .collect();
        f.obs_life_s = if lives.is_empty() { 0 } else { lives.iter().sum::<i32>() / lives.len() as i32 };
        f.sen_placed = op.sen_log.as_ref().map(|l| l.len() as u32).unwrap_or(0);
        let uses = op.item_uses.clone().unwrap_or_default();
        for pch in op.purchase_log.clone().unwrap_or_default() {
            let item = pch.key.trim_start_matches("item_").to_string();
            // Items you press, not passives or consumables.
            if is_key_item(&item) && !matches!(item.as_str(), "aegis" | "cheese" | "gem" | "smoke_of_deceit" | "aeon_disk" | "sphere") && !f.actives.iter().any(|a| a.0 == item) {
                let owned = ((m.duration - pch.time) / 60).max(0);
                f.actives.push((item.clone(), pch.time, *uses.get(&item).unwrap_or(&0), owned));
            }
        }
        f.stuns = op.stuns.unwrap_or(0.0);
        f.nemesis = op
            .killed_by
            .as_ref()
            .and_then(|k| k.iter().max_by_key(|(_, n)| **n).map(|(h, n)| (hero_display(hero_short(h)), *n)));
        f.buybacks = op.buyback_log.as_ref().map(|b| b.len() as u32).unwrap_or(0);

        match r.players.iter_mut().find(|p| p.hero == hero) {
            Some(p) => {
                p.od = Some(f);
                if p.metrics.obs_placed == 0 || p.od.as_ref().is_some_and(|o| o.obs_placed > 0) {
                    p.metrics.obs_placed = p.od.as_ref().map(|o| o.obs_placed).unwrap_or(0);
                }
            }
            // A teammate who didn't run the app: still part of the team report.
            None => r.players.push(PlayerReport {
                name: op.personaname.clone().filter(|n| !n.trim().is_empty()).unwrap_or_else(|| format!("Teammate ({})", hero_display(&hero))),
                hero: hero.clone(),
                hero_name: hero_display(&hero),
                team: if radiant { "radiant".into() } else { "dire".into() },
                k: op.kills,
                d: op.deaths,
                a: op.assists,
                lh: 0,
                gpm: 0,
                xpm: 0,
                level: 0,
                items: Vec::new(),
                title: String::new(),
                metrics: Metrics { deaths: op.deaths, ..Default::default() },
                worst: Vec::new(),
                best: Vec::new(),
                app: false,
                cards: Vec::new(),
                od: Some(f),
            }),
        }
    }

    // Real team fights replace ours: they include everyone and the enemy side.
    if !fights.is_empty() {
        let name_of = |i: usize| -> String {
            let hero = short_hero(m.players[i].hero_id, heroes);
            r.players.iter().find(|p| p.hero == hero).map(|p| p.name.clone()).unwrap_or_else(|| hero_display(&hero))
        };
        let new: Vec<Fight> = fights
            .iter()
            .map(|t| {
                let our_deaths: u32 = ours.iter().filter_map(|&i| t.players.get(i)).map(|p| p.deaths).sum();
                let their_deaths: u32 = theirs.iter().filter_map(|&i| t.players.get(i)).map(|p| p.deaths).sum();
                let result = match (their_deaths, our_deaths) {
                    (k, d) if k > d => "won",
                    (k, d) if d > k => "lost",
                    (0, 0) => "standoff",
                    _ => "traded",
                };
                let present: Vec<String> = ours.iter().filter(|&&i| t.players.get(i).is_some_and(|p| p.involved())).map(|&i| name_of(i)).collect();
                let absent: Vec<String> = ours.iter().filter(|&&i| !t.players.get(i).is_some_and(|p| p.involved())).map(|&i| name_of(i)).collect();
                // Keep our map position for the replay when one of our fights overlaps.
                let (x, y) = r.fights.iter().find(|f| f.start <= t.end && f.end >= t.start).map(|f| (f.x, f.y)).unwrap_or((0.0, 0.0));
                Fight { start: t.start, end: t.end, clock: clock(t.start), x, y, present, absent, our_deaths, our_kills: their_deaths, result: result.into() }
            })
            .collect();
        r.fights = new;
        r.fights_source = "replay".into();
        // Missed fights as moments, for players without the app too.
        for p in r.players.iter_mut().filter(|p| !p.app) {
            for f in r.fights.iter().filter(|f| f.absent.contains(&p.name) && f.result == "lost") {
                p.worst.push(Moment { t: f.start, clock: f.clock.clone(), text: format!("Team lost a fight without you ({} died)", f.our_deaths), weight: 3, kind: "missed_fight".into() });
            }
        }
    }
    let fw = r.fights.iter().filter(|f| f.result == "won").count();
    let fl = r.fights.iter().filter(|f| f.result == "lost").count();
    r.summary = crate::analysis::summary(r.won, r.duration.max(m.duration), r.fights.len(), fw, fl);
    let n = r.fights.len() as u32;
    let apps = r.players.iter().filter(|p| p.app).count();
    for p in &mut r.players {
        let mates = apps.saturating_sub(usize::from(p.app));
        p.cards = cards(p, n, mates);
    }
}

/// Adds "usually X" and better/worse to every fact, from the player's
/// earlier games (newest first, up to ten).
pub fn apply_usual(r: &mut MatchReport, history: &[MatchReport]) {
    for p in &mut r.players {
        let key = p.name.to_lowercase();
        let past: Vec<&PlayerReport> = history
            .iter()
            .filter(|h| h.match_id != r.match_id && h.started_ms < r.started_ms.max(1))
            .filter_map(|h| h.players.iter().find(|q| q.name.to_lowercase() == key))
            .take(10)
            .collect();
        for c in &mut p.cards {
            let vals: Vec<f32> = past.iter().filter_map(|q| q.cards.iter().find(|k| k.id == c.id)).map(|k| k.num).collect();
            if vals.len() < 2 {
                continue;
            }
            let avg = vals.iter().sum::<f32>() / vals.len() as f32;
            c.usual = Some(match c.id.as_str() {
                "fights" | "calls" | "far" => format!("usually {avg:.0}%"),
                _ => format!("usually {avg:.1}"),
            });
            let margin = (avg.abs() * 0.15).max(0.5);
            c.trend = Some(if (c.num - avg).abs() < margin {
                "same".into()
            } else if (c.num > avg) == c.up_good {
                "better".into()
            } else {
                "worse".into()
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{analysis::analyze, opendota::tests::SAMPLE};

    fn heroes() -> HashMap<u32, String> {
        [(8, "juggernaut"), (31, "lich"), (41, "faceless_void"), (25, "lina"), (99, "bristleback"), (2, "axe"), (26, "lion"), (44, "phantom_assassin"), (5, "crystal_maiden"), (9, "mirana")]
            .into_iter()
            .map(|(i, n)| (i, n.to_string()))
            .collect()
    }

    #[test]
    fn opendota_fills_facts_and_absent_teammates() {
        let mut rec = crate::analysis::tests::scenario();
        // Scenario heroes: A lina, B lich, C juggernaut, all radiant.
        rec.match_id = "8800000001".into();
        let mut r = analyze(&rec);
        let m: opendota::Match = serde_json::from_str(SAMPLE).unwrap();
        apply_opendota(&mut r, &m, &heroes());
        assert_eq!(r.fights_source, "replay");
        assert_eq!(r.fights.len(), 2);
        assert_eq!(r.fights[0].result, "won");
        assert_eq!(r.fights[1].result, "lost");
        assert_eq!(r.enemies.len(), 5);
        assert_eq!(r.killers.first().map(|k| k.0.as_str()), Some("Axe"));
        // Faceless Void and Bristleback didn't run the app but are in the report.
        assert_eq!(r.players.len(), 5);
        let random = r.players.iter().find(|p| p.hero == "bristleback").unwrap();
        assert_eq!(random.name, "random fifth");
        assert!(!random.app);
        let lich = r.players.iter().find(|p| p.hero == "lich").unwrap();
        let od = lich.od.as_ref().unwrap();
        assert_eq!(od.obs_placed, 2);
        assert_eq!(od.obs_life_s, (360 + 30) / 2);
        assert!(lich.cards.iter().any(|c| c.id == "item_uses" && c.tone == "bad" && c.text.contains("Glimmer")), "{:?}", lich.cards);
        let jug = r.players.iter().find(|p| p.hero == "juggernaut").unwrap();
        assert!(jug.cards.iter().any(|c| c.id == "fights" && c.source == "replay" && c.text.contains("1 of 2")), "{:?}", jug.cards);
    }

    #[test]
    fn solo_app_player_gets_no_teammate_facts() {
        // Only one player ran the app: distances to teammates are unknown.
        let mut rec = crate::analysis::tests::scenario();
        rec.tracks.retain(|k, _| k == "C");
        let r = analyze(&rec);
        let c = &r.players[0];
        for id in ["alone_deaths", "skipped_tp", "calls", "far", "first_death", "fights"] {
            assert!(!c.cards.iter().any(|k| k.id == id), "{id} must not appear for a solo app player: {:?}", c.cards);
        }
        // Facts about the player alone still do.
        assert!(c.cards.iter().any(|k| k.id == "lazy_gold"), "{:?}", c.cards);
    }

    #[test]
    fn usual_marks_better_and_worse() {
        let mut old = analyze(&crate::analysis::tests::scenario());
        old.started_ms = 1;
        let mut older = old.clone();
        older.match_id = "0".into();
        let mut now = old.clone();
        now.match_id = "2".into();
        now.started_ms = 10;
        let c = now.players.iter_mut().find(|p| p.name == "C").unwrap();
        c.cards.iter_mut().find(|k| k.id == "skipped_tp").unwrap().num = 0.0;
        apply_usual(&mut now, &[old, older]);
        let k = now.players.iter().find(|p| p.name == "C").unwrap().cards.iter().find(|k| k.id == "skipped_tp").unwrap().clone();
        assert_eq!(k.trend.as_deref(), Some("better"), "{k:?}");
        assert_eq!(k.usual.as_deref(), Some("usually 1.0"));
    }
}
