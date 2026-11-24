//! Post-game facts from OpenDota (api.opendota.com), which parses the match
//! replay. It covers all ten players, including teammates who don't run the
//! app and the enemy team, and turns several of our estimates into facts:
//! real ward placements and how long they lived, who killed whom, how often
//! an item like BKB was actually pressed, and who was in each team fight.
//!
//! Every field is optional: unparsed matches, older replays and API changes
//! must never break a report.

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

const API: &str = "https://api.opendota.com/api";

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Match {
    pub match_id: u64,
    pub duration: i32,
    pub radiant_win: Option<bool>,
    /// Non-null once the replay has been parsed.
    pub version: Option<u32>,
    pub players: Vec<Player>,
    pub teamfights: Option<Vec<Teamfight>>,
}

impl Match {
    pub fn parsed(&self) -> bool {
        self.version.is_some()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Player {
    pub player_slot: u32,
    pub hero_id: u32,
    pub personaname: Option<String>,
    pub kills: u32,
    pub deaths: u32,
    pub assists: u32,
    pub obs_log: Option<Vec<WardEvent>>,
    pub obs_left_log: Option<Vec<WardEvent>>,
    pub sen_log: Option<Vec<WardEvent>>,
    pub kills_log: Option<Vec<KillEvent>>,
    pub killed_by: Option<HashMap<String, u32>>,
    pub item_uses: Option<HashMap<String, u32>>,
    pub purchase_log: Option<Vec<Purchase>>,
    pub buyback_log: Option<Vec<serde_json::Value>>,
    pub stuns: Option<f32>,
    pub hero_healing: Option<u32>,
    pub teamfight_participation: Option<f32>,
}

impl Player {
    pub fn radiant(&self) -> bool {
        self.player_slot < 128
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct WardEvent {
    pub time: i32,
    pub ehandle: Option<u64>,
    pub x: Option<f32>,
    pub y: Option<f32>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct KillEvent {
    pub time: i32,
    /// Victim, e.g. "npc_dota_hero_lina".
    pub key: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Purchase {
    pub time: i32,
    pub key: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Teamfight {
    pub start: i32,
    pub end: i32,
    pub deaths: u32,
    /// Same order as `Match::players`.
    pub players: Vec<FightPlayer>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct FightPlayer {
    pub deaths: u32,
    pub damage: u64,
    pub healing: u64,
    pub ability_uses: Option<HashMap<String, u32>>,
    pub item_uses: Option<HashMap<String, u32>>,
}

impl FightPlayer {
    /// Took part: hit someone, healed, cast something or died there.
    pub fn involved(&self) -> bool {
        self.damage > 0
            || self.healing > 0
            || self.deaths > 0
            || self.ability_uses.as_ref().is_some_and(|m| !m.is_empty())
            || self.item_uses.as_ref().is_some_and(|m| !m.is_empty())
    }
}

#[derive(Deserialize, Debug, Clone)]
struct Hero {
    id: u32,
    name: String,
}

/// What the report keeps from OpenDota (small, serialisable).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OdStatus {
    /// "waiting", "parsing", "done", "unavailable" or "off".
    pub state: String,
    pub note: String,
    pub tries: u32,
}

fn client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(25))
        .user_agent("cyberspyde-team-link")
        .build()
        .map_err(|e| e.to_string())
}

/// hero id -> short internal name ("juggernaut").
pub async fn heroes() -> Result<HashMap<u32, String>, String> {
    let list: Vec<Hero> = client()?
        .get(format!("{API}/heroes"))
        .send()
        .await
        .map_err(|e| format!("heroes: {e}"))?
        .error_for_status()
        .map_err(|e| format!("heroes: {e}"))?
        .json()
        .await
        .map_err(|e| format!("heroes: {e}"))?;
    Ok(list.into_iter().map(|h| (h.id, protocol::hero_short(&h.name).to_string())).collect())
}

pub async fn fetch(match_id: &str) -> Result<Match, String> {
    let res = client()?.get(format!("{API}/matches/{match_id}")).send().await.map_err(|e| format!("match: {e}"))?;
    if res.status().as_u16() == 404 {
        return Err("OpenDota does not know this match yet".into());
    }
    let res = res.error_for_status().map_err(|e| format!("match: {e}"))?;
    res.json().await.map_err(|e| format!("match: {e}"))
}

/// Asks OpenDota to download and parse the replay.
pub async fn request_parse(match_id: &str) -> Result<(), String> {
    client()?
        .post(format!("{API}/request/{match_id}"))
        .send()
        .await
        .map_err(|e| format!("request: {e}"))?
        .error_for_status()
        .map_err(|e| format!("request: {e}"))?;
    Ok(())
}

/// Minutes after the game to try again: replays take a few minutes to
/// appear, and parsing can queue.
pub const SCHEDULE_MIN: [u64; 7] = [2, 5, 10, 20, 40, 90, 180];

#[cfg(test)]
pub mod tests {
    use super::*;

    /// A trimmed real-shaped OpenDota match: radiant = slots 0-4.
    pub const SAMPLE: &str = r#"{
      "match_id": 8800000001, "duration": 2400, "radiant_win": true, "version": 21,
      "players": [
        {"player_slot": 0, "hero_id": 8, "personaname": "carry guy", "kills": 9, "deaths": 2, "assists": 7,
         "item_uses": {"black_king_bar": 3, "tpscroll": 9},
         "purchase_log": [{"time": 1500, "key": "black_king_bar"}],
         "killed_by": {"npc_dota_hero_axe": 2}, "kills_log": [{"time": 700, "key": "npc_dota_hero_axe"}],
         "obs_log": [], "obs_left_log": [], "stuns": 3.5},
        {"player_slot": 1, "hero_id": 31, "kills": 1, "deaths": 6, "assists": 20,
         "obs_log": [{"time": 400, "ehandle": 11, "x": 120, "y": 130}, {"time": 900, "ehandle": 12, "x": 100, "y": 100}],
         "obs_left_log": [{"time": 760, "ehandle": 11}, {"time": 930, "ehandle": 12}],
         "item_uses": {"glimmer_cape": 0}, "purchase_log": [{"time": 800, "key": "glimmer_cape"}],
         "killed_by": {"npc_dota_hero_axe": 4, "npc_dota_hero_lion": 2}},
        {"player_slot": 2, "hero_id": 41, "kills": 4, "deaths": 5, "assists": 3},
        {"player_slot": 3, "hero_id": 25, "kills": 6, "deaths": 3, "assists": 9},
        {"player_slot": 4, "hero_id": 99, "personaname": "random fifth", "kills": 2, "deaths": 4, "assists": 11},
        {"player_slot": 128, "hero_id": 2, "kills": 7, "deaths": 6, "assists": 4},
        {"player_slot": 129, "hero_id": 26, "kills": 3, "deaths": 8, "assists": 8},
        {"player_slot": 130, "hero_id": 44, "kills": 5, "deaths": 4, "assists": 2},
        {"player_slot": 131, "hero_id": 5, "kills": 1, "deaths": 5, "assists": 10},
        {"player_slot": 132, "hero_id": 9, "kills": 2, "deaths": 7, "assists": 5}
      ],
      "teamfights": [
        {"start": 600, "end": 640, "deaths": 3, "players": [
          {"deaths": 0, "damage": 900}, {"deaths": 0, "damage": 0, "healing": 300}, {"deaths": 0, "damage": 0},
          {"deaths": 0, "damage": 400}, {"deaths": 0, "damage": 0},
          {"deaths": 1, "damage": 200}, {"deaths": 1, "damage": 100}, {"deaths": 1, "damage": 50}, {"deaths": 0}, {"deaths": 0}]},
        {"start": 1600, "end": 1650, "deaths": 2, "players": [
          {"deaths": 0, "damage": 0}, {"deaths": 1, "damage": 100}, {"deaths": 1, "damage": 300},
          {"deaths": 0, "damage": 0}, {"deaths": 0, "damage": 0},
          {"deaths": 0, "damage": 800}, {"deaths": 0, "damage": 600}, {"deaths": 0}, {"deaths": 0}, {"deaths": 0}]}
      ]
    }"#;

    #[test]
    fn parses_lenient_match() {
        let m: Match = serde_json::from_str(SAMPLE).unwrap();
        assert!(m.parsed());
        assert_eq!(m.players.len(), 10);
        assert!(m.players[0].radiant() && !m.players[5].radiant());
        assert_eq!(m.teamfights.as_ref().unwrap().len(), 2);
        assert!(m.teamfights.as_ref().unwrap()[0].players[1].involved(), "healing counts");
        assert!(!m.teamfights.as_ref().unwrap()[0].players[2].involved());
        // An unparsed match is mostly empty, and still reads.
        let bare: Match = serde_json::from_str(r#"{"match_id": 1, "players": [{"hero_id": 8}], "version": null}"#).unwrap();
        assert!(!bare.parsed());
    }
}
