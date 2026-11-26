//! Enemy intel from the in-game bridge: it reads the enemy team, wards,
//! Roshan, runes and the courier in-game and POSTs a snapshot here. We parse
//! it into protocol types and hand it to the team so it can be shared with
//! the relay and drawn on the overlay.

use protocol::{EnemyHero, EnemyInfo, EnemyWard, Item, RoshanInfo, Rune};
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub enum Parsed {
    Info(EnemyInfo),
}

fn u(v: &Value) -> u32 {
    v.as_f64().map(|f| f.max(0.0) as u32).unwrap_or(0)
}
fn i(v: &Value) -> i32 {
    v.as_f64().map(|f| f as i32).unwrap_or(0)
}
fn b(v: &Value) -> bool {
    v.as_bool().unwrap_or(false)
}
fn s(v: &Value) -> String {
    v.as_str().unwrap_or("").to_string()
}
fn f(v: &Value) -> f32 {
    v.as_f64().unwrap_or(0.0) as f32
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

fn hero(v: &Value) -> Option<EnemyHero> {
    let name = v.get("hero")?.as_str()?;
    let mut items = Vec::new();
    if let Some(arr) = v.get("items").and_then(|a| a.as_array()) {
        for it in arr {
            if let Some(i) = item(it) {
                items.push(i);
            }
        }
    }
    Some(EnemyHero {
        hero: name.to_string(),
        level: u(&v["level"]),
        alive: b(&v["alive"]),
        health: u(&v["health"]),
        max_health: u(&v["max_health"]),
        x: f(&v["x"]),
        y: f(&v["y"]),
        items,
        ult_cooldown: i(&v["ult_cooldown"]),
        magic_immune: b(&v["magic_immune"]),
        smoked: b(&v["smoked"]),
        stunned: b(&v["stunned"]),
    })
}

fn ward(v: &Value) -> Option<EnemyWard> {
    Some(EnemyWard {
        kind: s(&v["kind"]),
        x: f(&v["x"]),
        y: f(&v["y"]),
        age: u(&v["age"]),
    })
}

fn rune(v: &Value) -> Option<Rune> {
    Some(Rune {
        kind: s(&v["kind"]),
        x: f(&v["x"]),
        y: f(&v["y"]),
        available_in: u(&v["available_in"]),
    })
}

/// The port is localhost-only, so every well-formed snapshot is accepted.
pub fn parse(v: &Value) -> Parsed {
    let mut info = EnemyInfo::default();
    info.clock = i(&v["clock"]);
    info.captured_ms = u(&v["captured_ms"]).max(1) as u64;
    if let Some(arr) = v.get("heroes").and_then(|a| a.as_array()) {
        info.heroes = arr.iter().filter_map(hero).collect();
    }
    if let Some(arr) = v.get("wards").and_then(|a| a.as_array()) {
        info.wards = arr.iter().filter_map(ward).collect();
    }
    if let Some(r) = v.get("roshan") {
        info.roshan = RoshanInfo {
            alive: b(&r["alive"]),
            health: u(&r["health"]),
            max_health: u(&r["max_health"]),
            x: f(&r["x"]),
            y: f(&r["y"]),
            last_death: u(&r["last_death"]),
        };
    }
    if let Some(arr) = v.get("runes").and_then(|a| a.as_array()) {
        info.runes = arr.iter().filter_map(rune).collect();
    }
    if let Some(c) = v.get("courier") {
        info.courier = protocol::Courier {
            x: f(&c["x"]),
            y: f(&c["y"]),
            state: s(&c["state"]),
        };
    }
    Parsed::Info(info)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
      "clock": 610,
      "captured_ms": 1700000000000,
      "heroes": [
        {"hero": "npc_dota_hero_lich", "level": 10, "alive": true, "health": 800, "max_health": 1200,
         "x": 1000, "y": -2000, "items": [{"name": "item_tranquil_boots", "cooldown": 0}, {"name": "empty"}],
         "ult_cooldown": 42, "magic_immune": false, "smoked": false, "stunned": false}
      ],
      "wards": [{"kind": "observer", "x": 500, "y": 600, "age": 30}],
      "roshan": {"alive": true, "health": 6000, "max_health": 6000, "x": 2600, "y": -2600, "last_death": 0},
      "runes": [{"kind": "aegis", "x": 1300, "y": 1300, "available_in": 0}],
      "courier": {"x": 400, "y": 400, "state": "moving"}
    }"#;

    #[test]
    fn parses_snapshot() {
        let v: Value = serde_json::from_str(SAMPLE).unwrap();
        let Parsed::Info(info) = parse(&v) else { panic!() };
        assert_eq!(info.clock, 610);
        assert_eq!(info.heroes.len(), 1);
        assert_eq!(info.heroes[0].hero, "npc_dota_hero_lich");
        assert_eq!(info.heroes[0].items.len(), 1);
        assert_eq!(info.heroes[0].items[0].name, "tranquil_boots");
        assert_eq!(info.heroes[0].ult_cooldown, 42);
        assert_eq!(info.wards.len(), 1);
        assert_eq!(info.wards[0].kind, "observer");
        assert!(info.roshan.alive);
        assert_eq!(info.runes[0].kind, "aegis");
        assert_eq!(info.courier.state, "moving");
    }

    #[test]
    fn empty_snapshot_is_ok() {
        let v: Value = serde_json::from_str(r#"{}"#).unwrap();
        let Parsed::Info(info) = parse(&v) else { panic!() };
        assert!(info.heroes.is_empty());
        assert_eq!(info.clock, 0);
    }
}
