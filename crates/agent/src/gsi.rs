//! Dota 2 Game State Integration: Valve's official feed. Dota POSTs JSON about
//! the *local* player to http://127.0.0.1:<port>/ whenever something changes.
//! It carries only what that player can already see about themself.

use protocol::{hero_short, Ability, Item, PlayerState};
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub enum Parsed {
    /// Wrong or missing auth token: not from our Dota config.
    Unauthorized,
    /// Spectating or casting: we deliberately ignore observer feeds.
    Spectator,
    State(PlayerState),
}

fn u(v: &Value) -> u32 {
    v.as_f64().map(|f| f.max(0.0) as u32).unwrap_or(0)
}
fn b(v: &Value) -> bool {
    v.as_bool().unwrap_or(false)
}
fn s(v: &Value) -> String {
    v.as_str().unwrap_or("").to_string()
}

fn item(v: &Value) -> Option<Item> {
    let name = v.get("name")?.as_str()?;
    if name == "empty" || name.is_empty() {
        return None;
    }
    Some(Item {
        name: name.strip_prefix("item_").unwrap_or(name).to_string(),
        charges: v.get("charges").and_then(|c| c.as_f64()).map(|c| c as u32),
        cooldown: u(&v["cooldown"]),
    })
}

pub fn parse(v: &Value, token: &str) -> Parsed {
    if v["auth"]["token"].as_str() != Some(token) {
        return Parsed::Unauthorized;
    }
    let player = &v["player"];
    // Spectator payloads nest players under team2/team3.
    if player.get("team2").is_some() || player.get("team3").is_some() {
        return Parsed::Spectator;
    }
    let hero = &v["hero"];
    let map = &v["map"];
    let mut st = PlayerState {
        hero: hero["name"].as_str().map(|n| hero_short(n).to_string()).unwrap_or_default(),
        level: u(&hero["level"]),
        alive: b(&hero["alive"]),
        respawn_seconds: u(&hero["respawn_seconds"]),
        health: u(&hero["health"]),
        max_health: u(&hero["max_health"]),
        mana: u(&hero["mana"]),
        max_mana: u(&hero["max_mana"]),
        buyback_cost: u(&hero["buyback_cost"]),
        buyback_cooldown: u(&hero["buyback_cooldown"]),
        smoked: b(&hero["smoked"]),
        stunned: b(&hero["stunned"]),
        silenced: b(&hero["silenced"]),
        hexed: b(&hero["hexed"]),
        magic_immune: b(&hero["magicimmune"]),
        gold: u(&player["gold"]),
        kills: u(&player["kills"]),
        deaths: u(&player["deaths"]),
        assists: u(&player["assists"]),
        last_hits: u(&player["last_hits"]),
        denies: u(&player["denies"]),
        gpm: u(&player["gpm"]),
        xpm: u(&player["xpm"]),
        team: s(&player["team_name"]),
        clock: map["clock_time"].as_f64().unwrap_or(0.0) as i32,
        game_state: s(&map["game_state"]),
        daytime: b(&map["daytime"]),
        match_id: match &map["matchid"] {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            _ => String::new(),
        },
        win_team: match s(&map["win_team"]).as_str() {
            "none" | "" => String::new(),
            w => w.to_string(),
        },
        gold_reliable: u(&player["gold_reliable"]),
        gold_unreliable: u(&player["gold_unreliable"]),
        kill_streak: u(&player["kill_streak"]),
        ..Default::default()
    };
    if let (Some(x), Some(y)) = (hero["xpos"].as_f64(), hero["ypos"].as_f64()) {
        st.has_pos = true;
        st.x = x as f32;
        st.y = y as f32;
    }

    let items = &v["items"];
    for i in 0..6 {
        if let Some(it) = item(&items[format!("slot{i}")]) {
            st.items.push(it);
        }
    }
    for i in 6..9 {
        if let Some(it) = item(&items[format!("slot{i}")]) {
            st.backpack.push(it);
        }
    }
    for i in 0..6 {
        if let Some(it) = item(&items[format!("stash{i}")]) {
            st.stash.push(it);
        }
    }
    st.tp = item(&items["teleport0"]);
    st.neutral = item(&items["neutral0"]);

    if let Some(obj) = v["abilities"].as_object() {
        let mut list: Vec<(u32, Ability)> = obj
            .iter()
            .filter_map(|(k, a)| {
                let idx: u32 = k.strip_prefix("ability")?.parse().ok()?;
                let name = a.get("name")?.as_str()?;
                if name.starts_with("special_bonus") || name.contains("generic_hidden") || name.is_empty() {
                    return None;
                }
                Some((
                    idx,
                    Ability {
                        name: name.to_string(),
                        level: u(&a["level"]),
                        cooldown: u(&a["cooldown"]),
                        ultimate: b(&a["ultimate"]),
                        can_cast: b(&a["can_cast"]),
                        passive: b(&a["passive"]),
                    },
                ))
            })
            .collect();
        list.sort_by_key(|(i, _)| *i);
        st.abilities = list.into_iter().map(|(_, a)| a).collect();
    }
    Parsed::State(st)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
      "provider": {"name": "Dota 2", "appid": 570, "version": 47, "timestamp": 1700000000},
      "map": {"name": "start", "matchid": "1", "game_time": 905, "clock_time": 845, "daytime": true,
              "game_state": "DOTA_GAMERULES_STATE_GAME_IN_PROGRESS", "paused": false},
      "player": {"steamid": "7656", "name": "ilhom", "activity": "playing", "kills": 4, "deaths": 1,
                 "assists": 7, "last_hits": 88, "denies": 9, "team_name": "radiant", "gold": 1450,
                 "gpm": 512, "xpm": 600},
      "hero": {"xpos": -1200, "ypos": 640, "id": 8, "name": "npc_dota_hero_juggernaut", "level": 11,
               "alive": true, "respawn_seconds": 0, "buyback_cost": 900, "buyback_cooldown": 0,
               "health": 640, "max_health": 1400, "mana": 210, "max_mana": 500,
               "silenced": false, "stunned": false, "magicimmune": false, "hexed": false, "smoked": false},
      "abilities": {
        "ability0": {"name": "juggernaut_blade_fury", "level": 4, "can_cast": true, "passive": false, "cooldown": 0, "ultimate": false},
        "ability1": {"name": "juggernaut_healing_ward", "level": 1, "can_cast": true, "passive": false, "cooldown": 0, "ultimate": false},
        "ability2": {"name": "juggernaut_blade_dance", "level": 4, "can_cast": false, "passive": true, "cooldown": 0, "ultimate": false},
        "ability3": {"name": "juggernaut_omni_slash", "level": 1, "can_cast": false, "passive": false, "cooldown": 42, "ultimate": true},
        "ability4": {"name": "special_bonus_unique_juggernaut_3", "level": 0}
      },
      "items": {
        "slot0": {"name": "item_phase_boots", "cooldown": 0},
        "slot1": {"name": "item_bfury", "cooldown": 0},
        "slot2": {"name": "item_magic_wand", "charges": 12, "cooldown": 0},
        "slot3": {"name": "empty"}, "slot4": {"name": "empty"}, "slot5": {"name": "empty"},
        "slot6": {"name": "item_quelling_blade"}, "slot7": {"name": "empty"}, "slot8": {"name": "empty"},
        "teleport0": {"name": "item_tpscroll", "charges": 1, "cooldown": 23},
        "neutral0": {"name": "item_broom_handle"}
      },
      "auth": {"token": "secret"}
    }"#;

    #[test]
    fn parses_player_feed() {
        let v: Value = serde_json::from_str(SAMPLE).unwrap();
        let Parsed::State(st) = parse(&v, "secret") else { panic!("expected state") };
        assert_eq!(st.hero, "juggernaut");
        assert_eq!(st.level, 11);
        assert_eq!((st.health, st.max_health), (640, 1400));
        assert!(st.has_pos && st.x == -1200.0);
        assert_eq!(st.items.iter().map(|i| i.name.as_str()).collect::<Vec<_>>(), ["phase_boots", "bfury", "magic_wand"]);
        assert_eq!(st.items[2].charges, Some(12));
        assert_eq!(st.backpack[0].name, "quelling_blade");
        assert_eq!(st.tp_cooldown(), Some(23));
        assert_eq!(st.abilities.len(), 4, "talent entries are filtered");
        assert_eq!(st.ultimate().unwrap().cooldown, 42);
        assert_eq!(st.clock, 845);
        assert_eq!(st.match_id, "1");
        assert_eq!(st.win_team, "");
    }

    #[test]
    fn rejects_bad_token_and_spectator() {
        let v: Value = serde_json::from_str(SAMPLE).unwrap();
        assert_eq!(parse(&v, "other"), Parsed::Unauthorized);
        let spec: Value = serde_json::from_str(r#"{"auth":{"token":"t"},"player":{"team2":{}}}"#).unwrap();
        assert_eq!(parse(&spec, "t"), Parsed::Spectator);
    }

    #[test]
    fn menu_payload_is_not_in_match() {
        let v: Value = serde_json::from_str(r#"{"auth":{"token":"t"},"provider":{"name":"Dota 2"}}"#).unwrap();
        let Parsed::State(st) = parse(&v, "t") else { panic!() };
        assert!(!st.in_match());
    }
}
