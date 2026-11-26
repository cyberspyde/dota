//! `--demo`: fake game data so the overlay can be tried without Dota.
//! Generates my own state (which is also sent to the relay, if connected) and
//! four local-only teammates (the real team), one of whom keeps getting ganked.

use crate::team::{Notifier, SharedTeam};
use protocol::{Ability, CalloutKind, Item, PlayerState, ServerMsg};
use std::{sync::Arc, time::Duration};
use tokio::sync::watch;

fn hero(name: &str, level: u32, x: f32, y: f32, items: &[&str]) -> PlayerState {
    PlayerState {
        hero: name.into(),
        level,
        alive: true,
        health: 1400,
        max_health: 1400,
        mana: 600,
        max_mana: 800,
        gold: 1200,
        buyback_cost: 900,
        has_pos: true,
        x,
        y,
        items: items.iter().map(|n| Item { name: (*n).into(), charges: None, cooldown: 0 }).collect(),
        tp: Some(Item { name: "tpscroll".into(), charges: Some(1), cooldown: 0 }),
        abilities: vec![
            Ability { name: "q".into(), level: 3, can_cast: true, ..Default::default() },
            Ability { name: "w".into(), level: 2, can_cast: true, ..Default::default() },
            Ability { name: "e".into(), level: 2, ..Default::default() },
            Ability { name: "r".into(), level: if level >= 6 { 1 } else { 0 }, ultimate: true, ..Default::default() },
        ],
        game_state: "DOTA_GAMERULES_STATE_GAME_IN_PROGRESS".into(),
        team: "radiant".into(),
        ..Default::default()
    }
}

pub async fn run(team: SharedTeam, notify: Arc<Notifier>, state_tx: watch::Sender<Option<PlayerState>>) {
    let mut me = hero("abaddon", 9, -2500.0, -1800.0, &["phase_boots", "magic_wand", "ward_observer"]);
    me.items[2].charges = Some(2);
    // The real team (you are the player running the demo).
    let mut mates = vec![
        (1001, "Azamatjon", hero("juggernaut", 13, -5200.0, -3600.0, &["phase_boots", "bfury", "black_king_bar"])),
        (1002, "Azizbek", hero("lich", 8, 1800.0, 600.0, &["tranquil_boots", "glimmer_cape", "ward_observer", "ward_sentry"])),
        (1003, "Azimjon", hero("faceless_void", 11, 3100.0, 2900.0, &["power_treads", "maelstrom", "black_king_bar"])),
        (1004, "Otabek", hero("lina", 12, -300.0, 200.0, &["arcane_boots", "cyclone"])),
    ];
    mates[1].2.items[2].charges = Some(2);
    // CYBERSPYDE_DEMO_NAMES="A,B,C,D" swaps in other names (public screenshots).
    let custom: Vec<&'static str> = std::env::var("CYBERSPYDE_DEMO_NAMES")
        .ok()
        .map(|v| v.split(',').map(|s| &*Box::leak(s.trim().to_string().into_boxed_str())).collect())
        .unwrap_or_default();
    for (m, n) in mates.iter_mut().zip(custom) {
        m.1 = n;
    }
    {
        let mut t = team.lock().unwrap();
        for (id, name, _) in &mates {
            t.mates.insert(*id, crate::team::Mate::new(*id, (*name).into()));
        }
    }

    let home: Vec<(f32, f32)> = mates.iter().map(|m| (m.2.x, m.2.y)).collect();
    let mut tick = tokio::time::interval(Duration::from_millis(100));
    let mut n: u64 = 0;
    loop {
        tick.tick().await;
        n += 1;
        // 20 s script: 0-3 s Azimjon is ganked alone; 6-10 s Azamatjon and Otabek
        // join me with ults up (a good fight window); 10-14 s we fight and I
        // get low; 14 s Azamatjon calls retreat; then everyone recovers.
        let cycle = n % 200;
        let clock = (n / 10) as i32 + 600;
        me.clock = clock;
        me.gold = 900 + cycle as u32 * 10;
        me.x += ((n as f32) / 30.0).sin() * 15.0;
        me.y += ((n as f32) / 25.0).cos() * 15.0;
        if let Some(r) = me.abilities.last_mut() {
            r.cooldown = match cycle {
                60..=109 => 0,
                110..=199 => (200 - cycle as u32) / 2,
                _ => 0,
            };
        }
        if (100..140).contains(&cycle) {
            me.health = me.health.saturating_sub(25).max(250);
        } else {
            me.health = (me.health + 8).min(me.max_health);
        }
        state_tx.send_replace(Some(me.clone()));
        team.lock().unwrap().set_my_state(me.clone());

        let grouped = (60..170).contains(&cycle);
        for (i, off) in [(0usize, (600.0, 300.0)), (3usize, (-400.0, 700.0))] {
            let mate = &mut mates[i].2;
            if grouped {
                mate.x = me.x + off.0;
                mate.y = me.y + off.1;
            } else {
                mate.x = home[i].0;
                mate.y = home[i].1;
            }
        }
        {
            let otabek = &mut mates[3].2;
            if (100..140).contains(&cycle) {
                otabek.health = otabek.health.saturating_sub(18).max(300);
            } else {
                otabek.health = (otabek.health + 8).min(otabek.max_health);
            }
        }
        if cycle == 140 {
            team.lock().unwrap().apply(ServerMsg::Callout { id: 1001, server_ms: 0, kind: CalloutKind::Retreat, x: me.x, y: me.y });
        }

        // Azimjon gets jumped every 20 s: 3 s of burst damage, dies on every other cycle.
        let azimjon = &mut mates[2].2;
        azimjon.clock = clock;
        if cycle < 30 {
            if azimjon.alive {
                azimjon.health = azimjon.health.saturating_sub(if (n / 200) % 2 == 0 { 25 } else { 60 });
                if azimjon.health == 0 {
                    azimjon.alive = false;
                    azimjon.respawn_seconds = 25;
                }
            }
        } else if !azimjon.alive {
            if n % 10 == 0 {
                azimjon.respawn_seconds = azimjon.respawn_seconds.saturating_sub(1);
            }
            if azimjon.respawn_seconds == 0 {
                azimjon.alive = true;
                azimjon.health = azimjon.max_health;
            }
        } else {
            azimjon.health = (azimjon.health + 4).min(azimjon.max_health);
        }
        if cycle == 5 {
            team.lock().unwrap().apply(ServerMsg::Callout { id: 1003, server_ms: 0, kind: CalloutKind::Help, x: azimjon.x, y: azimjon.y });
        }
        if cycle == 120 {
            team.lock().unwrap().apply(ServerMsg::Note {
                id: 1001,
                server_ms: 0,
                to: None,
                text: "Group mid after my BKB, smoke at 14:00".into(),
            });
        }
        for (id, name, st) in mates.iter_mut() {
            // The relay's Welcome resets the mate list, so re-add fakes by name.
            team.lock().unwrap().mates.entry(*id).or_insert_with(|| crate::team::Mate::new(*id, (*name).into()));
            st.clock = clock;
            if *id == 1002 {
                st.x += ((n as f32 + *id as f32) / 20.0).sin() * 12.0;
                st.y += ((n as f32 + *id as f32) / 23.0).cos() * 12.0;
            }
            team.lock().unwrap().apply(ServerMsg::State { id: *id, server_ms: 0, state: st.clone(), enemy: None });
        }
        notify.poke();
    }
}
