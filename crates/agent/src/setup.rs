//! First-launch setup: three short steps instead of one long form.
//!
//!   1. You        name, theme, size
//!   2. Fine-tune  layout, side, opacity, what to show, the Dota launch option
//!   3. Team key   key (checked live against the server), server address
//!
//! then a short "you're in" animation and the overlay starts. Opens
//! automatically when there is no team key yet, and with `--setup`.
//! Everything can be changed later in the in-game menu (Numpad *).

use crate::{config::Config, forms, theme, theme::Theme};
use eframe::egui::{self, Color32, CornerRadius, Pos2, Rect, RichText, Sense, Stroke, Vec2};
use futures_util::{SinkExt, StreamExt};
use protocol::{ClientMsg, Role, ServerMsg, PROTOCOL_VERSION};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

/// What people type -> relay URL.
/// - a domain ("dota.cyberspyde.uz") -> wss://domain/ws (public, TLS edge)
/// - an IP, localhost or host:port -> ws://host:4747/ws (LAN or test relay)
/// - full ws:// or wss:// URLs pass through.
pub fn relay_url(input: &str) -> String {
    let s = input.trim().trim_end_matches('/');
    if s.starts_with("ws://") || s.starts_with("wss://") {
        return s.to_string();
    }
    let host = s.strip_prefix("http://").or_else(|| s.strip_prefix("https://")).unwrap_or(s);
    let is_ip = host.parse::<std::net::IpAddr>().is_ok();
    if host.contains(':') {
        format!("ws://{host}/ws")
    } else if is_ip || host == "localhost" {
        format!("ws://{host}:4747/ws")
    } else {
        format!("wss://{host}/ws")
    }
}

/// Host part of a relay URL, for showing as the current server.
pub fn relay_host(url: &str) -> String {
    let s = url.trim_start_matches("ws://").trim_start_matches("wss://");
    let s = s.trim_end_matches("/ws");
    s.strip_suffix(":4747").unwrap_or(s).to_string()
}

/// http(s) address of the team site behind a relay URL.
pub fn site_url(relay: &str) -> String {
    let secure = relay.starts_with("wss://");
    format!("{}://{}", if secure { "https" } else { "http" }, relay.trim_start_matches("ws://").trim_start_matches("wss://").trim_end_matches("/ws"))
}

// ---------------------------------------------------------------- key check

#[derive(Clone, Debug)]
enum Check {
    Idle,
    Busy(Instant),
    Failed(String),
    /// The server accepted the key.
    Ok,
    /// The server could not be reached; the key is unknown. Offer to go on.
    Offline(String),
}

/// Connects as a viewer (so teammates see no one join), sends the key and
/// waits for the server's answer.
async fn check_key(url: String, key: String) -> Check {
    let connect = tokio::time::timeout(Duration::from_secs(6), tokio_tungstenite::connect_async_with_config(url.as_str(), None, true)).await;
    let mut ws = match connect {
        Ok(Ok((ws, _))) => ws,
        Ok(Err(e)) => return Check::Offline(crate::net::friendly(&e.to_string())),
        Err(_) => return Check::Offline("The team server did not answer in time".into()),
    };
    let hello = ClientMsg::Hello { key, name: "setup check".into(), role: Role::Viewer, version: PROTOCOL_VERSION };
    let text = serde_json::to_string(&hello).unwrap_or_default();
    if ws.send(tokio_tungstenite::tungstenite::Message::Text(text.into())).await.is_err() {
        return Check::Offline("Lost the connection to the team server".into());
    }
    let answer = tokio::time::timeout(Duration::from_secs(6), async {
        while let Some(Ok(m)) = ws.next().await {
            if let tokio_tungstenite::tungstenite::Message::Text(t) = m {
                match serde_json::from_str::<ServerMsg>(t.as_str()) {
                    Ok(ServerMsg::Welcome { .. }) => return Check::Ok,
                    Ok(ServerMsg::Error { message }) if message.contains("key") => {
                        return Check::Failed("That key is not right. Check it with your host.".into())
                    }
                    Ok(ServerMsg::Error { message }) => return Check::Failed(message),
                    _ => {}
                }
            }
        }
        Check::Offline("The team server closed the connection".into())
    })
    .await
    .unwrap_or_else(|_| Check::Offline("The team server did not answer in time".into()));
    let _ = ws.close(None).await;
    answer
}

