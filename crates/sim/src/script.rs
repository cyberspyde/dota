//! A scripted 40-minute match, played into the relay in seconds, so the match
//! recorder, the analysis, the site and the AI review can be tested without
//! five people in a real game. Each player has a habit the analysis should
//! catch:
//!   0 carry    - joins most fights, sits on gold before big items
//!   1 mid      - always there, calls for help
//!   2 farmer   - farms alone, skips fights with TP ready, dies alone with gold
//!   3 support  - carries wards around for too long, then places them
//!   4 offlane  - ignores a RETREAT call and dies

use protocol::{Ability, CalloutKind, Item, PlayerState, GAME_IN_PROGRESS, GAME_POST_GAME};

pub const LENGTH: i32 = 2400;
pub const HEROES: [&str; 5] = ["juggernaut", "lina", "faceless_void", "lich", "axe"];

/// (start, x, y, participants, our deaths, our kills)
const FIGHTS: [(i32, f32, f32, &[usize], &[usize], u32); 6] = [
    (600, -1500.0, -1200.0, &[0, 1, 3], &[], 2),
    (905, 300.0, 400.0, &[1, 3, 4], &[3], 1),
    (1250, 2500.0, 1500.0, &[0, 1, 3, 4], &[], 3),
    (1600, -3000.0, 2500.0, &[0, 1, 3, 4], &[1], 2),
    (1900, 3500.0, -1000.0, &[0, 1, 3, 4], &[4, 3], 1),
    (2250, 4800.0, 4200.0, &[0, 1, 2, 3, 4], &[], 4),
];
const LANES: [(f32, f32); 5] = [(5200.0, -6000.0), (0.0, 0.0), (-6400.0, -1800.0), (4200.0, -5800.0), (-5800.0, 4800.0)];
/// The farmer dies alone at these times, each after hoarding gold.
const LONE_DEATHS: [i32; 3] = [1100, 1700, 2100];

pub fn callouts() -> Vec<(i32, usize, CalloutKind, f32, f32)> {
    vec![
        (1215, 1, CalloutKind::Help, 2500.0, 1500.0),
        (1560, 3, CalloutKind::Group, -3000.0, 2500.0),
        (1893, 0, CalloutKind::Retreat, 3500.0, -1000.0),
    ]
}

fn wobble(t: i32, p: usize) -> (f32, f32) {
    let a = (t as f32 / 23.0) + p as f32;
    (a.sin() * 450.0, (a * 1.3).cos() * 450.0)
}

pub fn state(p: usize, t: i32) -> PlayerState {
    let mut st = PlayerState {
        hero: HEROES[p].into(),
        match_id: "8800000001".into(),
        game_state: if t >= LENGTH { GAME_POST_GAME.into() } else { GAME_IN_PROGRESS.into() },
        win_team: if t >= LENGTH { "radiant".into() } else { String::new() },
        team: "radiant".into(),
        clock: t.min(LENGTH),
        alive: true,
        has_pos: true,
        level: (1 + t / 100).min(30) as u32,
        max_health: 600 + t as u32,
        max_mana: 400 + t as u32 / 3,
        gpm: [620, 560, 540, 300, 430][p],
        xpm: [640, 700, 600, 380, 520][p],
        buyback_cost: 200 + t as u32 / 2,
        tp: Some(Item { name: "tpscroll".into(), charges: Some(1), cooldown: 0 }),
        ..Default::default()
    };
    st.health = st.max_health;
    st.mana = st.max_mana;
    let (wx, wy) = wobble(t, p);
    let (mut x, mut y) = (LANES[p].0 + wx, LANES[p].1 + wy);

    // Gold: earned steadily, spent every ~5 minutes; the carry and the farmer hoard.
    let earned = st.gpm as i32 * t / 60;
    let cycle = match p {
        0 | 2 => 420,
        _ => 300,
    };
    let since = t % cycle;
    st.gold = (150 + since * st.gpm as i32 / 60).max(0) as u32;
    st.gold_unreliable = st.gold * 2 / 3;
    st.gold_reliable = st.gold - st.gold_unreliable;
    st.last_hits = (earned / 45).max(0) as u32 * [3, 2, 3, 1, 2][p] / 2;

    let mut kills = 0;
    let mut deaths = 0;
    let mut assists = 0;
    for (fs, fx, fy, who, died, k) in FIGHTS {
        let fe = fs + 10;
        if t >= fe {
            if who.contains(&p) {
                if p == 1 || p == 0 {
                    kills += k / 2 + (p as u32 & 1);
                } else {
                    assists += k;
                }
            }
            if died.contains(&p) {
                deaths += 1;
            }
        }
        if who.contains(&p) && (fs - 25..fe + 5).contains(&t) {
            let (ox, oy) = wobble(t, p);
            x = fx + ox * 0.6;
            y = fy + oy * 0.6;
            if (fs..fe).contains(&t) {
                let hurt = (t - fs) as f32 / 10.0 * if died.contains(&p) { 1.0 } else { 0.55 };
                st.health = (st.max_health as f32 * (1.0 - hurt)).max(0.0) as u32;
            }
            if died.contains(&p) && (fe..fe + 30).contains(&t) {
                st.alive = false;
                st.respawn_seconds = (fe + 30 - t) as u32;
            }
        }
    }
    // The farmer hoards before each lone death and loses the unreliable gold.
    if p == 2 {
        for (i, d) in LONE_DEATHS.iter().enumerate() {
            if t >= *d {
                deaths += 1;
            }
            if (d - 150..*d).contains(&t) {
                st.gold = 2400 + i as u32 * 400;
                st.gold_unreliable = 1800;
            }
            if (*d..d + 35).contains(&t) {
                st.alive = false;
                st.respawn_seconds = (d + 35 - t) as u32;
                st.gold = 400;
                st.gold_unreliable = 0;
            }
        }
    }
    st.kills = kills;
    st.deaths = deaths;
    st.assists = assists;
    if !st.alive {
        st.health = 0;
    }
    st.x = x;
    st.y = y;

    // Support: carries 2 observers from 300 to 700 s, then places them one by one.
    let mut items = vec![Item { name: ["power_treads", "arcane_boots", "power_treads", "tranquil_boots", "phase_boots"][p].into(), ..Default::default() }];
    if p == 3 {
        let obs = match t {
            300..=700 => 2,
            701..=760 => 1,
            1000..=1100 => 2,
            1101..=1130 => 1,
            _ => 0,
        };
        if obs > 0 {
            items.push(Item { name: "ward_observer".into(), charges: Some(obs), cooldown: 0 });
        }
        if t > 900 {
            items.push(Item { name: "glimmer_cape".into(), ..Default::default() });
        }
    }
    if p == 0 && t > 1500 {
        items.push(Item { name: "black_king_bar".into(), ..Default::default() });
        items.push(Item { name: "bfury".into(), ..Default::default() });
    }
    if p == 2 && t > 1000 {
        items.push(Item { name: "black_king_bar".into(), ..Default::default() });
    }
    if p == 4 && t > 1200 {
        items.push(Item { name: "blink".into(), ..Default::default() });
    }
    st.items = items;
    st.abilities = vec![Ability {
        name: format!("{}_ultimate", HEROES[p]),
        level: if t > 360 { 1 } else { 0 },
        cooldown: if (t / 60) % 3 == 0 { 30 } else { 0 },
        ultimate: true,
        ..Default::default()
    }];
    st
}
