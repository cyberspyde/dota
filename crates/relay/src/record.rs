//! Records every match the team plays, one compact sample per player per game
//! second, so the post-game report can replay who was where when it mattered.
//!
//! Time axis is Dota's own clock (`clock_time`), not wall time: pauses do not
//! stretch the timeline, and a simulated match can run faster than real time.

use protocol::{is_key_item, CalloutKind, PlayerState, GAME_IN_PROGRESS, GAME_POST_GAME};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// One player at one game second.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Sample {
    /// Game clock, seconds.
    pub t: i32,
    pub alive: bool,
    /// HP fraction, 0..1.
    pub hp: f32,
    pub x: f32,
    pub y: f32,
    pub lvl: u32,
    pub k: u32,
    pub d: u32,
    pub a: u32,
    pub lh: u32,
    pub gold: u32,
    pub unrel: u32,
    pub gpm: u32,
    pub xpm: u32,
    pub bb_cost: u32,
    pub bb_cd: u32,
    /// TP scroll cooldown; -1 = no TP.
    pub tp: i32,
    /// Ultimate cooldown; -1 = not learned.
    pub ult: i32,
    pub obs: u32,
    pub sen: u32,
    pub smoked: bool,
    /// Fight-changing items that were off cooldown.
    pub ready: Vec<String>,
}