fn spawn_check(url: String, key: String, out: Arc<Mutex<Check>>, ctx: egui::Context) {
    std::thread::spawn(move || {
        let res = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
            Ok(rt) => rt.block_on(check_key(url, key)),
            Err(e) => Check::Offline(e.to_string()),
        };
        tracing::info!("key check finished: {res:?}");
        *out.lock().unwrap() = res;
        ctx.request_repaint();
    });
}

// ---------------------------------------------------------------- window

const STEPS: [&str; 3] = ["You", "Fine-tune", "Team key"];
/// Step slide/fade duration.
const SLIDE: f32 = 0.28;

fn ease(x: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    1.0 - (1.0 - x).powi(3)
}

struct SetupApp {
    cfg: Config,
    out: Arc<Mutex<Option<Config>>>,
    /// 0..=2 are the steps, 3 is the finish screen.
    step: usize,
    /// When the step changed and in which direction (+1 forward, -1 back).
    changed_at: Instant,
    dir: f32,
    show_key: bool,
    show_server: bool,
    host_edit: String,
    error: Option<String>,
    check: Arc<Mutex<Check>>,
    done_at: Option<Instant>,
    ctx: Option<egui::Context>,
}

impl SetupApp {
    fn go(&mut self, step: usize) {
        self.dir = if step > self.step { 1.0 } else { -1.0 };
        self.step = step;
        self.changed_at = Instant::now();
        self.error = None;
    }

    fn finish(&mut self) {
        if self.show_server && !self.host_edit.trim().is_empty() {
            self.cfg.relay = relay_url(&self.host_edit);
        }
        self.cfg.name = self.cfg.name.trim().chars().take(24).collect();
        self.cfg.key = self.cfg.key.trim().to_string();
        *self.out.lock().unwrap() = Some(self.cfg.clone());
        self.go(3);
        self.done_at = Some(Instant::now());
    }

