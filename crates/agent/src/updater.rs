//! One-click and automatic updates.
//!
//! On launch the agent asks the team site which version it offers. If that
//! is newer, a small window downloads it, checks its SHA-256 against the one
//! the site publishes, swaps the program file (the running exe is renamed to
//! `.old`, which Windows allows) and starts the new version. Anything that
//! goes wrong leaves the current version running; the update is tried again
//! next launch.

use crate::{forms, setup::site_url, theme};
use eframe::egui::{self, CornerRadius, RichText, Vec2};
use serde::Deserialize;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Info {
    pub version: String,
    pub available: bool,
    pub sha256: String,
    pub size: u64,
}

/// "0.2.10" > "0.2.9"
pub fn newer(remote: &str, local: &str) -> bool {
    fn parse(v: &str) -> Vec<u32> {
        v.trim().split('.').map(|p| p.parse().unwrap_or(0)).collect()
    }
    !remote.trim().is_empty() && parse(remote) > parse(local)
}

fn old_path(exe: &Path) -> PathBuf {
    exe.with_extension("old")
}

/// Removes the previous version left behind by the last update.
pub fn cleanup() {
    if let Ok(exe) = std::env::current_exe() {
        let _ = std::fs::remove_file(old_path(&exe));
        let _ = std::fs::remove_file(exe.with_extension("new"));
    }
}

/// Developer builds (under `target\`) never replace themselves.
fn dev_build(exe: &Path) -> bool {
    cfg!(debug_assertions) || exe.components().any(|c| c.as_os_str().eq_ignore_ascii_case("target"))
}

fn runtime() -> Option<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_current_thread().enable_all().build().ok()
}

fn client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(120))
        .connect_timeout(Duration::from_secs(4))
        .user_agent(concat!("cyberspyde-agent/", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|e| e.to_string())
}

/// What the site offers, if it is newer than this program.
pub fn check(relay: &str) -> Option<Info> {
    let rt = runtime()?;
    let url = format!("{}/api/info", site_url(relay));
    let info: Info = rt
        .block_on(async {
            let c = client().ok()?;
            let r = tokio::time::timeout(Duration::from_secs(5), c.get(&url).send()).await.ok()?.ok()?;
            r.json::<Info>().await.ok()
        })?;
    (info.available && info.sha256.len() == 64 && newer(&info.version, protocol::AGENT_VERSION)).then_some(info)
}

#[derive(Clone, Debug)]
enum Stage {
    Downloading { got: u64, total: u64 },
    Installing,
    Done,
    Failed(String),
}

/// Downloads, verifies and swaps the exe. Progress goes to `stage`.
async fn download_and_install(relay: String, info: Info, stage: Arc<Mutex<Stage>>, ctx: egui::Context) -> Result<(), String> {
    use futures_util::StreamExt;
    use sha2::Digest;
    let url = format!("{}/download/cyberspyde-agent.exe", site_url(&relay));
    let res = client()?.get(&url).send().await.map_err(|e| format!("download failed: {e}"))?;
    if !res.status().is_success() {
        return Err(format!("download failed: {}", res.status()));
    }
    let total = res.content_length().unwrap_or(info.size).max(1);
    let mut bytes = Vec::with_capacity(total as usize);
    let mut stream = res.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("download interrupted: {e}"))?;
        bytes.extend_from_slice(&chunk);
        *stage.lock().unwrap() = Stage::Downloading { got: bytes.len() as u64, total };
        ctx.request_repaint();
    }
    *stage.lock().unwrap() = Stage::Installing;
    ctx.request_repaint();
    let hash: String = sha2::Sha256::digest(&bytes).iter().map(|b| format!("{b:02x}")).collect();
    if !hash.eq_ignore_ascii_case(&info.sha256) {
        return Err("the download did not match the server's fingerprint; keeping the current version".into());
    }
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let new = exe.with_extension("new");
    std::fs::write(&new, &bytes).map_err(|e| format!("could not save the update next to the program: {e}"))?;
    let old = old_path(&exe);
    let _ = std::fs::remove_file(&old);
    std::fs::rename(&exe, &old).map_err(|e| {
        let _ = std::fs::remove_file(&new);
        format!("could not replace the program (is the folder read-only?): {e}")
    })?;
    if let Err(e) = std::fs::rename(&new, &exe) {
        // Put the old version back so the program still starts.
        let _ = std::fs::rename(&old, &exe);
        return Err(format!("could not install the update: {e}"));
    }
    Ok(())
}

struct UpdateApp {
    info: Info,
    stage: Arc<Mutex<Stage>>,
    started: Instant,
    ended: Option<Instant>,
    theme_id: String,
}