impl Sample {
    pub fn from_state(st: &PlayerState) -> Self {
        let (obs, sen) = st.ward_count();
        Sample {
            t: st.clock,
            alive: st.alive,
            hp: st.hp_frac(),
            x: st.x,
            y: st.y,
            lvl: st.level,
            k: st.kills,
            d: st.deaths,
            a: st.assists,
            lh: st.last_hits,
            gold: st.gold,
            unrel: st.gold_unreliable,
            gpm: st.gpm,
            xpm: st.xpm,
            bb_cost: st.buyback_cost,
            bb_cd: st.buyback_cooldown,
            tp: st.tp_cooldown().map(|c| c as i32).unwrap_or(-1),
            ult: st.ultimate().filter(|u| u.level > 0).map(|u| u.cooldown as i32).unwrap_or(-1),
            obs,
            sen,
            smoked: st.smoked,
            ready: st
                .items
                .iter()
                .filter(|i| is_key_item(&i.name) && i.cooldown == 0 && !i.name.starts_with("smoke") && i.name != "aegis")
                .map(|i| i.name.clone())
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Track {
    pub name: String,
    pub hero: String,
    pub team: String,
    pub samples: Vec<Sample>,
    /// Final items, for the report card.
    pub items: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CalloutEvent {
    pub t: i32,
    pub from: String,
    pub kind: CalloutKind,
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Recording {
    pub match_id: String,
    pub started_ms: u64,
    pub last_ms: u64,
    pub win_team: String,
    /// When the post-game state was first seen (wall ms), 0 = not yet.
    pub ended_ms: u64,
    pub tracks: BTreeMap<String, Track>,
    pub callouts: Vec<CalloutEvent>,
}

impl Recording {
    pub fn duration(&self) -> i32 {
        self.tracks.values().filter_map(|t| t.samples.last()).map(|s| s.t).max().unwrap_or(0)
    }
}

/// Matches currently being played, keyed by Dota match id.
#[derive(Default)]
pub struct Recorder {
    live: BTreeMap<String, Recording>,
    /// player name -> match id they are in, for callouts.
    current: BTreeMap<String, String>,
}

/// A match shorter than this (game clock) is not worth a report.
pub const MIN_DURATION: i32 = 8 * 60;
/// Finish a match this long after post-game, or after this much silence.
const POST_GAME_GRACE_MS: u64 = 45_000;
const SILENCE_MS: u64 = 10 * 60_000;

impl Recorder {
    pub fn on_state(&mut self, name: &str, st: &PlayerState, now_ms: u64) {
        if st.match_id.is_empty() || st.match_id == "0" || !st.in_match() {
            return;
        }
        let post = st.game_state == GAME_POST_GAME;
        if st.game_state != GAME_IN_PROGRESS && !post {
            return;
        }
        let rec = self.live.entry(st.match_id.clone()).or_insert_with(|| Recording {
            match_id: st.match_id.clone(),
            started_ms: now_ms,
            ..Default::default()
        });
        rec.last_ms = now_ms;
        self.current.insert(name.to_string(), st.match_id.clone());
        if post {
            if !st.win_team.is_empty() {
                rec.win_team = st.win_team.clone();
            }
            if rec.ended_ms == 0 {
                rec.ended_ms = now_ms;
            }
        }
        let track = rec.tracks.entry(name.to_string()).or_insert_with(|| Track {
            name: name.to_string(),
            ..Default::default()
        });
        track.hero = st.hero.clone();
        track.team = st.team.clone();
        track.items = st.items.iter().map(|i| i.name.clone()).collect();
        if post {
            return;
        }
        let s = Sample::from_state(st);
        match track.samples.last_mut() {
            // Same game second: keep the newest values, but never lose a death.
            Some(last) if last.t == s.t => {
                let died = last.alive && !s.alive;
                if died || last.alive == s.alive {
                    *last = s;
                } else {
                    track.samples.push(s);
                }
            }
            // Clock went backwards: a rejoin or replay glitch; ignore.
            Some(last) if s.t < last.t => {}
            _ => track.samples.push(s),
        }
    }

    pub fn on_callout(&mut self, name: &str, kind: CalloutKind, x: f32, y: f32) {
        let Some(mid) = self.current.get(name) else { return };
        let Some(rec) = self.live.get_mut(mid) else { return };
        let t = rec.tracks.get(name).and_then(|t| t.samples.last()).map(|s| s.t).unwrap_or(0);
        rec.callouts.push(CalloutEvent { t, from: name.to_string(), kind, x, y });
    }

    /// Removes and returns matches that are over.
    pub fn take_finished(&mut self, now_ms: u64) -> Vec<Recording> {
        let done: Vec<String> = self
            .live
            .iter()
            .filter(|(_, r)| {
                (r.ended_ms > 0 && now_ms.saturating_sub(r.ended_ms) > POST_GAME_GRACE_MS)
                    || now_ms.saturating_sub(r.last_ms) > SILENCE_MS
            })
            .map(|(k, _)| k.clone())
            .collect();
        let out: Vec<Recording> = done.iter().filter_map(|k| self.live.remove(k)).collect();
        self.current.retain(|_, m| self.live.contains_key(m));
        out
    }

    /// Summary of matches in progress, for the site's live view.
    pub fn live_ids(&self) -> Vec<(String, i32, Vec<String>)> {
        self.live
            .values()
            .map(|r| (r.match_id.clone(), r.duration(), r.tracks.keys().cloned().collect()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn st(clock: i32, alive: bool) -> PlayerState {
        PlayerState {
            hero: "lina".into(),
            match_id: "77".into(),
            game_state: GAME_IN_PROGRESS.into(),
            clock,
            alive,
            max_health: 100,
            health: 50,
            ..Default::default()
        }
    }

    #[test]
    fn one_sample_per_second_and_deaths_kept() {
        let mut r = Recorder::default();
        r.on_state("a", &st(10, true), 0);
        r.on_state("a", &st(10, true), 50);
        r.on_state("a", &st(11, true), 100);
        r.on_state("a", &st(11, false), 150);
        r.on_state("a", &st(12, false), 200);
        let rec = &r.live["77"];
        let s = &rec.tracks["a"].samples;
        assert_eq!(s.iter().map(|x| (x.t, x.alive)).collect::<Vec<_>>(), [(10, true), (11, false), (12, false)]);
    }

    #[test]
    fn finishes_after_post_game() {
        let mut r = Recorder::default();
        r.on_state("a", &st(900, true), 0);
        let mut end = st(905, true);
        end.game_state = GAME_POST_GAME.into();
        end.win_team = "radiant".into();
        r.on_state("a", &end, 1_000);
        assert!(r.take_finished(10_000).is_empty());
        let done = r.take_finished(60_000);
        assert_eq!(done.len(), 1);
        assert_eq!(done[0].win_team, "radiant");
    }
}