    /// Number circles joined by a line that fills as you go.
    fn progress(&self, ui: &mut egui::Ui, t: &Theme) {
        let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 52.0), Sense::hover());
        let p = ui.painter();
        let n = STEPS.len();
        let xs: Vec<f32> = (0..n).map(|i| egui::lerp(rect.left() + 34.0..=rect.right() - 34.0, i as f32 / (n - 1) as f32)).collect();
        let y = rect.top() + 16.0;
        let shown = ui.ctx().animate_value_with_time(egui::Id::new("progress"), self.step.min(n - 1) as f32, SLIDE);
        p.line_segment([Pos2::new(xs[0], y), Pos2::new(xs[n - 1], y)], Stroke::new(3.0_f32, t.track));
        let fill_x = egui::lerp(xs[0]..=xs[n - 1], shown / (n - 1) as f32);
        p.line_segment([Pos2::new(xs[0], y), Pos2::new(fill_x, y)], Stroke::new(3.0_f32, t.accent));
        for (i, label) in STEPS.iter().enumerate() {
            let c = Pos2::new(xs[i], y);
            let done = i < self.step;
            let current = i == self.step;
            let fill = if done || current { t.accent } else { t.bottom.to_opaque() };
            p.circle_filled(c, 13.0, fill);
            p.circle_stroke(c, 13.0, Stroke::new(1.5_f32, if done || current { t.accent } else { t.border }));
            let ink = if t.light { Color32::WHITE } else { Color32::from_rgb(4, 16, 20) };
            if done {
                draw_check(p, c, 6.0, 1.0, ink, 2.2);
            } else {
                p.text(c, egui::Align2::CENTER_CENTER, (i + 1).to_string(), egui::FontId::proportional(13.0), if current { ink } else { t.muted });
            }
            p.text(Pos2::new(c.x, y + 24.0), egui::Align2::CENTER_TOP, *label, egui::FontId::proportional(12.0), if current { t.text } else { t.muted });
        }
    }

    fn step_you(&mut self, ui: &mut egui::Ui, t: &Theme) {
        ui.label(RichText::new("Welcome").size(26.0).strong().color(t.text));
        forms::hint(ui, t, "Your teammates' health, ultimates and TPs, right on your screen. Let's set it up.");
        ui.add_space(8.0);
        forms::card(ui, t, |ui| {
            ui.label(RichText::new("Your name").color(t.muted).size(12.0));
            let r = ui.add(
                egui::TextEdit::singleline(&mut self.cfg.name)
                    .hint_text(RichText::new("as your team knows you").color(t.dim))
                    .char_limit(24)
                    .font(egui::FontId::proportional(17.0))
                    .margin(egui::Margin::symmetric(12, 9))
                    .desired_width(f32::INFINITY),
            );
            if self.step == 0 && ui.ctx().input(|i| i.time) < 1.0 {
                r.request_focus();
            }
            if r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.next();
            }
        });
        ui.add_space(6.0);
        forms::card(ui, t, |ui| {
            forms::look_basic(ui, t, &mut self.cfg);
        });
    }

    fn step_more(&mut self, ui: &mut egui::Ui, t: &Theme, screen_w: f32) {
        ui.label(RichText::new("Make it yours").size(26.0).strong().color(t.text));
        forms::hint(ui, t, "All optional. The menu in game (Numpad *) changes any of this later.");
        ui.add_space(8.0);
        ui.columns(2, |cols| {
            forms::card(&mut cols[0], t, |ui| {
                forms::look_more(ui, t, &mut self.cfg, screen_w);
            });
            forms::card(&mut cols[1], t, |ui| {
                ui.label(RichText::new("Show").color(t.muted).size(12.0));
                forms::toggle(ui, t, &mut self.cfg.overlay.show_summary, "Team summary");
                forms::toggle(ui, t, &mut self.cfg.overlay.show_build, "Next-item help");
                forms::toggle(ui, t, &mut self.cfg.overlay.show_tips, "Game-plan tips");
                forms::toggle(ui, t, &mut self.cfg.overlay.show_radar, "Mini radar");
                forms::toggle(ui, t, &mut self.cfg.overlay.show_notes, "Tips from the site");
            });
        });
        ui.add_space(6.0);
        forms::card(ui, t, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Dota needs one launch option:").color(t.text));
                ui.label(RichText::new("-gamestateintegration").monospace().size(14.0).color(t.accent));
            });
            forms::hint(ui, t, "Steam > Dota 2 > Properties > Launch options. And use Borderless window in Dota's video settings.");
        });
    }

    fn step_key(&mut self, ui: &mut egui::Ui, t: &Theme) {
        ui.label(RichText::new("Join your team").size(26.0).strong().color(t.text));
        forms::hint(ui, t, "Your host has the team key. It is checked with the server before you start.");
        ui.add_space(8.0);
        let busy = matches!(*self.check.lock().unwrap(), Check::Busy(_));
        forms::card(ui, t, |ui| {
            ui.label(RichText::new("Team key").color(t.muted).size(12.0));
            ui.horizontal(|ui| {
                let w = ui.available_width() - 84.0;
                let r = ui.add_enabled(
                    !busy,
                    egui::TextEdit::singleline(&mut self.cfg.key)
                        .password(!self.show_key)
                        .hint_text(RichText::new("paste the key here").color(t.dim))
                        .font(egui::FontId::proportional(17.0))
                        .margin(egui::Margin::symmetric(12, 9))
                        .desired_width(w),
                );
                if r.changed() {
                    *self.check.lock().unwrap() = Check::Idle;
                }
                if r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.next();
                }
                if ui.button(if self.show_key { "Hide" } else { "Show" }).clicked() {
                    self.show_key = !self.show_key;
                }
            });
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                forms::hint(ui, t, &format!("Server: {}", relay_host(&self.cfg.relay)));
                if ui.small_button(if self.show_server { "Done" } else { "Change" }).clicked() {
                    if self.show_server && !self.host_edit.trim().is_empty() {
                        self.cfg.relay = relay_url(&self.host_edit);
                        *self.check.lock().unwrap() = Check::Idle;
                    }
                    self.show_server = !self.show_server;
                    self.host_edit = relay_host(&self.cfg.relay);
                }
            });
            if self.show_server {
                ui.add(egui::TextEdit::singleline(&mut self.host_edit).hint_text(RichText::new("dota.cyberspyde.uz").color(t.dim)).margin(egui::Margin::symmetric(10, 7)).desired_width(f32::INFINITY));
            }
        });
        ui.add_space(8.0);
        let state = self.check.lock().unwrap().clone();
        match state {
            Check::Busy(since) => {
                ui.horizontal(|ui| {
                    spinner(ui, t, since);
                    ui.label(RichText::new("Checking the key with the team server...").color(t.muted));
                });
            }
            Check::Failed(msg) => {
                ui.label(RichText::new(msg).color(t.bad).strong());
            }
            Check::Offline(msg) => {
                ui.label(RichText::new(format!("{msg}. The key could not be checked.")).color(t.warn));
                if ui.button("Start anyway").clicked() {
                    self.finish();
                }
            }
            Check::Ok | Check::Idle => {}
        }
    }

    fn step_done(&mut self, ui: &mut egui::Ui, t: &Theme) {
        let k = self.done_at.map(|d| d.elapsed().as_secs_f32()).unwrap_or(0.0);
        let area = ui.available_rect_before_wrap();
        let c = Pos2::new(area.center().x, area.top() + area.height() * 0.38);
        let p = ui.painter();
        // Ring draws itself, then the tick, then a soft pulse outwards.
        let ring = ease(k / 0.55);
        let r = 44.0;
        p.circle_filled(c, r, t.accent.gamma_multiply(0.12 * ring));
        arc(p, c, r, ring, Stroke::new(4.0_f32, t.accent));
        draw_check(p, c, 20.0, ease((k - 0.45) / 0.35), t.accent, 5.0);
        let pulse = ((k - 0.8) / 0.7).clamp(0.0, 1.0);
        if pulse > 0.0 && pulse < 1.0 {
            p.circle_stroke(c, r + 30.0 * ease(pulse), Stroke::new(2.0_f32, t.accent.gamma_multiply(1.0 - pulse)));
        }
        let text_k = ease((k - 0.6) / 0.4);
        let name = self.cfg.name.trim().to_string();
        p.text(c + Vec2::new(0.0, r + 34.0 - 8.0 * (1.0 - text_k)), egui::Align2::CENTER_CENTER, format!("You're in, {name}"), egui::FontId::proportional(24.0), t.text.gamma_multiply(text_k));
        p.text(c + Vec2::new(0.0, r + 66.0), egui::Align2::CENTER_CENTER, "Starting the overlay. Numpad * opens the menu.", egui::FontId::proportional(14.0), t.muted.gamma_multiply(text_k));
    }

    /// Validates the current step and moves on.
    fn next(&mut self) {
        match self.step {
            0 => {
                if self.cfg.name.trim().is_empty() {
                    self.error = Some("Type the name your team knows you by.".into());
                } else {
                    self.go(1);
                }
            }
            1 => self.go(2),
            2 => {
                if self.cfg.key.trim().is_empty() {
                    *self.check.lock().unwrap() = Check::Failed("Paste the team key from your host.".into());
                    return;
                }
                if matches!(*self.check.lock().unwrap(), Check::Busy(_)) {
                    return;
                }
                if self.show_server && !self.host_edit.trim().is_empty() {
                    self.cfg.relay = relay_url(&self.host_edit);
                }
                *self.check.lock().unwrap() = Check::Busy(Instant::now());
                if let Some(ctx) = &self.ctx {
                    spawn_check(self.cfg.relay.clone(), self.cfg.key.trim().to_string(), self.check.clone(), ctx.clone());
                }
            }
            _ => {}
        }
    }
}

