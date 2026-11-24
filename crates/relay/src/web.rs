//! The public site and its API.
//!
//!   /                          site (landing, download, live board, history)
//!   /download/cyberspyde-agent.exe
//!   /api/info                  public: agent version and download size
//!   /api/matches               team key: match list
//!   /api/matches/{id}          team key: full report
//!   /api/matches/{id}/review   team key, POST: write the AI review again
//!   /api/players               team key: each player across all matches
//!
//! The team key travels in the `x-team-key` header (the site keeps it in the
//! browser after the first visit with `#key=`), never in a query string.

use crate::{ai, analysis, record::MIN_DURATION, store::valid_id, AppState};
use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use protocol::now_ms;
use serde::Serialize;
use std::{collections::BTreeMap, sync::Arc, time::Duration};
use tracing::{info, warn};

const SITE: &str = include_str!("site.html");
const AGENT_EXE: &str = "cyberspyde-agent.exe";

type App = Arc<AppState>;

pub fn routes() -> Router<App> {
    Router::new()
        .route("/", get(|| async { Html(SITE) }))
        .route("/download/cyberspyde-agent.exe", get(download))
        .route("/assets/{name}", get(asset))
        .route("/api/info", get(info_api))
        .route("/api/matches", get(matches))
        .route("/api/matches/{id}", get(match_one))
        .route("/api/matches/{id}/review", post(rereview))
        .route("/api/players", get(players))
}

fn authed(app: &AppState, h: &HeaderMap) -> bool {
    h.get("x-team-key").and_then(|v| v.to_str().ok()).is_some_and(|k| app.hub.check_key(k))
}

fn deny() -> Response {
    (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "team key required" }))).into_response()
}

async fn download(State(app): State<App>) -> Response {
    match tokio::fs::read(app.public.join(AGENT_EXE)).await {
        Ok(bytes) => (
            [
                (header::CONTENT_TYPE, "application/vnd.microsoft.portable-executable"),
                (header::CONTENT_DISPOSITION, "attachment; filename=\"cyberspyde-agent.exe\""),
                (header::CACHE_CONTROL, "no-cache"),
            ],
            bytes,
        )
            .into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "the agent is not published yet").into_response(),
    }
}

/// Images for the landing page, from <public>/assets. Plain names only.
async fn asset(State(app): State<App>, Path(name): Path<String>) -> Response {
    let ok_name = !name.is_empty() && name.len() <= 64 && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.') && !name.starts_with('.');
    let ctype = match name.rsplit('.').next() {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        _ => "",
    };
    if !ok_name || ctype.is_empty() {
        return StatusCode::NOT_FOUND.into_response();
    }
    match tokio::fs::read(app.public.join("assets").join(&name)).await {
        Ok(bytes) => ([(header::CONTENT_TYPE, ctype), (header::CACHE_CONTROL, "public, max-age=3600")], bytes).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[derive(Serialize)]
struct Info {
    version: String,
    size: u64,
    available: bool,
    live_matches: usize,
    matches: usize,
}

async fn info_api(State(app): State<App>) -> Json<Info> {
    let meta = tokio::fs::metadata(app.public.join(AGENT_EXE)).await.ok();
    let version = tokio::fs::read_to_string(app.public.join("version.txt")).await.unwrap_or_default();
    Json(Info {
        version: version.trim().to_string(),
        size: meta.as_ref().map(|m| m.len()).unwrap_or(0),
        available: meta.is_some(),
        live_matches: app.recorder.lock().unwrap().live_ids().len(),
        matches: app.store.list().len(),
    })
}

async fn matches(State(app): State<App>, h: HeaderMap) -> Response {
    if !authed(&app, &h) {
        return deny();
    }
    let live: Vec<_> = app
        .recorder
        .lock()
        .unwrap()
        .live_ids()
        .into_iter()
        .map(|(id, t, players)| serde_json::json!({ "match_id": id, "clock": analysis::clock(t), "players": players }))
        .collect();
    Json(serde_json::json!({ "live": live, "matches": app.store.list() })).into_response()
}

async fn match_one(State(app): State<App>, h: HeaderMap, Path(id): Path<String>) -> Response {
    if !authed(&app, &h) {
        return deny();
    }
    match app.store.get(&id) {
        Some(r) => Json(r).into_response(),
        None => (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "no such match" }))).into_response(),
    }
}

