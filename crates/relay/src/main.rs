//! cyberspyde-relay: one process, one team room.
//!
//! Each agent sends its own player's state; the relay forwards it to every
//! other member the moment it arrives. No batching, no tick: a message is
//! serialized once and pushed onto each member's outgoing queue, and sockets
//! run with TCP_NODELAY so small frames are not held back by Nagle.

mod ai;
mod analysis;
mod hub;
mod profiles;
mod record;
mod store;
mod web;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        ConnectInfo, State,
    },
    response::IntoResponse,
    routing::get,
};
use futures_util::{SinkExt, StreamExt};
use hub::Hub;
use protocol::{now_ms, ClientMsg, Role, ServerMsg, PROTOCOL_VERSION};
use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::{mpsc, oneshot};
use tracing::{info, warn};

const HELLO_TIMEOUT: Duration = Duration::from_secs(5);
/// Agents ping every 2 s, so 20 s of silence means the connection is dead.
const IDLE_TIMEOUT: Duration = Duration::from_secs(20);

struct Config {
    bind: SocketAddr,
    key: String,
    team: Option<PathBuf>,
    data: PathBuf,
    public: PathBuf,
    ai: ai::AiConfig,
}

/// Shared by every connection handler.
pub struct AppState {
    pub hub: Hub,
    pub team: Option<PathBuf>,
    pub recorder: Mutex<record::Recorder>,
    pub store: store::Store,
    pub public: PathBuf,
    pub ai: ai::AiConfig,
}

fn parse_args() -> Result<Config, String> {
    let mut bind: SocketAddr = "0.0.0.0:4747".parse().unwrap();
    let mut key = std::env::var("RELAY_KEY").unwrap_or_default();
    let mut team = None;
    let mut data = PathBuf::from("data");
    let mut public = PathBuf::from("public");
    let mut ai = ai::AiConfig { url: String::new(), model: "qwen3.6-agent".into(), language: "English".into() };
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--bind" => {
                bind = args
                    .next()
                    .ok_or("--bind needs an address")?
                    .parse()
                    .map_err(|e| format!("bad --bind: {e}"))?
            }
            "--key" => key = args.next().ok_or("--key needs a value")?,
            // Preferred for services: the key stays in an ACL-protected file,
            // not in the command line or environment other accounts can read.
            "--key-file" => {
                let p = args.next().ok_or("--key-file needs a path")?;
                key = std::fs::read_to_string(&p).map_err(|e| format!("read {p}: {e}"))?.trim().to_string();
            }
            "--team" => team = Some(PathBuf::from(args.next().ok_or("--team needs a path")?)),
            "--data-dir" => data = PathBuf::from(args.next().ok_or("--data-dir needs a path")?),
            "--public-dir" => public = PathBuf::from(args.next().ok_or("--public-dir needs a path")?),
            "--ai-url" => ai.url = args.next().ok_or("--ai-url needs a URL")?,
            "--ai-model" => ai.model = args.next().ok_or("--ai-model needs a name")?,
            "--ai-lang" => ai.language = args.next().ok_or("--ai-lang needs a language")?,
            "analyze" => {
                // cyberspyde-relay analyze <match.rec.json>: re-run the analysis offline.
                let p = args.next().ok_or("analyze needs a .rec.json path")?;
                let text = std::fs::read_to_string(&p).map_err(|e| format!("read {p}: {e}"))?;
                let rec: record::Recording = serde_json::from_str(&text).map_err(|e| format!("parse {p}: {e}"))?;
                println!("{}", serde_json::to_string_pretty(&analysis::analyze(&rec)).unwrap());
                std::process::exit(0);
            }
            "-h" | "--help" => {
                println!(
                    "cyberspyde-relay [--bind 0.0.0.0:4747] (--key <team-key> | --key-file <path>) [--team team.toml]\n\
                     --team: private per-player coaching profiles, re-read on each connect.\n\
                     --data-dir data: match history. --public-dir public: cyberspyde-agent.exe and version.txt for the site.\n\
                     --ai-url http://127.0.0.1:11434 [--ai-model qwen3.6-agent] [--ai-lang English]: post-game reviews by Ollama.\n\
                     `cyberspyde-relay analyze <match.rec.json>` re-runs the analysis and prints the report.\n\
                     The key can also come from the RELAY_KEY environment variable.\n\
                     Dashboard: http://<host>:<port>/#key=<team-key>"
                );
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument {other}")),
        }
    }
    if key.len() < 6 {
        return Err("set a team key of at least 6 characters with --key or RELAY_KEY".into());
    }
    if let Some(t) = &team {
        profiles::load(t)?;
    }
    Ok(Config { bind, key, team, data, public, ai })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        // Plain text when running as a service with output going to a log file.
        .with_ansi(std::io::IsTerminal::is_terminal(&std::io::stdout()))
        .init();

    let cfg = match parse_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}\nrun with --help for usage");
            std::process::exit(2);
        }
    };

    if let Some(t) = &cfg.team {
        info!("coaching profiles: {}", t.display());
    }
    let store = match store::Store::open(&cfg.data) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(2);
        }
    };
    info!("match history: {} ({} matches)", cfg.data.display(), store.list().len());
    if cfg.ai.url.is_empty() {
        info!("AI reviews off (no --ai-url)");
    } else {
        info!("AI reviews: {} model {}", cfg.ai.url, cfg.ai.model);
    }
    let hub = Arc::new(AppState {
        hub: Hub::new(cfg.key),
        team: cfg.team,
        recorder: Mutex::new(record::Recorder::default()),
        store,
        public: cfg.public,
        ai: cfg.ai,
    });
    tokio::spawn(web::finisher(hub.clone()));
    let app = web::routes()
        .route("/health", get(|| async { "ok" }))
        .route("/ws", get(ws_upgrade))
        .with_state(hub);

    let listener = tokio::net::TcpListener::bind(cfg.bind)
        .await
        .unwrap_or_else(|e| panic!("cannot bind {}: {e}", cfg.bind));
    info!("relay listening on {} (ws path /ws, site /)", cfg.bind);

    let listener = axum::serve::ListenerExt::tap_io(listener, |tcp| {
        if let Err(e) = tcp.set_nodelay(true) {
            warn!("set_nodelay failed: {e}");
        }
    });
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(async {
        let _ = tokio::signal::ctrl_c().await;
    })
    .await
    .expect("server error");
}