impl eframe::App for UpdateApp {
    fn clear_color(&self, _v: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let t = theme::get(&self.theme_id);
        theme::style(ctx, &t);
        ctx.request_repaint_after(Duration::from_millis(30));
        let stage = self.stage.lock().unwrap().clone();
        if matches!(stage, Stage::Done | Stage::Failed(_)) && self.ended.is_none() {
            self.ended = Some(Instant::now());
        }
        // Done closes quickly; a failure stays readable for a moment.
        if let Some(e) = self.ended {
            let wait = if matches!(stage, Stage::Done) { 700 } else { 3500 };
            if e.elapsed() > Duration::from_millis(wait) {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
        let close = forms::window_chrome(ctx, &t, "Update", |ui| {
            ui.add_space(6.0);
            ui.label(RichText::new(format!("Updating to {}", self.info.version)).size(20.0).strong().color(t.text));
            let (frac, line, color) = match &stage {
                Stage::Downloading { got, total } => (
                    *got as f32 / *total as f32,
                    format!("Downloading  {:.1} / {:.1} MB", *got as f32 / 1_048_576.0, *total as f32 / 1_048_576.0),
                    t.muted,
                ),
                Stage::Installing => (1.0, "Checking and installing...".to_string(), t.muted),
                Stage::Done => (1.0, "Done. Starting the new version...".to_string(), t.good),
                Stage::Failed(e) => (0.0, format!("{e}. Starting the current version."), t.warn),
            };
            ui.add_space(10.0);
            let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 10.0), egui::Sense::hover());
            ui.painter().rect_filled(r, CornerRadius::same(5), t.track);
            // Indeterminate shimmer until the first bytes arrive.
            let shown = if frac <= 0.0 && !matches!(stage, Stage::Failed(_)) {
                0.15 + 0.1 * (self.started.elapsed().as_secs_f32() * 4.0).sin()
            } else {
                frac
            };
            let mut fill = r;
            fill.set_width(r.width() * shown.clamp(0.0, 1.0));
            ui.painter().rect_filled(fill, CornerRadius::same(5), if matches!(stage, Stage::Failed(_)) { t.warn } else { t.accent });
            ui.add_space(8.0);
            ui.label(RichText::new(line).color(color).size(13.0));
            ui.label(RichText::new(format!("You have {}", protocol::AGENT_VERSION)).color(t.dim).size(12.0));
        });
        if close {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

/// Checks for an update on launch and installs it with a small progress
/// window. Returns true when the new version was started (the caller exits).
pub fn on_launch(relay: &str, theme_id: &str) -> bool {
    cleanup();
    let Ok(exe) = std::env::current_exe() else { return false };
    if dev_build(&exe) {
        return false;
    }
    let Some(info) = check(relay) else { return false };
    tracing::info!("update available: {} -> {}", protocol::AGENT_VERSION, info.version);
    let stage = Arc::new(Mutex::new(Stage::Downloading { got: 0, total: info.size.max(1) }));
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("CYBERSPYDE update")
            .with_inner_size([440.0, 200.0])
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_always_on_top()
            .with_icon(crate::icon()),
        centered: true,
        ..Default::default()
    };
    let (relay_s, info_s, stage_s) = (relay.to_string(), info.clone(), stage.clone());
    let app = UpdateApp { info, stage: stage.clone(), started: Instant::now(), ended: None, theme_id: theme_id.to_string() };
    let res = eframe::run_native(
        "cyberspyde update",
        opts,
        Box::new(move |cc| {
            let ctx = cc.egui_ctx.clone();
            std::thread::spawn(move || {
                let result = match runtime() {
                    Some(rt) => rt.block_on(download_and_install(relay_s, info_s, stage_s.clone(), ctx.clone())),
                    None => Err("could not start the downloader".into()),
                };
                *stage_s.lock().unwrap() = match result {
                    Ok(()) => Stage::Done,
                    Err(e) => {
                        tracing::warn!("update failed: {e}");
                        Stage::Failed(e)
                    }
                };
                ctx.request_repaint();
            });
            Ok(Box::new(app))
        }),
    );
    if let Err(e) = res {
        tracing::warn!("update window failed: {e}");
    }
    if !matches!(*stage.lock().unwrap(), Stage::Done) {
        return false;
    }
    restart()
}

/// Starts this program again (the freshly installed file) with the same
/// options, telling it to wait until this process has exited.
pub fn restart() -> bool {
    let Ok(exe) = std::env::current_exe() else { return false };
    let mut args: Vec<String> = std::env::args().skip(1).filter(|a| a != "--wait-old").collect();
    args.push("--wait-old".into());
    match std::process::Command::new(&exe).args(&args).spawn() {
        Ok(_) => true,
        Err(e) => {
            tracing::warn!("could not start the new version: {e}");
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_order() {
        assert!(newer("0.2.10", "0.2.9"));
        assert!(newer("0.3.0", "0.2.9"));
        assert!(!newer("0.2.3", "0.2.3"));
        assert!(!newer("0.2.2", "0.2.3"));
        assert!(!newer("", "0.2.3"));
    }

    #[test]
    fn dev_builds_never_update() {
        assert!(dev_build(Path::new(r"D:\x\target\release\cyberspyde-agent.exe")));
    }
}