async fn rereview(State(app): State<App>, h: HeaderMap, Path(id): Path<String>) -> Response {
    if !authed(&app, &h) {
        return deny();
    }
    if !valid_id(&id) {
        return StatusCode::NOT_FOUND.into_response();
    }
    // Re-analyse from the raw recording too, so improved rules apply to old games.
    let report = match app.store.get_recording(&id) {
        Some(rec) => analysis::analyze(&rec),
        None => match app.store.get(&id) {
            Some(r) => r,
            None => return StatusCode::NOT_FOUND.into_response(),
        },
    };
    tokio::spawn(write_review(app.clone(), report));
    (StatusCode::ACCEPTED, Json(serde_json::json!({ "status": "writing" }))).into_response()
}

#[derive(Serialize, Default)]
struct PlayerAgg {
    name: String,
    games: u32,
    wins: u32,
    avg_usefulness: f32,
    best_usefulness: u32,
    deaths_alone: u32,
    fights_present: u32,
    fights_total: u32,
    calls_answered: u32,
    calls_total: u32,
    fights_missed_tp_ready: u32,
    hoard_minutes: f32,
    titles: BTreeMap<String, u32>,
    heroes: BTreeMap<String, u32>,
    /// Oldest first, for a trend line.
    trend: Vec<u32>,
}

async fn players(State(app): State<App>, h: HeaderMap) -> Response {
    if !authed(&app, &h) {
        return deny();
    }
    let mut by: BTreeMap<String, PlayerAgg> = BTreeMap::new();
    let mut list = app.store.list();
    list.reverse();
    for m in &list {
        for p in &m.players {
            let a = by.entry(p.name.to_lowercase()).or_default();
            a.name = p.name.clone();
            a.games += 1;
            if m.won == Some(true) {
                a.wins += 1;
            }
            a.avg_usefulness += p.usefulness as f32;
            a.best_usefulness = a.best_usefulness.max(p.usefulness);
            a.deaths_alone += p.deaths_alone;
            a.fights_present += p.fights_present;
            a.fights_total += p.fights_total;
            a.calls_answered += p.calls_answered;
            a.calls_total += p.calls_total;
            a.fights_missed_tp_ready += p.fights_missed_tp_ready;
            a.hoard_minutes += p.hoard_s as f32 / 60.0;
            *a.titles.entry(p.title.clone()).or_default() += 1;
            *a.heroes.entry(p.hero_name.clone()).or_default() += 1;
            a.trend.push(p.usefulness);
        }
    }
    let mut out: Vec<PlayerAgg> = by
        .into_values()
        .map(|mut a| {
            a.avg_usefulness = (a.avg_usefulness / a.games.max(1) as f32 * 10.0).round() / 10.0;
            a.hoard_minutes = a.hoard_minutes.round();
            a
        })
        .collect();
    out.sort_by(|a, b| b.avg_usefulness.total_cmp(&a.avg_usefulness));
    Json(out).into_response()
}

/// Every 15 s: close finished matches, analyse them, save, ask for a review.
pub async fn finisher(app: App) {
    let mut tick = tokio::time::interval(Duration::from_secs(15));
    loop {
        tick.tick().await;
        let done = app.recorder.lock().unwrap().take_finished(now_ms());
        for rec in done {
            let dur = rec.duration();
            if dur < MIN_DURATION || rec.tracks.is_empty() {
                info!("match {} ended after {} - too short to report", rec.match_id, analysis::clock(dur));
                continue;
            }
            if let Err(e) = app.store.save_recording(&rec) {
                warn!("match {}: {e}", rec.match_id);
            }
            let report = analysis::analyze(&rec);
            info!("match {} analysed: {}", rec.match_id, report.summary);
            tokio::spawn(write_review(app.clone(), report));
        }
    }
}

async fn write_review(app: App, mut report: analysis::MatchReport) {
    if app.ai.url.is_empty() {
        report.ai_status = "off".into();
        if let Err(e) = app.store.save(&report) {
            warn!("save report {}: {e}", report.match_id);
        }
        return;
    }
    report.ai_status = "writing".into();
    report.ai_review = None;
    if let Err(e) = app.store.save(&report) {
        warn!("save report {}: {e}", report.match_id);
        return;
    }
    match ai::review(&app.ai, &report).await {
        Ok(text) => {
            report.ai_review = Some(text);
            report.ai_status = "done".into();
            info!("match {}: AI review written", report.match_id);
        }
        Err(e) => {
            warn!("match {}: AI review failed: {e}", report.match_id);
            report.ai_status = format!("failed: {e}");
        }
    }
    if let Err(e) = app.store.save(&report) {
        warn!("save report {}: {e}", report.match_id);
    }
}
