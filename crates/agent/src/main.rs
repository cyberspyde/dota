//! cyberspyde-agent: runs on each player's PC.
//!
//!   Dota 2 --(GSI HTTP, localhost)--> agent --(WebSocket)--> relay --> teammates
//!
//! Usage: cyberspyde-agent [--setup] [--config agent.toml] [--demo] [--headless] [--no-install] [--console]

// A windowed program: no console window behind the overlay. Logs go to
// agent.log next to the program; --console (or --headless) opens one.
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod builds;
mod coach;
mod config;
mod demo;
mod dota;
mod forms;
mod gsi;
mod net;
mod overlay;
mod setup;
mod team;
mod theme;

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
    console: bool,
}

fn parse_args() -> Args {
    let mut a = Args { config: config::default_path(), demo: false, headless: false, no_install: false, setup: false, console: false };
    let mut it = std::env::args().skip(1);
    while let Some(x) = it.next() {
        match x.as_str() {
            "--config" => a.config = PathBuf::from(it.next().expect("--config needs a path")),
            "--demo" => a.demo = true,
            "--headless" => a.headless = true,
            "--no-install" => a.no_install = true,
            "--setup" => a.setup = true,
            "--console" => a.console = true,
            "-h" | "--help" => {
                show_console();
                println!(
                    "cyberspyde-agent [--setup] [--config agent.toml] [--demo] [--headless] [--no-install] [--console]\n\
                     --setup       open the setup window again\n\
                     --demo        fake game data, to try the overlay without Dota\n\
                     --headless    no overlay window (just share your data)\n\
                     --no-install  don't write the Dota GSI config file\n\
                     --console     show logs in a console window instead of agent.log"
                );
                std::process::exit(0);
            }
            other => {
                message_box(&format!("Unknown option {other}. Run with --help to see the options."));
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

/// Window and taskbar icon: the brand mark, drawn in code.
pub fn icon() -> egui_icon::IconData {
    const N: u32 = 64;
    let mut rgba = Vec::with_capacity((N * N * 4) as usize);
    let inside = |x: f32, y: f32, lo: f32, hi: f32, r: f32| {
        let cx = x.clamp(lo + r, hi - r);
        let cy = y.clamp(lo + r, hi - r);
        (x - cx).powi(2) + (y - cy).powi(2) <= r * r
    };
    for y in 0..N {
        for x in 0..N {
            let (fx, fy) = (x as f32 + 0.5, y as f32 + 0.5);
            let px = if inside(fx, fy, 18.0, 46.0, 4.0) {
                [7, 9, 13, 255]
            } else if inside(fx, fy, 2.0, 62.0, 14.0) {
                // Teal to blue diagonal.
                let k = (fx + fy) / (2.0 * N as f32);
                [(47.0 + 43.0 * k) as u8, (217.0 - 49.0 * k) as u8, (181.0 + 74.0 * k) as u8, 255]
            } else {
                [0, 0, 0, 0]
            };
            rgba.extend_from_slice(&px);
        }
    }
    egui_icon::IconData { rgba, width: N, height: N }
}
mod egui_icon {
    pub use eframe::egui::IconData;
}

/// A short message for a person who started the program by double-click.
fn message_box(text: &str) {
    #[cfg(windows)]
    unsafe {
        use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONINFORMATION, MB_OK};
        let w: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
        let title: Vec<u16> = "CYBERSPYDE Team Link".encode_utf16().chain(std::iter::once(0)).collect();
        MessageBoxW(std::ptr::null_mut(), w.as_ptr(), title.as_ptr(), MB_OK | MB_ICONINFORMATION);
    }
    #[cfg(not(windows))]
    eprintln!("{text}");
}

/// Attaches to the console we were started from, or opens one.
fn show_console() {
    #[cfg(windows)]
    unsafe {
        use windows_sys::Win32::System::Console::{AllocConsole, AttachConsole, ATTACH_PARENT_PROCESS};
        if AttachConsole(ATTACH_PARENT_PROCESS) == 0 {
            AllocConsole();
        }
    }
}

/// Only one agent at a time: a second copy would fight over the Dota port.
#[cfg(windows)]
fn single_instance() -> bool {
    use windows_sys::Win32::{Foundation::{GetLastError, ERROR_ALREADY_EXISTS}, System::Threading::CreateMutexW};
    let name: Vec<u16> = "Local\\cyberspyde-agent".encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        let h = CreateMutexW(std::ptr::null(), 0, name.as_ptr());
        // Deliberately leaked: the mutex lives as long as the process.
        !h.is_null() && GetLastError() != ERROR_ALREADY_EXISTS
    }
}
#[cfg(not(windows))]
fn single_instance() -> bool {
    true
}

fn init_logging(args: &Args) {
    let filter = || tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,wgpu=warn,eframe=warn,egui_glow=warn".into());
    if args.console || args.headless || cfg!(debug_assertions) {
        if !cfg!(debug_assertions) {
            show_console();
        }
        tracing_subscriber::fmt().with_env_filter(filter()).init();
        return;
    }
    // Fresh log each start, next to the settings file.
    let path = args.config.with_file_name("agent.log");
    match std::fs::File::create(&path) {
        Ok(f) => tracing_subscriber::fmt().with_env_filter(filter()).with_ansi(false).with_writer(Mutex::new(f)).init(),
        Err(_) => tracing_subscriber::fmt().with_env_filter(filter()).init(),
    }
}

fn main() {
    let args = parse_args();
    init_logging(&args);
    if !args.demo && !single_instance() {
        message_box("CYBERSPYDE Team Link is already running.\n\nPress Numpad * in game to open its menu.");
        return;
    }

    let (mut cfg, created) = match config::load_or_create(&args.config) {
        Ok(x) => x,
        Err(e) => {
            error!("{e}");
            message_box(&format!("Could not read the settings:\n{e}\n\nDelete agent.toml next to the program to start fresh."));
            std::process::exit(1);
        }
    };
    // First launch (or no key yet): a setup window instead of a settings file.
    if args.setup || (!args.demo && (created || cfg.key.is_empty())) {
        match setup::run(&cfg) {
            Some(new) => {
                cfg = new;
                if let Err(e) = config::save(&args.config, &cfg) {
                    error!("{e}");
                }
            }
            // Closed without saving on first launch: nothing to run with.
            None if cfg.key.is_empty() => return,
            None => {}
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
    let (net_tx, net_rx) = watch::channel(net::NetConfig { url: cfg.relay.clone(), key: cfg.key.clone(), name: cfg.name.clone() });

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
                if cfg.key.is_empty() {
                    // Offline demo: look connected so the overlay shows as in a game.
                    let mut t = team.lock().unwrap();
                    t.connected = true;
                    t.rtt_ms = Some(31.0);
                }
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
            // Demo without a key stays offline; otherwise connect (the menu can set a key later).
            if !(demo && cfg.key.is_empty()) {
                net::run(net_rx, team, notify, state_rx, out_rx).await;
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
    if let Err(e) = overlay::run(cfg, args.config, team, notify, out_tx, net_tx, db) {
        error!("overlay failed: {e}");
        message_box(&format!("The overlay could not start:\n{e}"));
    }
    rt.shutdown_background();
}
