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

use crate::{
    ai, analysis, facts,
    opendota::{self, OdStatus},
    record::MIN_DURATION,
    store::valid_id,
    AppState,
};
use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use protocol::now_ms;
use serde::Serialize;
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
    time::Duration,
};
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
        .route("/api/matches/{id}/replay", get(replay))
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
    /// SHA-256 of the agent download; the agent checks it before replacing itself.
    sha256: String,
    live_matches: usize,
    matches: usize,
}

/// (modified time, size, hash) of the published agent, so it is hashed once per publish.
static AGENT_HASH: std::sync::Mutex<Option<(std::time::SystemTime, u64, String)>> = std::sync::Mutex::new(None);

async fn agent_hash(path: &std::path::Path, meta: &std::fs::Metadata) -> String {
    let modified = meta.modified().unwrap_or(std::time::UNIX_EPOCH);
    if let Some((m, n, h)) = AGENT_HASH.lock().unwrap().as_ref() {
        if *m == modified && *n == meta.len() {
            return h.clone();
        }
    }
    let Ok(bytes) = tokio::fs::read(path).await else { return String::new() };
    use sha2::Digest;
    let h: String = sha2::Sha256::digest(&bytes).iter().map(|b| format!("{b:02x}")).collect();
    *AGENT_HASH.lock().unwrap() = Some((modified, meta.len(), h.clone()));
    h
}