// The context is needed to wake the window when the background check ends.
impl SetupApp {
    #[allow(clippy::new_ret_no_self)]
    fn with_ctx(mut self, ctx: egui::Context) -> Self {
        self.ctx = Some(ctx);
        self
    }
}

fn spinner(ui: &mut egui::Ui, t: &Theme, since: Instant) {
    let (r, _) = ui.allocate_exact_size(Vec2::splat(18.0), Sense::hover());
    let a = since.elapsed().as_secs_f32() * 5.0;
    let p = ui.painter();
    p.circle_stroke(r.center(), 7.0, Stroke::new(2.5_f32, t.track));
    let pts: Vec<Pos2> = (0..=16).map(|i| {
        let ang = a + i as f32 / 16.0 * 2.0;
        r.center() + Vec2::angled(ang) * 7.0
    }).collect();
    p.add(egui::Shape::line(pts, Stroke::new(2.5_f32, t.accent)));
    ui.ctx().request_repaint();
}

/// Arc from the top, clockwise, `frac` of a full circle.
fn arc(p: &egui::Painter, c: Pos2, r: f32, frac: f32, stroke: Stroke) {
    if frac <= 0.0 {
        return;
    }
    let n = (64.0 * frac).ceil().max(2.0) as usize;
    let pts: Vec<Pos2> = (0..=n)
        .map(|i| {
            let ang = -std::f32::consts::FRAC_PI_2 + i as f32 / n as f32 * frac * std::f32::consts::TAU;
            c + Vec2::angled(ang) * r
        })
        .collect();
    p.add(egui::Shape::line(pts, stroke));
}