async fn ws_upgrade(
    ws: WebSocketUpgrade,
    State(hub): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    // Behind the edge proxy every peer is 127.0.0.1; name the real client in logs.
    let h = |n: &str| headers.get(n).and_then(|v| v.to_str().ok()).unwrap_or("").to_string();
    let fwd = h("x-forwarded-for");
    let ua: String = h("user-agent").chars().take(60).collect();
    let peer = if fwd.is_empty() { addr.to_string() } else { format!("{fwd} [{ua}]") };
    ws.max_message_size(256 * 1024)
        .on_upgrade(move |socket| handle_socket(socket, hub, peer))
}

fn encode(msg: &ServerMsg) -> axum::extract::ws::Utf8Bytes {
    serde_json::to_string(msg).expect("serialize").into()
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>, addr: String) {
    let app = state.clone();
    let hub = &state.hub;
    let (mut sink, mut stream) = socket.split();

    // 1. Handshake.
    // Why a handshake failed, for the log (never the frame itself: it holds the key).
    let mut why = String::new();
    let hello = match tokio::time::timeout(HELLO_TIMEOUT, stream.next()).await {
        Ok(Some(Ok(Message::Text(t)))) => match serde_json::from_str::<ClientMsg>(t.as_str()) {
            Ok(m) => Some(m),
            Err(e) => {
                why = format!("unreadable hello ({e})");
                None
            }
        },
        Ok(Some(Ok(other))) => {
            why = format!("first frame was {}", match other { Message::Binary(_) => "binary", Message::Ping(_) => "ping", Message::Close(_) => "close", _ => "other" });
            None
        }
        Ok(Some(Err(e))) => {
            why = format!("socket error {e}");
            None
        }
        Ok(None) => {
            why = "closed before hello".into();
            None
        }
        Err(_) => {
            why = "no hello within 5 s".into();
            None
        }
    };
    let (name, role) = match hello {
        Some(ClientMsg::Hello { key, name, role, version }) => {
            if !hub.check_key(&key) {
                warn!("{addr}: wrong team key");
                let _ = sink
                    .send(Message::Text(encode(&ServerMsg::Error {
                        message: "wrong team key".into(),
                    })))
                    .await;
                return;
            }
            if version != PROTOCOL_VERSION {
                let _ = sink
                    .send(Message::Text(encode(&ServerMsg::Error {
                        message: format!(
                            "protocol version {version} not supported, relay speaks {PROTOCOL_VERSION}; update the agent"
                        ),
                    })))
                    .await;
                return;
            }
            let name: String = name.trim().chars().take(24).collect();
            (if name.is_empty() { "player".to_string() } else { name }, role)
        }
        _ => {
            warn!("{addr}: no valid hello: {why}");
            return;
        }
    };

    // 2. Join the room.
    let (tx, mut rx) = mpsc::channel::<axum::extract::ws::Utf8Bytes>(256);
    let (kick_tx, mut kick_rx) = oneshot::channel::<()>();
    let (id, members) = hub.join(name.clone(), role, tx.clone(), kick_tx);
    info!("{addr}: {name} joined as #{id} ({role:?})");
    // One person is one row in match history, whatever case they typed.
    let canon = profiles::canonical(state.team.as_deref(), &name);

    // Only this player's own switches; never anyone else's.
    let coaching = if role == Role::Player {
        profiles::for_player(state.team.as_deref(), &name)
    } else {
        protocol::Coaching::default()
    };
    if sink
        .send(Message::Text(encode(&ServerMsg::Welcome {
            you: id,
            server_ms: now_ms(),
            members,
            coaching,
            latest_agent: std::fs::read_to_string(state.public.join("version.txt")).unwrap_or_default().trim().to_string(),
        })))
        .await
        .is_err()
    {
        hub.leave(id);
        return;
    }

    // 3. Writer: drains this member's queue into the socket.
    let writer = tokio::spawn(async move {
        while let Some(frame) = rx.recv().await {
            if sink.send(Message::Text(frame)).await.is_err() {
                break;
            }
        }
        let _ = sink.close().await;
    });

    // 4. Reader: every inbound frame is handled and fanned out immediately.
    loop {
        let next = tokio::select! {
            _ = &mut kick_rx => {
                info!("#{id} {name}: replaced by a newer connection with the same name");
                break;
            }
            r = tokio::time::timeout(IDLE_TIMEOUT, stream.next()) => r,
        };
        let frame = match next {
            Err(_) => {
                info!("#{id} {name}: idle timeout");
                break;
            }
            Ok(None) | Ok(Some(Err(_))) => break,
            Ok(Some(Ok(m))) => m,
        };
        let text = match frame {
            Message::Text(t) => t,
            Message::Close(_) => break,
            _ => continue,
        };
        let msg = match serde_json::from_str::<ClientMsg>(text.as_str()) {
            Ok(m) => m,
            Err(e) => {
                warn!("#{id}: bad frame: {e}");
                continue;
            }
        };
        match msg {
            ClientMsg::Ping { id: ping } => {
                let _ = tx.try_send(encode(&ServerMsg::Pong { id: ping, server_ms: now_ms() }));
            }
            ClientMsg::State { state } if role == Role::Player => {
                let out = encode(&ServerMsg::State { id, server_ms: now_ms(), state: state.clone() });
                app.recorder.lock().unwrap().on_state(&canon, &state, now_ms());
                hub.set_state(id, state);
                hub.broadcast(&out, Some(id));
            }
            ClientMsg::Callout { kind, x, y } if role == Role::Player => {
                let out = encode(&ServerMsg::Callout { id, server_ms: now_ms(), kind, x, y });
                app.recorder.lock().unwrap().on_callout(&canon, kind, x, y);
                hub.broadcast(&out, None);
            }
            ClientMsg::Note { to, text } => {
                let text: String = text.chars().take(200).collect();
                let out = encode(&ServerMsg::Note { id, server_ms: now_ms(), to, text });
                hub.broadcast(&out, None);
            }
            _ => {}
        }
    }

    hub.leave(id);
    drop(tx);
    writer.abort();
    info!("#{id} {name} left");
}
