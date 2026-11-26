//! Relay connection. Sends my state the instant GSI changes it (watch channel:
//! if two updates arrive while a send is in flight, only the newest goes),
//! applies teammates' updates as they arrive, and reconnects quickly.

use crate::team::{Notifier, SharedTeam};
use futures_util::{SinkExt, StreamExt};
use protocol::{ClientMsg, PlayerState, Role, ServerMsg, PROTOCOL_VERSION};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{mpsc, watch};
use tokio_tungstenite::{connect_async_with_config, tungstenite::Message};
use tracing::{info, warn};

#[derive(Clone, PartialEq, Debug)]
pub struct NetConfig {
    pub url: String,
    pub key: String,
    pub name: String,
}

fn text(msg: &ClientMsg) -> Message {
    Message::Text(serde_json::to_string(msg).expect("serialize").into())
}

/// Runs forever. A new value on `cfg_rx` (name, key or server changed in the
/// menu) drops the current connection and reconnects with it.
pub async fn run(
    mut cfg_rx: watch::Receiver<NetConfig>,
    team: SharedTeam,
    notify: Arc<Notifier>,
    mut state_rx: watch::Receiver<Option<PlayerState>>,
    mut out_rx: mpsc::Receiver<ClientMsg>,
) {
    let mut backoff = Duration::from_millis(250);
    loop {
        let cfg = cfg_rx.borrow_and_update().clone();
        if cfg.key.is_empty() {
            fail(&team, &notify, "no team key - open the menu (Numpad *) to set it".into());
            if cfg_rx.changed().await.is_err() {
                return;
            }
            continue;
        }
        team.lock().unwrap().my_name = cfg.name.clone();
        let attempt = tokio::time::timeout(
            Duration::from_secs(5),
            // disable_nagle = true: small frames leave immediately.
            connect_async_with_config(cfg.url.as_str(), None, true),
        )
        .await;
        let ws = match attempt {
            Ok(Ok((ws, _))) => ws,
            Ok(Err(e)) => {
                fail(&team, &notify, friendly(&e.to_string()));
                wait_or_change(&mut cfg_rx, backoff, &team).await;
                backoff = (backoff * 2).min(Duration::from_secs(3));
                continue;
            }
            Err(_) => {
                fail(&team, &notify, "relay connect timed out".into());
                continue;
            }
        };
        backoff = Duration::from_millis(250);
        info!("connected to relay {}", cfg.url);
        let (mut tx, mut rx) = ws.split();

        let hello = ClientMsg::Hello {
            key: cfg.key.clone(),
            name: cfg.name.clone(),
            role: Role::Player,
            version: PROTOCOL_VERSION,
        };
        if tx.send(text(&hello)).await.is_err() {
            continue;
        }
        // Teammates see us right away, not on our next game-state change.
        let current = state_rx.borrow_and_update().clone();
        if let Some(state) = current {
            let enemy = team.lock().unwrap().enemy.clone();
            let _ = tx.send(text(&ClientMsg::State { state, enemy })).await;
        }

        let mut ping = tokio::time::interval(Duration::from_secs(2));
        let mut pings: HashMap<u64, Instant> = HashMap::new();
        let mut ping_id = 0u64;

        loop {
            tokio::select! {
                _ = cfg_rx.changed() => {
                    info!("settings changed; reconnecting");
                    team.lock().unwrap().last_error = None;
                    let _ = tx.close().await;
                    break;
                }
                frame = rx.next() => {
                    let t = match frame {
                        Some(Ok(Message::Text(t))) => t,
                        Some(Ok(Message::Close(_))) | None | Some(Err(_)) => break,
                        Some(Ok(_)) => continue,
                    };
                    let msg = match serde_json::from_str::<ServerMsg>(t.as_str()) {
                        Ok(m) => m,
                        Err(e) => { warn!("bad frame from relay: {e}"); continue; }
                    };
                    if let ServerMsg::Pong { id, .. } = &msg {
                        if let Some(t0) = pings.remove(id) {
                            let rtt = t0.elapsed().as_secs_f32() * 1000.0;
                            let mut g = team.lock().unwrap();
                            g.rtt_ms = Some(match g.rtt_ms { Some(prev) => prev * 0.7 + rtt * 0.3, None => rtt });
                        }
                        continue;
                    }
                    if let ServerMsg::Error { message } = &msg {
                        warn!("relay says: {message}");
                    }
                    team.lock().unwrap().apply(msg);
                    notify.poke();
                }
                changed = state_rx.changed() => {
                    if changed.is_err() { return; }
                    let latest = state_rx.borrow_and_update().clone();
                    if let Some(state) = latest {
                        let enemy = team.lock().unwrap().enemy.clone();
                        if tx.send(text(&ClientMsg::State { state, enemy })).await.is_err() { break; }
                    }
                }
                out = out_rx.recv() => {
                    let Some(m) = out else { return };
                    if tx.send(text(&m)).await.is_err() { break; }
                }
                _ = ping.tick() => {
                    ping_id += 1;
                    pings.insert(ping_id, Instant::now());
                    pings.retain(|_, t| t.elapsed() < Duration::from_secs(10));
                    if tx.send(text(&ClientMsg::Ping { id: ping_id })).await.is_err() { break; }
                }
            }
        }
        fail(&team, &notify, "disconnected from relay, reconnecting".into());
        // A rejected key or version will not fix itself; don't hammer the relay.
        let fatal = matches!(team.lock().unwrap().last_error.as_deref(), Some(e) if e.contains("key") || e.contains("version"));
        if fatal {
            wait_or_change(&mut cfg_rx, Duration::from_secs(10), &team).await;
        }
    }
}

/// Sleeps, but wakes early when the settings change (and forgets the old error).
async fn wait_or_change(cfg_rx: &mut watch::Receiver<NetConfig>, d: Duration, team: &SharedTeam) {
    tokio::select! {
        _ = tokio::time::sleep(d) => {}
        r = cfg_rx.changed() => {
            if r.is_ok() {
                team.lock().unwrap().last_error = None;
                // Leave the change unseen so the main loop picks up the new value.
                cfg_rx.mark_changed();
            }
        }
    }
}

fn fail(team: &SharedTeam, notify: &Notifier, why: String) {
    let mut g = team.lock().unwrap();
    if g.connected || g.last_error.as_deref() != Some(why.as_str()) {
        warn!("{why}");
    }
    g.connected = false;
    // Keep a relay-side error (like a wrong key) visible instead of overwriting it.
    if !matches!(g.last_error.as_deref(), Some(e) if e.contains("key") || e.contains("version")) {
        g.last_error = Some(why);
    }
    g.rtt_ms = None;
    drop(g);
    notify.poke();
}

/// Connection errors in words a player understands.
pub fn friendly(e: &str) -> String {
    let l = e.to_lowercase();
    if l.contains("refused") {
        "Team server is not answering (is the server address right?)".into()
    } else if l.contains("dns") || l.contains("no such host") || l.contains("failed to lookup") || l.contains("nodename") {
        "Can't find the team server - check your internet".into()
    } else if l.contains("certificate") || l.contains("tls") {
        "Secure connection to the team server failed".into()
    } else if l.contains("timed out") || l.contains("unreachable") {
        "Can't reach the team server - check your internet".into()
    } else {
        format!("Can't connect to the team server: {e}")
    }
}
