//! Build helper built on the legacy CYBERSPYDE build database
//! (legacy-helper/build_data_final.json, embedded at compile time), plus an
//! optional `builds.override.json` next to the agent where your pro friend can
//! write builds and skill orders once instead of repeating them every game.

use protocol::PlayerState;
use serde::Deserialize;
use std::{collections::HashMap, path::Path};

const LEGACY: &str = include_str!("../../../legacy-helper/build_data_final.json");

#[derive(Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct BuildItem {
    pub id: String,
    pub name: String,
    pub cost: u32,
    pub phase: String,
    pub priority: String,
    pub description: String,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Playstyle {
    pub dos: Vec<String>,
    pub donts: Vec<String>,
    pub tips: Vec<String>,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Gameplan {
    pub early: String,
    pub mid: String,
    pub late: String,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Build {
    pub hero_id: String,
    pub mood: String,
    pub items: Vec<BuildItem>,
    pub playstyle: Playstyle,
    pub gameplan: Gameplan,
    /// Optional skill order, e.g. ["Q","W","Q","E","Q","R",...]. Override file only.
    pub skills: Vec<String>,
    /// Who wrote it, shown in the overlay ("from Pro").
    pub author: String,
}

/// Legacy database ids that differ from Dota's internal names.
fn hero_internal(id: &str) -> &str {
    match id {
        "centaur_warrunner" => "centaur",
        "clockwerk" => "rattletrap",
        "doom" => "doom_bringer",
        "io" => "wisp",
        "lifestealer" => "life_stealer",
        "magnus" => "magnataur",
        "shadow_fiend" => "nevermore",
        "zeus" => "zuus",
        "natures_prophet" => "furion",
        "wraith_king" => "skeleton_king",
        "timbersaw" => "shredder",
        "windranger" => "windrunner",
        "queen_of_pain" => "queenofpain",
        "outworld_destroyer" => "obsidian_destroyer",
        "underlord" => "abyssal_underlord",
        "necrophos" => "necrolyte",
        "vengeful_spirit" => "vengefulspirit",
        "anti_mage" | "anti-mage" => "antimage",
        "treant_protector" => "treant",
        other => other,
    }
}

pub fn item_internal(id: &str) -> &str {
    match id {
        "aghanims_scepter" => "ultimate_scepter",
        "euls_scepter_of_divinity" => "cyclone",
        "eye_of_skadi" => "skadi",
        "scythe_of_vyse" => "sheepstick",
        "divine_rapier" => "rapier",
        "battle_fury" => "bfury",
        "armlet_of_mordiggian" => "armlet",
        "refresher_orb" => "refresher",
        "boots_of_travel" => "travel_boots",
        "skull_basher" => "basher",
        "daedalus" => "greater_crit",
        "linkens_sphere" => "sphere",
        "shadow_blade" => "invis_sword",
        other => other,
    }
}

/// Owning a later item in an upgrade chain counts as owning the earlier one.
fn upgrades_of(item: &str) -> &'static [&'static str] {
    match item {
        "blink" => &["overwhelming_blink", "swift_blink", "arcane_blink"],
        "travel_boots" => &["travel_boots_2"],
        "invis_sword" => &["silver_edge"],
        "dragon_lance" => &["hurricane_pike"],
        "urn_of_shadows" => &["spirit_vessel"],
        "orchid" => &["bloodthorn"],
        "basher" => &["abyssal_blade"],
        "maelstrom" => &["mjollnir", "gungir"],
        "mekansm" => &["guardian_greaves"],
        "yasha" => &["sange_and_yasha", "yasha_and_kaya", "manta"],
        "vanguard" => &["crimson_guard", "abyssal_blade"],
        "helm_of_the_dominator" => &["helm_of_the_overlord"],
        "cyclone" => &["wind_waker"],
        _ => &[],
    }
}

pub struct BuildDb {
    by_hero: HashMap<String, Vec<Build>>,
    pub source_note: String,
}

impl BuildDb {
    pub fn load(override_path: &Path) -> Self {
        let mut by_hero: HashMap<String, Vec<Build>> = HashMap::new();
        let legacy: Vec<Build> = serde_json::from_str(LEGACY).unwrap_or_default();
        let legacy_n = legacy.len();
        for mut b in legacy {
            b.author = "CYBERSPYDE db".into();
            by_hero.entry(hero_internal(&b.hero_id).to_string()).or_default().push(b);
        }
        let mut note = format!("{legacy_n} builds from the legacy database");
        if let Ok(text) = std::fs::read_to_string(override_path) {
            match serde_json::from_str::<Vec<Build>>(&text) {
                Ok(list) => {
                    note.push_str(&format!(", {} from {}", list.len(), override_path.display()));
                    for b in list {
                        // Overrides go first so they win.
                        by_hero.entry(hero_internal(&b.hero_id).to_string()).or_default().insert(0, b);
                    }
                }
                Err(e) => note.push_str(&format!(" (override file ignored: {e})")),
            }
        }
        BuildDb { by_hero, source_note: note }
    }

    pub fn for_hero(&self, hero: &str, mood: &str) -> Option<&Build> {
        let list = self.by_hero.get(hero)?;
        list.iter()
            .find(|b| !mood.is_empty() && b.mood.eq_ignore_ascii_case(mood))
            .or_else(|| list.first())
    }

    pub fn hero_count(&self) -> usize {
        self.by_hero.len()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Advice {
    pub next: Option<BuildItem>,
    pub gold_needed: u32,
    pub after: Vec<String>,
    pub phase_tip: String,
    pub tip: Option<String>,
    pub skill_hint: Option<String>,
    pub skills: Vec<String>,
    pub author: String,
}

fn owns(st: &PlayerState, id: &str) -> bool {
    let internal = item_internal(id);
    st.has_item(internal) || upgrades_of(internal).iter().any(|u| st.has_item(u))
}

/// Ultimate level a hero should have by now (6/12/18 rule).
fn expected_ult(level: u32) -> u32 {
    match level {
        0..=5 => 0,
        6..=11 => 1,
        12..=17 => 2,
        _ => 3,
    }
}

pub fn advise(db: &BuildDb, st: &PlayerState, mood: &str) -> Option<Advice> {
    if !st.in_match() {
        return None;
    }
    let skill_hint = st.ultimate().and_then(|u| {
        // Invoker-style heroes level their "ultimate" differently.
        if st.hero == "invoker" || st.hero == "meepo" {
            return None;
        }
        (u.level < expected_ult(st.level)).then(|| "Level your ultimate!".to_string())
    });

    let build = db.for_hero(&st.hero, mood);
    let Some(build) = build else {
        return Some(Advice {
            next: None,
            gold_needed: 0,
            after: vec![],
            phase_tip: String::new(),
            tip: None,
            skill_hint,
            skills: vec![],
            author: String::new(),
        });
    };
    let missing: Vec<&BuildItem> = build.items.iter().filter(|i| !owns(st, &i.id)).collect();
    let next = missing.first().map(|i| (*i).clone());
    let gold_needed = next.as_ref().map(|i| i.cost.saturating_sub(st.gold)).unwrap_or(0);
    let after = missing.iter().skip(1).take(2).map(|i| i.name.clone()).collect();
    let minute = st.clock / 60;
    let phase_tip = if minute < 12 {
        build.gameplan.early.clone()
    } else if minute < 28 {
        build.gameplan.mid.clone()
    } else {
        build.gameplan.late.clone()
    };
    // Rotate through tips every 40 s of game time so it stays fresh but readable.
    let pool: Vec<&String> =
        build.playstyle.dos.iter().chain(build.playstyle.tips.iter()).chain(build.playstyle.donts.iter()).collect();
    let tip = (!pool.is_empty()).then(|| pool[(st.clock.max(0) as usize / 40) % pool.len()].clone());
    Some(Advice {
        next,
        gold_needed,
        after,
        phase_tip,
        tip,
        skill_hint,
        skills: build.skills.clone(),
        author: build.author.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Ability, Item};

    fn db() -> BuildDb {
        BuildDb::load(Path::new("does-not-exist.json"))
    }

    #[test]
    fn legacy_db_loads_with_internal_names() {
        let db = db();
        assert!(db.hero_count() >= 60, "got {}", db.hero_count());
        assert!(db.for_hero("abaddon", "").is_some());
        assert!(db.for_hero("rattletrap", "").is_some(), "clockwerk mapped to internal name");
        assert!(db.for_hero("wisp", "").is_some(), "io mapped to internal name");
    }

    #[test]
    fn next_item_skips_owned_and_upgraded() {
        let db = db();
        let b = db.for_hero("abaddon", "").unwrap();
        let first = &b.items[0];
        let mut st = PlayerState { hero: "abaddon".into(), level: 8, gold: 100, ..Default::default() };
        let a = advise(&db, &st, "").unwrap();
        assert_eq!(a.next.as_ref().unwrap().id, first.id);
        assert_eq!(a.gold_needed, first.cost - 100);
        st.items.push(Item { name: item_internal(&first.id).into(), ..Default::default() });
        let a = advise(&db, &st, "").unwrap();
        assert_ne!(a.next.unwrap().id, first.id);
    }

    #[test]
    fn ult_reminder() {
        let db = db();
        let mut st = PlayerState { hero: "axe".into(), level: 6, ..Default::default() };
        st.abilities.push(Ability { name: "axe_culling_blade".into(), level: 0, ultimate: true, ..Default::default() });
        assert_eq!(advise(&db, &st, "").unwrap().skill_hint.as_deref(), Some("Level your ultimate!"));
        st.abilities[0].level = 1;
        assert_eq!(advise(&db, &st, "").unwrap().skill_hint, None);
    }
}