/// A tick mark drawn up to `k` (0..1) of its length.
fn draw_check(p: &egui::Painter, c: Pos2, s: f32, k: f32, color: Color32, width: f32) {
    if k <= 0.0 {
        return;
    }
    let a = c + Vec2::new(-s * 0.9, 0.0);
    let b = c + Vec2::new(-s * 0.25, s * 0.65);
    let e = c + Vec2::new(s, -s * 0.7);
    let l1 = (b - a).length();
    let l2 = (e - b).length();
    let total = (l1 + l2) * k.min(1.0);
    let mut pts = vec![a];
    if total <= l1 {
        pts.push(a + (b - a) * (total / l1));
    } else {
        pts.push(b);
        pts.push(b + (e - b) * ((total - l1) / l2));
    }
    p.add(egui::Shape::line(pts, Stroke::new(width, color)));
}

impl eframe::App for SetupApp {
    fn clear_color(&self, _v: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.ctx.is_none() {
            self.ctx = Some(ctx.clone());
        }
        // Debug aid for screenshots: CYBERSPYDE_SETUP_AUTOSTART=1 presses Start once.
        if self.step == 2 && ctx.input(|i| i.time) > 0.5 && std::env::var("CYBERSPYDE_SETUP_AUTOSTART").is_ok_and(|v| v == "1") && matches!(*self.check.lock().unwrap(), Check::Idle) {
            self.next();
        }
        let t = theme::get(&self.cfg.overlay.theme);
        theme::style(ctx, &t);
        let screen_w = ctx.input(|i| i.viewport().monitor_size).map(|s| s.x).unwrap_or_else(forms::screen_width);

        // The key check came back fine: celebrate, then close.
        if self.step == 2 && matches!(*self.check.lock().unwrap(), Check::Ok) {
            self.finish();
        }
        if let Some(d) = self.done_at {
            if d.elapsed() > Duration::from_millis(2100) {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            ctx.request_repaint();
        }

        let since = self.changed_at.elapsed().as_secs_f32();
        let k = ease(since / SLIDE);
        if k < 1.0 {
            ctx.request_repaint();
        }
        // Fade the whole window out over the last moments of the finish screen.
        let fade_out = self.done_at.map(|d| 1.0 - ((d.elapsed().as_secs_f32() - 1.75) / 0.35).clamp(0.0, 1.0)).unwrap_or(1.0);

        let mut close = false;
        let step = self.step;
        close |= forms::window_chrome(ctx, &t, "Team Link setup", |ui| {
            ui.set_opacity(fade_out);
            if step < 3 {
                self.progress(ui, &t);
                ui.add_space(6.0);
            }
            // Current step slides in from the side it came from.
            let full = ui.available_rect_before_wrap();
            let footer_h = if step < 3 { 52.0 } else { 0.0 };
            let body = Rect::from_min_max(full.min, Pos2::new(full.max.x, full.max.y - footer_h));
            let offset = Vec2::new(self.dir * 46.0 * (1.0 - k), 0.0);
            let mut child = ui.new_child(egui::UiBuilder::new().max_rect(body.translate(offset)));
            child.set_clip_rect(body.expand2(Vec2::new(18.0, 4.0)));
            child.multiply_opacity(k);
            match step {
                0 => self.step_you(&mut child, &t),
                1 => self.step_more(&mut child, &t, screen_w),
                2 => self.step_key(&mut child, &t),
                _ => self.step_done(&mut child, &t),
            }
            if step < 3 {
                let foot = Rect::from_min_max(Pos2::new(full.min.x, full.max.y - footer_h + 8.0), full.max);
                let mut f = ui.new_child(egui::UiBuilder::new().max_rect(foot).layout(egui::Layout::left_to_right(egui::Align::Center)));
                if step > 0 && f.add(egui::Button::new("Back").min_size(Vec2::new(90.0, 36.0)).corner_radius(9.0)).clicked() {
                    self.go(step - 1);
                }
                if let Some(e) = &self.error {
                    f.label(RichText::new(e).color(t.bad));
                }
                f.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let busy = matches!(*self.check.lock().unwrap(), Check::Busy(_));
                    let label = match step {
                        2 if busy => "Checking...",
                        2 => "Start",
                        _ => "Next",
                    };
                    let ink = if t.light { Color32::WHITE } else { Color32::from_rgb(4, 16, 20) };
                    let b = egui::Button::new(RichText::new(label).strong().color(ink).size(15.0))
                        .fill(t.accent)
                        .corner_radius(CornerRadius::same(9))
                        .min_size(Vec2::new(140.0, 38.0));
                    if ui.add_enabled(!busy, b).clicked() {
                        self.next();
                    }
                    ui.label(RichText::new(format!("Step {} of 3", step + 1)).color(t.dim).size(12.0));
                });
            }
        });
        if close {
            // Closing during setup keeps whatever was finished (nothing, before step 3).
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

/// Shows the setup window. Returns the new settings, or None if it was closed
/// before the last step.
pub fn run(cfg: &Config) -> Option<Config> {
    let out = Arc::new(Mutex::new(None));
    let (w, h) = (600.0, 640.0f32.min(forms::screen_height() - 60.0));
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("CYBERSPYDE Team Link setup")
            .with_inner_size([w, h])
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_icon(crate::icon()),
        centered: true,
        ..Default::default()
    };
    let start_step = std::env::var("CYBERSPYDE_SETUP_STEP").ok().and_then(|s| s.parse().ok()).unwrap_or(0usize).min(3);
    let app = SetupApp {
        cfg: cfg.clone(),
        out: out.clone(),
        step: start_step,
        changed_at: Instant::now(),
        dir: 1.0,
        show_key: false,
        show_server: false,
        host_edit: relay_host(&cfg.relay),
        error: None,
        check: Arc::new(Mutex::new(Check::Idle)),
        done_at: (start_step == 3).then(Instant::now),
        ctx: None,
    };
    let result_slot = out.clone();
    if let Err(e) = eframe::run_native("cyberspyde setup", opts, Box::new(move |cc| Ok(Box::new(app.with_ctx(cc.egui_ctx.clone()))))) {
        tracing::error!("setup window failed: {e}");
    }
    let result = result_slot.lock().unwrap().take();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relay_addresses() {
        assert_eq!(relay_url("203.0.113.10"), "ws://203.0.113.10:4747/ws");
        assert_eq!(relay_url(" team.example.com:9000 "), "ws://team.example.com:9000/ws");
        assert_eq!(relay_url("http://1.2.3.4/"), "ws://1.2.3.4:4747/ws");
        assert_eq!(relay_url("wss://relay.example.com/ws"), "wss://relay.example.com/ws");
        assert_eq!(relay_host("ws://203.0.113.10:4747/ws"), "203.0.113.10");
        assert_eq!(relay_host("ws://h:9000/ws"), "h:9000");
        assert_eq!(relay_url("dota.cyberspyde.uz"), "wss://dota.cyberspyde.uz/ws");
        assert_eq!(relay_url("localhost"), "ws://localhost:4747/ws");
        assert_eq!(relay_host("wss://dota.cyberspyde.uz/ws"), "dota.cyberspyde.uz");
        assert_eq!(site_url("wss://dota.cyberspyde.uz/ws"), "https://dota.cyberspyde.uz");
        assert_eq!(site_url("ws://1.2.3.4:4747/ws"), "http://1.2.3.4:4747");
    }

    #[test]
    fn check_mark_and_ease_are_bounded() {
        assert_eq!(ease(-1.0), 0.0);
        assert_eq!(ease(2.0), 1.0);
        assert!(ease(0.5) > 0.5, "ease-out starts fast");
    }
}
