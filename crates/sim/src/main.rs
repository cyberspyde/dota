//! cyberspyde-sim: connects N fake players to a relay, each sending state at
//! GSI rate, and measures how long every update takes to reach every other
//! player (send -> relay -> receive). All players live in this one process,
//! so a single clock times both ends exactly.
//!
//! cyberspyde-sim --relay ws://127.0.0.1:4747/ws --key <key> [--players 5] [--hz 20] [--secs 10]
//! cyberspyde-sim --relay ... --key ... --script [--names a,b,c,d,e] [--step-ms 5]
//!   plays a scripted 40-minute match for testing the post-game report.

mod script;

use futures_util::{SinkExt, StreamExt};
use protocol::{ClientMsg, PlayerState, Role, ServerMsg, PROTOCOL_VERSION};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio_tungstenite::{connect_async_with_config, tungstenite::Message};

struct Args {
    relay: String,
    key: String,
    players: usize,
    hz: u64,
    secs: u64,
    script: bool,
    names: Vec<String>,
    step_ms: u64,
}

fn args() -> Args {
    let mut a = Args {
        relay: "ws://127.0.0.1:4747/ws".into(),
        key: String::new(),
        players: 5,
        hz: 20,
        secs: 10,
        script: false,
        names: (1..=5).map(|i| format!("Player{i}")).collect(),
        step_ms: 5,
    };
    let mut it = std::env::args().skip(1);
    while let Some(x) = it.next() {
        if x == "--script" {
            a.script = true;
            continue;
        }
        let v = it.next().unwrap_or_default();
        match x.as_str() {
            "--names" => a.names = v.split(',').map(|s| s.trim().to_string()).collect(),
            "--step-ms" => a.step_ms = v.parse().expect("--step-ms"),
            "--relay" => a.relay = v,
            "--key" => a.key = v,
            "--players" => a.players = v.parse().expect("--players"),
            "--hz" => a.hz = v.parse().expect("--hz"),
            "--secs" => a.secs = v.parse().expect("--secs"),
            _ => panic!("unknown arg {x}"),
        }
    }
    a
}

type Sent = Arc<Mutex<HashMap<(u32, u32), Instant>>>;

async fn play_script(a: &Args) {
    assert!(a.names.len() >= 5, "--names needs 5 names");
    let mut conns = Vec::new();
    for name in a.names.iter().take(5) {
        let (ws, _) = connect_async_with_config(a.relay.as_str(), None, true).await.expect("connect");
        let (mut tx, mut rx) = ws.split();
        let hello = ClientMsg::Hello { key: a.key.clone(), name: name.clone(), role: Role::Player, version: PROTOCOL_VERSION };
        tx.send(Message::Text(serde_json::to_string(&hello).unwrap().into())).await.unwrap();
        // Drain incoming frames so the relay never sees us as slow.
        tokio::spawn(async move {
            while let Some(Ok(m)) = rx.next().await {
                if let Message::Text(t) = m {
                    if let Ok(ServerMsg::Error { message }) = serde_json::from_str::<ServerMsg>(t.as_str()) {
                        eprintln!("relay error: {message}");
                        std::process::exit(1);
                    }
                }
            }
        });
        conns.push(tx);
    }
    let send = |m: &ClientMsg| Message::Text(serde_json::to_string(m).unwrap().into());
    let calls = script::callouts();
    for t in 0..=script::LENGTH + 3 {
        for (p, tx) in conns.iter_mut().enumerate() {
            let _ = tx.send(send(&ClientMsg::State { state: script::state(p, t) })).await;
        }
        for (ct, p, kind, x, y) in &calls {
            if *ct == t {
                let _ = conns[*p].send(send(&ClientMsg::Callout { kind: *kind, x: *x, y: *y })).await;
            }
        }
        if t % 300 == 0 {
            println!("game clock {}:{:02}", t / 60, t % 60);
        }
        tokio::time::sleep(Duration::from_millis(a.step_ms)).await;
    }
    println!("post-game sent; the relay writes the report about 45 s later");
    tokio::time::sleep(Duration::from_secs(2)).await;
}

#[tokio::main]
async fn main() {
    let a = args();
    if a.script {
        play_script(&a).await;
        return;
    }
    let sent: Sent = Arc::new(Mutex::new(HashMap::new()));
    let samples = Arc::new(Mutex::new(Vec::<f64>::new()));
    let start = Instant::now();
    let deadline = start + Duration::from_secs(a.secs);

    let mut tasks = Vec::new();
    for p in 0..a.players {
        let (relay, key, sent, samples) = (a.relay.clone(), a.key.clone(), sent.clone(), samples.clone());
        let hz = a.hz;
        tasks.push(tokio::spawn(async move {
            let (ws, _) = connect_async_with_config(relay.as_str(), None, true).await.expect("connect");
            let (mut tx, mut rx) = ws.split();
            let hello = ClientMsg::Hello { key, name: format!("sim{p}"), role: Role::Player, version: PROTOCOL_VERSION };
            tx.send(Message::Text(serde_json::to_string(&hello).unwrap().into())).await.unwrap();

            let reader_samples = samples.clone();
            let reader_sent = sent.clone();
            let reader = tokio::spawn(async move {
                while let Some(Ok(Message::Text(t))) = rx.next().await {
                    match serde_json::from_str::<ServerMsg>(t.as_str()) {
                        Ok(ServerMsg::State { state, .. }) => {
                            let k = (state.kills, state.last_hits);
                            if let Some(t0) = reader_sent.lock().unwrap().get(&k) {
                                reader_samples.lock().unwrap().push(t0.elapsed().as_secs_f64() * 1000.0);
                            }
                        }
                        Ok(ServerMsg::Error { message }) => panic!("relay error: {message}"),
                        _ => {}
                    }
                }
            });

            tokio::time::sleep(Duration::from_millis(300)).await; // let everyone join
            let mut tick = tokio::time::interval(Duration::from_millis(1000 / hz));
            let mut seq = 0u32;
            while Instant::now() < deadline {
                tick.tick().await;
                seq += 1;
                // Realistic payload size: full inventory and abilities.
                let st = PlayerState {
                    hero: "juggernaut".into(),
                    level: 12,
                    alive: true,
                    health: 1200,
                    max_health: 1800,
                    kills: p as u32,
                    last_hits: seq,
                    items: (0..6).map(|i| protocol::Item { name: format!("item{i}"), charges: None, cooldown: 0 }).collect(),
                    abilities: (0..6)
                        .map(|i| protocol::Ability { name: format!("ab{i}"), level: 2, ultimate: i == 5, ..Default::default() })
                        .collect(),
                    ..Default::default()
                };
                sent.lock().unwrap().insert((p as u32, seq), Instant::now());
                let m = ClientMsg::State { state: st };
                if tx.send(Message::Text(serde_json::to_string(&m).unwrap().into())).await.is_err() {
                    break;
                }
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
            reader.abort();
        }));
    }
    for t in tasks {
        t.await.expect("player task");
    }

    let mut s = samples.lock().unwrap().clone();
    if s.is_empty() {
        eprintln!("no samples received - is the relay running and the key right?");
        std::process::exit(1);
    }
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let pct = |p: f64| s[((s.len() as f64 - 1.0) * p) as usize];
    let expected = a.players * (a.players - 1) * (a.hz * a.secs) as usize;
    println!(
        "players={} hz={} delivered={} (~{} expected)\nlatency ms: p50={:.2} p95={:.2} p99={:.2} max={:.2}",
        a.players,
        a.hz,
        s.len(),
        expected,
        pct(0.50),
        pct(0.95),
        pct(0.99),
        s[s.len() - 1]
    );
}
