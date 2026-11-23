//! cyberspyde-agent: runs on each player's PC.
//!
//!   Dota 2 --(GSI HTTP, localhost)--> agent --(WebSocket)--> relay --> teammates
//!
//! Usage: cyberspyde-agent [--setup] [--config agent.toml] [--demo] [--headless] [--no-install]

mod builds;
mod coach;
mod config;
mod demo;
mod dota;
mod gsi;
mod net;
mod overlay;
mod setup;
mod team;

use axum::{body::Bytes, extract::State, http::StatusCode, routing::post, Router};
use protocol::PlayerState;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use team::{Notifier, SharedTeam, Team};
use tokio::sync::{mpsc, watch};
use tracing::{error, info, warn};

struct Args {
    config: PathBuf,
    demo: bool,
    headless: bool,
    no_install: bool,
    setup: bool,
}

fn parse_args() -> Args {
    let mut a = Args { config: config::default_path(), demo: false, headless: false, no_install: false, setup: false };
    let mut it = std::env::args().skip(1);
    while let Some(x) = it.next() {
        match x.as_str() {
            "--config" => a.config = PathBuf::from(it.next().expect("--config needs a path")),
            "--demo" => a.demo = true,
            "--headless" => a.headless = true,
            "--no-install" => a.no_install = true,
            "--setup" => a.setup = true,
            "-h" | "--help" => {
                println!(
                    "cyberspyde-agent [--setup] [--config agent.toml] [--demo] [--headless] [--no-install]\n\
                     --setup       ask for name, host, key and overlay look again\n\
                     --demo        fake game data, to try the overlay without Dota\n\
                     --headless    no overlay window (just share your data)\n\
                     --no-install  don't write the Dota GSI config file"
                );
                std::process::exit(0);
            }
            other => {
                eprintln!("unknown argument {other}; try --help");
                std::process::exit(2);
            }
        }
    }
    a
}

struct GsiCtx {
    token: String,
    team: SharedTeam,
    notify: Arc<Notifier>,
    state_tx: watch::Sender<Option<PlayerState>>,
    last: Mutex<Option<PlayerState>>,
}

async fn gsi_post(State(ctx): State<Arc<GsiCtx>>, body: Bytes) -> StatusCode {
    let v: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(_) => return StatusCode::BAD_REQUEST,
    };
    match gsi::parse(&v, &ctx.token) {
        gsi::Parsed::Unauthorized => StatusCode::UNAUTHORIZED,
        gsi::Parsed::Spectator => StatusCode::OK,
        gsi::Parsed::State(st) => {
            let mut last = ctx.last.lock().unwrap();
            if last.as_ref() != Some(&st) {
                *last = Some(st.clone());
                drop(last);
                ctx.team.lock().unwrap().set_my_state(st.clone());
                // Wakes the relay sender immediately.
                ctx.state_tx.send_replace(Some(st));
                ctx.notify.poke();
            } else {
                drop(last);
                ctx.team.lock().unwrap().last_gsi = Some(std::time::Instant::now());
            }
            StatusCode::OK
        }
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,wgpu=warn,eframe=warn".into()),
        )
        .init();
    let args = parse_args();

    let (mut cfg, created) = match config::load_or_create(&args.config) {
        Ok(x) => x,
        Err(e) => {
            error!("{e}");
            std::process::exit(1);
        }
    };
    // First launch (or no key yet): ask instead of making people edit a file.
    if args.setup || (!args.demo && (created || cfg.key.is_empty())) {
        if setup::run(&mut cfg) {
            if let Err(e) = config::save(&args.config, &cfg) {
                error!("{e}");
            }
        }
    }
    if cfg.key.is_empty() && !args.demo {
        warn!("no team key set in {}; the relay will reject this agent", args.config.display());
    }

    if !args.no_install && !args.demo {
        match dota::find_dota(&cfg.dota_path) {
            Some(dir) => match dota::install(&dir, cfg.gsi_port, &cfg.gsi_token) {
                Ok((path, true)) => info!(
                    "wrote {} - restart Dota 2 once, with launch option -gamestateintegration",
                    path.display()
                ),
                Ok((path, false)) => info!("GSI config up to date: {}", path.display()),
                Err(e) => warn!("could not write the GSI config in {}: {e}", dir.display()),
            },
            None => warn!("Dota 2 not found; set dota_path in {}", args.config.display()),
        }
    }

    let override_path = args.config.with_file_name("builds.override.json");
    let db = Arc::new(builds::BuildDb::load(&override_path));
    info!("build helper: {} ({} heroes)", db.source_note, db.hero_count());

    let team: SharedTeam = Arc::new(Mutex::new(Team::new(cfg.name.clone())));
    let notify = Arc::new(Notifier::default());
    let (state_tx, state_rx) = watch::channel::<Option<PlayerState>>(None);
    let (out_tx, out_rx) = mpsc::channel(64);

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .expect("tokio runtime");

    {
        let (team, notify, cfg) = (team.clone(), notify.clone(), cfg.clone());
        let demo = args.demo;
        rt.spawn(async move {
            if demo {
                tokio::spawn(demo::run(team.clone(), notify.clone(), state_tx));
            } else {
                let ctx = Arc::new(GsiCtx {
                    token: cfg.gsi_token.clone(),
                    team: team.clone(),
                    notify: notify.clone(),
                    state_tx,
                    last: Mutex::new(None),
                });
                let app = Router::new().route("/", post(gsi_post)).with_state(ctx);
                let addr = std::net::SocketAddr::from(([127, 0, 0, 1], cfg.gsi_port));
                match tokio::net::TcpListener::bind(addr).await {
                    Ok(l) => {
                        info!("listening for Dota game state on http://{addr}/");
                        tokio::spawn(async move {
                            if let Err(e) = axum::serve(l, app).await {
                                error!("GSI server stopped: {e}");
                            }
                        });
                    }
                    Err(e) => error!("cannot listen on {addr}: {e} (is another agent running?)"),
                }
            }
            if !cfg.key.is_empty() {
                net::run(
                    net::NetConfig { url: cfg.relay.clone(), key: cfg.key.clone(), name: cfg.name.clone() },
                    team,
                    notify,
                    state_rx,
                    out_rx,
                )
                .await;
            }
        });
    }

    if args.headless || !cfg.overlay.enabled {
        info!("running headless; Ctrl+C to quit");
        rt.block_on(async {
            let _ = tokio::signal::ctrl_c().await;
        });
        return;
    }
    if let Err(e) = overlay::run(cfg, args.config, team, notify, out_tx, db) {
        error!("overlay failed: {e}");
    }
    rt.shutdown_background();
}