async fn info_api(State(app): State<App>) -> Json<Info> {
    let path = app.public.join(AGENT_EXE);
    let meta = tokio::fs::metadata(&path).await.ok();
    let version = tokio::fs::read_to_string(app.public.join("version.txt")).await.unwrap_or_default();
    let sha256 = match &meta {
        Some(m) => agent_hash(&path, m).await,
        None => String::new(),
    };
    Json(Info {
        version: version.trim().to_string(),
        size: meta.as_ref().map(|m| m.len()).unwrap_or(0),
        available: meta.is_some(),
        sha256,
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
    // One review at a time per match: the GPU is shared.
    if app.store.get(&id).is_some_and(|r| r.ai_status == "writing") {
        return (StatusCode::CONFLICT, Json(serde_json::json!({ "status": "already writing" }))).into_response();
    }
    // Re-analyse from the raw recording too, so improved rules apply to old games.
    let Some(report) = rebuild(&app, &id).await.or_else(|| app.store.get(&id)) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    tokio::spawn(write_review(app.clone(), report));
    (StatusCode::ACCEPTED, Json(serde_json::json!({ "status": "writing" }))).into_response()
}

#[derive(serde::Deserialize)]
struct ReplayQuery {
    from: i32,
    to: i32,
}

/// Everyone's position each second between `from` and `to` (at most 3 minutes),
/// read from the raw recording only when someone opens a fight replay.
async fn replay(State(app): State<App>, h: HeaderMap, Path(id): Path<String>, axum::extract::Query(q): axum::extract::Query<ReplayQuery>) -> Response {
    if !authed(&app, &h) {
        return deny();
    }
    let Some(rec) = app.store.get_recording(&id) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let to = q.to.min(q.from + 180);
    let players: Vec<serde_json::Value> = rec
        .tracks
        .values()
        .map(|t| {
            let pts: Vec<[f32; 5]> = t
                .samples
                .iter()
                .filter(|s| s.t >= q.from && s.t <= to)
                .map(|s| [s.t as f32, s.x, s.y, s.hp, if s.alive { 1.0 } else { 0.0 }])
                .collect();
            serde_json::json!({ "name": t.name, "hero": protocol::hero_display(&t.hero), "team": t.team, "pts": pts })
        })
        .collect();
    Json(serde_json::json!({ "from": q.from, "to": to, "players": players })).into_response()
}

#[derive(Serialize, Default)]
struct FactTrend {
    id: String,
    label: String,
    up_good: bool,
    /// Oldest first.
    values: Vec<f32>,
    avg: f32,
    /// Latest game's sentence.
    last: String,
}

#[derive(Serialize, Default)]
struct PlayerAgg {
    name: String,
    games: u32,
    wins: u32,
    titles: BTreeMap<String, u32>,
    heroes: BTreeMap<String, u32>,
    facts: Vec<FactTrend>,
}

/// What a fact's number means when averaged across games.
fn trend_label(id: &str) -> &'static str {
    match id {
        "fights" => "Team fights shown up to (%)",
        "pickoffs" => "Picked off outside fights, per game",
        "alone_deaths" => "Died with no teammate near, per game",
        "calls" => "Help calls answered (%)",
        "skipped_tp" => "Fights skipped with TP ready, per game",
        "item_uses" => "Active item presses per 10 min",
        "wards" => "Observers placed per game",
        "ward_hold" => "Minutes carrying unplaced observers",
        "lazy_gold" => "Minutes on 2,000+ unspent gold",
        "gold_lost" => "Gold lost dying, per game",
        "first_death" => "First to die in fights, per game",
        "ignored_retreat" => "Deaths right after a RETREAT call",
        "far" => "Time far from the team (%)",
        "nemesis" => "Deaths to the same enemy hero",
        _ => "",
    }
}

async fn players(State(app): State<App>, h: HeaderMap) -> Response {
    if !authed(&app, &h) {
        return deny();
    }
    let mut by: BTreeMap<String, PlayerAgg> = BTreeMap::new();
    let mut history = app.store.history(60);
    history.reverse();
    for m in &history {
        for p in &m.players {
            let a = by.entry(p.name.to_lowercase()).or_default();
            a.name = p.name.clone();
            a.games += 1;
            if m.won == Some(true) {
                a.wins += 1;
            }
            *a.titles.entry(p.title.clone()).or_default() += 1;
            *a.heroes.entry(p.hero_name.clone()).or_default() += 1;
            for c in &p.cards {
                let label = trend_label(&c.id);
                if label.is_empty() {
                    continue;
                }
                let idx = match a.facts.iter().position(|f| f.id == c.id) {
                    Some(i) => i,
                    None => {
                        a.facts.push(FactTrend { id: c.id.clone(), label: label.into(), up_good: c.up_good, ..Default::default() });
                        a.facts.len() - 1
                    }
                };
                a.facts[idx].values.push(c.num);
                a.facts[idx].last = c.text.clone();
            }
        }
    }
    let mut out: Vec<PlayerAgg> = by
        .into_values()
        .map(|mut a| {
            for f in &mut a.facts {
                f.avg = (f.values.iter().sum::<f32>() / f.values.len().max(1) as f32 * 10.0).round() / 10.0;
            }
            a
        })
        .collect();
    out.sort_by(|a, b| b.games.cmp(&a.games).then(a.name.cmp(&b.name)));
    Json(out).into_response()
}

/// Analysis from the raw recording, plus OpenDota's facts (if fetched) and
/// each player's history. None if the recording is gone.
pub async fn rebuild(app: &App, id: &str) -> Option<analysis::MatchReport> {
    let rec = app.store.get_recording(id)?;
    let mut r = analysis::analyze(&rec);
    if let Some(m) = app.store.get_opendota(id) {
        let heroes = hero_names(app).await;
        if !heroes.is_empty() {
            facts::apply_opendota(&mut r, &m, &heroes);
            r.od = OdStatus { state: "done".into(), note: String::new(), tries: 0 };
        }
    } else if let Some(old) = app.store.get(id) {
        r.od = old.od;
    }
    facts::apply_usual(&mut r, &app.store.history(15));
    Some(r)
}

/// OpenDota hero ids, fetched once and kept.
async fn hero_names(app: &App) -> HashMap<u32, String> {
    let mut g = app.heroes.lock().await;
    if g.is_empty() {
        match opendota::heroes().await {
            Ok(h) => *g = h,
            Err(e) => warn!("OpenDota heroes: {e}"),
        }
    }
    g.clone()
}

fn set_od(app: &App, id: &str, state: &str, note: String, tries: u32) {
    if let Some(mut r) = app.store.get(id) {
        r.od = OdStatus { state: state.into(), note, tries };
        if let Err(e) = app.store.save(&r) {
            warn!("save report {id}: {e}");
        }
    }
}

/// After a game: wait for the replay, ask OpenDota to parse it, and once it
/// is parsed rebuild the report with the facts and write the review again.
async fn opendota_task(app: App, id: String) {
    if !app.opendota {
        set_od(&app, &id, "off", String::new(), 0);
        return;
    }
    let mut waited = 0u64;
    let mut asked = false;
    for (i, at) in opendota::SCHEDULE_MIN.iter().enumerate() {
        tokio::time::sleep(Duration::from_secs((at - waited) * 60)).await;
        waited = *at;
        let tries = i as u32 + 1;
        match opendota::fetch(&id).await {
            Ok(m) if m.parsed() => {
                if let Err(e) = app.store.save_opendota(&id, &m) {
                    warn!("match {id}: {e}");
                }
                info!("match {id}: OpenDota facts in after {at} min");
                if let Some(r) = rebuild(&app, &id).await {
                    write_review(app.clone(), r).await;
                }
                return;
            }
            Ok(_) => {
                if !asked {
                    asked = true;
                    if let Err(e) = opendota::request_parse(&id).await {
                        warn!("match {id}: {e}");
                    }
                }
                set_od(&app, &id, "parsing", "Waiting for OpenDota to parse the replay".into(), tries);
            }
            // Not known yet, or OpenDota is down: try again on schedule.
            Err(e) => set_od(&app, &id, "waiting", e, tries),
        }
    }
    set_od(&app, &id, "unavailable", "OpenDota could not parse this match (private lobby or no replay)".into(), opendota::SCHEDULE_MIN.len() as u32);
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
            let mut report = analysis::analyze(&rec);
            facts::apply_usual(&mut report, &app.store.history(15));
            report.od = OdStatus { state: "waiting".into(), note: "Replay facts come a few minutes after the game".into(), tries: 0 };
            info!("match {} analysed: {}", rec.match_id, report.summary);
            let id = rec.match_id.clone();
            let a = app.clone();
            tokio::spawn(async move {
                write_review(a.clone(), report).await;
                opendota_task(a, id).await;
            });
        }
    }
}

async fn write_review(app: App, mut report: analysis::MatchReport) {
    if app.ai.url.is_empty() {
        report.ai_status = "off".into();
        if let Err(e) = app.store.save(&report) {
            warn!("save report {}: {e}", report.match_id);
        }
        crate::postgame::push(&app, &report);
        return;
    }
    report.ai_status = "writing".into();
    report.ai_review = None;
    if let Err(e) = app.store.save(&report) {
        warn!("save report {}: {e}", report.match_id);
        return;
    }
    // Players see their card now; the coach's line follows.
    crate::postgame::push(&app, &report);
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
    crate::postgame::push(&app, &report);
}
