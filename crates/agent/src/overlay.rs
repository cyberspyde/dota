//! Transparent, click-through, always-on-top overlay. Works with Dota in
//! borderless-window mode (exclusive fullscreen draws over every window).

use crate::{
    builds::{advise, BuildDb},
    config::{self, Config},
    team::{Mate, Notifier, Severity, SharedTeam, Status, MAP_HALF},
};
use eframe::egui::{self, Color32, CornerRadius, Pos2, Rect, RichText, Sense, Stroke, Vec2};
use global_hotkey::{
    hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use protocol::{hero_display, is_key_item, item_short, CalloutKind, ClientMsg};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::mpsc;

const W: f32 = 330.0;
const H: f32 = 700.0;

const RED: Color32 = Color32::from_rgb(248, 81, 73);
const AMBER: Color32 = Color32::from_rgb(240, 180, 41);
const GREEN: Color32 = Color32::from_rgb(63, 185, 80);
const BLUE: Color32 = Color32::from_rgb(76, 141, 255);
const MUTED: Color32 = Color32::from_rgb(150, 158, 172);
const TEXT: Color32 = Color32::from_rgb(232, 235, 240);
const DIM: Color32 = Color32::from_rgb(90, 96, 108);
const TITLE: &str = "cyberspyde overlay";

/// Click-through makes winit add WS_EX_LAYERED to the window, but it never
/// calls SetLayeredWindowAttributes, and Windows does not draw a layered
/// window that has none: the overlay renders but never reaches the screen.
/// Setting full window opacity makes it visible again; per-pixel
/// transparency still comes from the transparent GL surface.
#[cfg(windows)]
fn ensure_layered_visible() {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        FindWindowW, GetLayeredWindowAttributes, GetWindowLongPtrW, SetLayeredWindowAttributes, GWL_EXSTYLE,
        LWA_ALPHA, WS_EX_LAYERED,
    };
    thread_local! {
        static TITLE_W: Vec<u16> = TITLE.encode_utf16().chain(std::iter::once(0)).collect();
    }
    TITLE_W.with(|t| unsafe {
        let hwnd = FindWindowW(std::ptr::null(), t.as_ptr());
        if hwnd.is_null() {
            return;
        }
        if GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32 & WS_EX_LAYERED == 0 {
            return;
        }
        let (mut key, mut alpha, mut flags) = (0u32, 0u8, 0u32);
        if GetLayeredWindowAttributes(hwnd, &mut key, &mut alpha, &mut flags) == 0 {
            SetLayeredWindowAttributes(hwnd, 0, 255, LWA_ALPHA);
        }
    });
}

#[cfg(not(windows))]
fn ensure_layered_visible() {}

#[derive(Clone, Copy)]
enum Action {
    Callout(CalloutKind),
    Edit,
    Toggle,
}

pub struct OverlayApp {
    cfg: Config,
    cfg_path: PathBuf,
    team: SharedTeam,
    out: mpsc::Sender<ClientMsg>,
    db: Arc<BuildDb>,
    _hk: Option<GlobalHotKeyManager>,
    hk_map: HashMap<u32, Action>,
    hk_errors: Vec<String>,
    edit: bool,
    hidden: bool,
    flash: Option<(String, Instant)>,
}

pub fn run(
    cfg: Config,
    cfg_path: PathBuf,
    team: SharedTeam,
    notify: Arc<Notifier>,
    out: mpsc::Sender<ClientMsg>,
    db: Arc<BuildDb>,
) -> eframe::Result<()> {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(TITLE)
            .with_inner_size([W * cfg.overlay.scale, H * cfg.overlay.scale])
            .with_position([cfg.overlay.x, cfg.overlay.y])
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_mouse_passthrough(true)
            .with_taskbar(false)
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "cyberspyde overlay",
        opts,
        Box::new(move |cc| {
            notify.set(cc.egui_ctx.clone());
            cc.egui_ctx.set_zoom_factor(cfg.overlay.scale);
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(OverlayApp::new(cfg, cfg_path, team, out, db)))
        }),
    )
}

impl OverlayApp {
    fn new(cfg: Config, cfg_path: PathBuf, team: SharedTeam, out: mpsc::Sender<ClientMsg>, db: Arc<BuildDb>) -> Self {
        let mut hk_map = HashMap::new();
        let mut hk_errors = Vec::new();
        // Must be created on the UI thread: it relies on that thread's message loop.
        let mgr = match GlobalHotKeyManager::new() {
            Ok(m) => Some(m),
            Err(e) => {
                hk_errors.push(format!("hotkeys unavailable: {e}"));
                None
            }
        };
        if let Some(mgr) = &mgr {
            let h = &cfg.hotkeys;
            let binds = [
                (&h.help, Action::Callout(CalloutKind::Help)),
                (&h.group, Action::Callout(CalloutKind::Group)),
                (&h.retreat, Action::Callout(CalloutKind::Retreat)),
                (&h.careful, Action::Callout(CalloutKind::Careful)),
                (&h.on_my_way, Action::Callout(CalloutKind::OnMyWay)),
                (&h.roshan, Action::Callout(CalloutKind::Roshan)),
                (&h.edit_mode, Action::Edit),
                (&h.toggle, Action::Toggle),
            ];
            for (spec, action) in binds {
                if spec.trim().is_empty() {
                    continue;
                }
                match spec.parse::<HotKey>() {
                    Ok(hk) => match mgr.register(hk) {
                        Ok(()) => {
                            hk_map.insert(hk.id(), action);
                        }
                        Err(e) => hk_errors.push(format!("{spec}: {e}")),
                    },
                    Err(e) => hk_errors.push(format!("{spec}: {e}")),
                }
            }
        }
        for e in &hk_errors {
            tracing::warn!("hotkey: {e}");
        }
        OverlayApp {
            cfg,
            cfg_path,
            team,
            out,
            db,
            _hk: mgr,
            hk_map,
            hk_errors,
            edit: false,
            hidden: false,
            flash: None,
        }
    }

    fn poll_hotkeys(&mut self, ctx: &egui::Context) {
        while let Ok(ev) = GlobalHotKeyEvent::receiver().try_recv() {
            if ev.state != HotKeyState::Pressed {
                continue;
            }
            let Some(action) = self.hk_map.get(&ev.id).copied() else { continue };
            match action {
                Action::Callout(kind) => {
                    let (x, y) = {
                        let t = self.team.lock().unwrap();
                        t.my_state().map(|s| (s.x, s.y)).unwrap_or((0.0, 0.0))
                    };
                    let sent = self.out.try_send(ClientMsg::Callout { kind, x, y }).is_ok();
                    self.flash = Some((
                        if sent { format!("sent: {}", kind.label()) } else { "callout not sent".into() },
                        Instant::now(),
                    ));
                }
                Action::Toggle => self.hidden = !self.hidden,
                Action::Edit => {
                    self.edit = !self.edit;
                    ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(!self.edit));
                    if !self.edit {
                        self.save_position(ctx);
                    }
                }
            }
        }
    }

    fn save_position(&mut self, ctx: &egui::Context) {
        if let Some(r) = ctx.input(|i| i.viewport().outer_rect) {
            // outer_rect is in zoomed UI points; the config stores logical pixels.
            let z = ctx.zoom_factor();
            self.cfg.overlay.x = r.min.x * z;
            self.cfg.overlay.y = r.min.y * z;
            match config::save(&self.cfg_path, &self.cfg) {
                Ok(()) => self.flash = Some(("position saved".into(), Instant::now())),
                Err(e) => self.flash = Some((format!("save failed: {e}"), Instant::now())),
            }
        }
    }
}

impl eframe::App for OverlayApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Cheap (two syscalls) and self-healing: winit re-applies the
        // layered style whenever click-through is toggled.
        ensure_layered_visible();
        self.poll_hotkeys(ctx);
        // Timers (respawn, cooldowns) tick between data updates; data updates
        // themselves repaint immediately through the Notifier.
        ctx.request_repaint_after(Duration::from_millis(200));

        let now = Instant::now();
        let (alerts, mates, me, connected, err, rtt, last_gsi, callouts, notes) = {
            let mut t = self.team.lock().unwrap();
            t.prune(now);
            (
                t.alerts(now),
                t.mates.values().cloned().collect::<Vec<_>>(),
                t.me.clone(),
                t.connected,
                t.last_error.clone(),
                t.rtt_ms,
                t.last_gsi,
                t.callouts.iter().cloned().collect::<Vec<_>>(),
                t.notes.iter().cloned().collect::<Vec<_>>(),
            )
        };

        egui::CentralPanel::default().frame(egui::Frame::NONE).show(ctx, |ui| {
            if self.edit {
                let r = ui.interact(ctx.screen_rect(), egui::Id::new("drag"), Sense::drag());
                if r.drag_started() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
            }
            if self.hidden && !self.edit {
                return;
            }
            let alpha = (self.cfg.overlay.opacity.clamp(0.0, 1.0) * 255.0) as u8;
            let mut frame = egui::Frame::new()
                .fill(Color32::from_rgba_unmultiplied(12, 14, 19, alpha))
                .corner_radius(8.0)
                .inner_margin(10.0);
            if self.edit {
                frame = frame.stroke(Stroke::new(2.0_f32, AMBER));
            }
            frame.show(ui, |ui| {
                ui.set_width(W - 20.0);
                ui.spacing_mut().item_spacing = Vec2::new(6.0, 3.0);

                // Header
                ui.horizontal(|ui| {
                    ui.label(RichText::new("TEAM").strong().color(TEXT).size(13.0));
                    if connected {
                        dot(ui, GREEN);
                        let ms = rtt.map(|r| format!("{r:.0} ms")).unwrap_or_else(|| "-- ms".into());
                        ui.label(RichText::new(format!("relay {ms}")).color(MUTED).size(11.0));
                        ui.label(RichText::new(format!("{} online", mates.len() + 1)).color(MUTED).size(11.0));
                    } else {
                        dot(ui, RED);
                        ui.label(RichText::new("relay offline").color(RED).size(11.0));
                    }
                });
                if let Some(e) = err.as_ref().filter(|_| !connected) {
                    ui.label(RichText::new(e).color(RED).size(10.5));
                }
                if last_gsi.is_none_or(|t| now.duration_since(t) > Duration::from_secs(20)) {
                    ui.label(
                        RichText::new("No game data yet. Restart Dota with launch option -gamestateintegration")
                            .color(AMBER)
                            .size(10.5),
                    );
                }
                if self.edit {
                    ui.label(RichText::new("MOVE MODE: drag to move, press the edit key to lock").color(AMBER).size(11.0));
                }
                for e in self.hk_errors.iter().take(2) {
                    ui.label(RichText::new(format!("hotkey {e}")).color(DIM).size(10.0));
                }
                if let Some((msg, at)) = &self.flash {
                    if now.duration_since(*at) < Duration::from_secs(2) {
                        ui.label(RichText::new(msg).color(GREEN).size(11.0));
                    }
                }

                // Alerts
                if !alerts.is_empty() {
                    ui.add_space(4.0);
                    for a in alerts.iter().take(5) {
                        let (c, size) = match a.severity {
                            Severity::Danger => (RED, 13.5),
                            Severity::Warn => (AMBER, 12.0),
                        };
                        ui.label(RichText::new(&a.text).color(c).size(size).strong());
                    }
                }

                // Team rows
                ui.add_space(6.0);
                if let Some(me) = &me {
                    mate_row(ui, me, now, true);
                }
                for m in &mates {
                    mate_row(ui, m, now, false);
                }
                if mates.is_empty() {
                    ui.label(RichText::new("No teammates connected yet").color(DIM).size(11.0));
                }

                // Radar
                if self.cfg.overlay.show_radar {
                    ui.add_space(6.0);
                    radar(ui, me.as_ref(), &mates, &callouts, now);
                }

                // Build helper
                if self.cfg.overlay.show_build {
                    if let Some(st) = me.as_ref().and_then(|m| m.state.as_ref()) {
                        if let Some(adv) = advise(&self.db, st, &self.cfg.build_mood) {
                            ui.add_space(6.0);
                            ui.separator();
                            if let Some(h) = &adv.skill_hint {
                                ui.label(RichText::new(h).color(AMBER).strong().size(12.5));
                            }
                            match &adv.next {
                                Some(next) => {
                                    ui.horizontal_wrapped(|ui| {
                                        ui.label(RichText::new("NEXT").color(MUTED).size(11.0));
                                        ui.label(RichText::new(&next.name).color(TEXT).strong().size(13.0));
                                        if adv.gold_needed == 0 {
                                            ui.label(RichText::new("BUY NOW").color(GREEN).strong().size(11.5));
                                        } else {
                                            ui.label(
                                                RichText::new(format!("{}g, need {}", next.cost, adv.gold_needed))
                                                    .color(MUTED)
                                                    .size(11.0),
                                            );
                                        }
                                    });
                                    if !next.description.is_empty() {
                                        ui.label(RichText::new(&next.description).color(MUTED).size(10.5));
                                    }
                                    if !adv.after.is_empty() {
                                        ui.label(
                                            RichText::new(format!("then: {}", adv.after.join(", "))).color(MUTED).size(10.5),
                                        );
                                    }
                                }
                                None if adv.author.is_empty() => {
                                    ui.label(
                                        RichText::new(format!(
                                            "No build saved for {} yet. Add one to builds.override.json.",
                                            hero_display(&st.hero)
                                        ))
                                        .color(DIM)
                                        .size(10.5),
                                    );
                                }
                                None => {
                                    ui.label(RichText::new("Build complete - situational items now").color(GREEN).size(11.0));
                                }
                            }
                            if !adv.skills.is_empty() {
                                // Highlight the pick for the current level (T = talent works too).
                                let next_idx = (st.level as usize).saturating_sub(1);
                                let shown: Vec<String> = adv
                                    .skills
                                    .iter()
                                    .enumerate()
                                    .map(|(i, s)| if i == next_idx { format!("[{s}]") } else { s.clone() })
                                    .collect();
                                ui.label(RichText::new(format!("Skills: {}", shown.join(" "))).color(TEXT).size(11.0));
                            }
                            if !adv.phase_tip.is_empty() {
                                ui.label(RichText::new(&adv.phase_tip).color(MUTED).size(10.5));
                            }
                            if let Some(t) = &adv.tip {
                                ui.label(RichText::new(format!("Tip: {t}")).color(DIM).size(10.5));
                            }
                            if !adv.author.is_empty() {
                                ui.label(RichText::new(format!("build by {}", adv.author)).color(DIM).size(9.5));
                            }
                        }
                    }
                }

                // Notes from the team board
                if !notes.is_empty() {
                    ui.add_space(4.0);
                    ui.separator();
                    for n in notes.iter().rev().take(3) {
                        let c = if n.to_me { AMBER } else { TEXT };
                        ui.label(RichText::new(format!("{}: {}", n.from, n.text)).color(c).size(11.5));
                    }
                }
            });
        });
    }
}

fn dot(ui: &mut egui::Ui, c: Color32) {
    let (r, _) = ui.allocate_exact_size(Vec2::splat(10.0), Sense::hover());
    ui.painter().circle_filled(r.center(), 4.0, c);
}

fn status_color(s: Status) -> Color32 {
    match s {
        Status::UnderAttack => RED,
        Status::Low => AMBER,
        Status::Ok => GREEN,
        Status::Dead => DIM,
        Status::Stale | Status::InMenu | Status::NoData => DIM,
    }
}

fn bar(ui: &mut egui::Ui, frac: f32, color: Color32, height: f32) {
    let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), height), Sense::hover());
    let p = ui.painter();
    p.rect_filled(r, CornerRadius::same(2), Color32::from_rgb(30, 33, 40));
    let mut fill = r;
    fill.set_width(r.width() * frac.clamp(0.0, 1.0));
    p.rect_filled(fill, CornerRadius::same(2), color);
}

fn mate_row(ui: &mut egui::Ui, m: &Mate, now: Instant, is_me: bool) {
    let status = m.status(now);
    ui.horizontal(|ui| {
        dot(ui, status_color(status));
        let name = if is_me { format!("{} (you)", m.name) } else { m.name.clone() };
        ui.label(RichText::new(name).strong().color(TEXT).size(12.0));
        if let Some(st) = m.state.as_ref().filter(|s| s.in_match()) {
            ui.label(RichText::new(format!("{} {}", hero_display(&st.hero), st.level)).color(MUTED).size(11.0));
        }
    });
    let Some(st) = m.state.as_ref() else {
        ui.label(RichText::new("waiting for game data").color(DIM).size(10.5));
        return;
    };
    if !st.in_match() {
        ui.label(RichText::new("not in a match").color(DIM).size(10.5));
        return;
    }
    if status == Status::Stale {
        ui.label(
            RichText::new(format!("no update for {}s", now.duration_since(m.updated).as_secs())).color(DIM).size(10.5),
        );
    }
    if !st.alive {
        let bb = if st.buyback_cooldown > 0 {
            format!("BB cd {}s", st.buyback_cooldown)
        } else if st.gold >= st.buyback_cost {
            "BB ready".into()
        } else {
            "no BB gold".into()
        };
        ui.label(RichText::new(format!("DEAD {}s - {}", st.respawn_seconds, bb)).color(RED).size(11.0));
    } else {
        let hp_color = if status == Status::UnderAttack { RED } else if st.hp_frac() < 0.3 { AMBER } else { GREEN };
        bar(ui, st.hp_frac(), hp_color, 5.0);
        bar(ui, st.mana_frac(), BLUE, 3.0);
    }
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 5.0;
        match st.ultimate() {
            Some(u) if u.level == 0 => {
                ui.label(RichText::new("ULT -").color(DIM).size(10.5));
            }
            Some(u) if u.cooldown == 0 => {
                ui.label(RichText::new("ULT").color(GREEN).strong().size(10.5));
            }
            Some(u) => {
                ui.label(RichText::new(format!("ULT {}", u.cooldown)).color(MUTED).size(10.5));
            }
            None => {}
        }
        match st.tp_cooldown() {
            Some(0) => {
                ui.label(RichText::new("TP").color(GREEN).size(10.5));
            }
            Some(cd) => {
                ui.label(RichText::new(format!("TP {cd}")).color(MUTED).size(10.5));
            }
            None => {
                ui.label(RichText::new("no TP").color(RED).size(10.5));
            }
        }
        let (obs, sen) = st.ward_count();
        if obs + sen > 0 {
            ui.label(RichText::new(format!("wards {obs}/{sen}")).color(AMBER).size(10.5));
        }
        if st.smoked {
            ui.label(RichText::new("SMOKED").color(BLUE).size(10.5));
        }
        for it in &st.items {
            let key = is_key_item(&it.name);
            let mut label = item_short(&it.name);
            if it.cooldown > 0 {
                label.push_str(&format!(" {}", it.cooldown));
            }
            let color = if key && it.cooldown == 0 {
                AMBER
            } else if key {
                MUTED
            } else {
                DIM
            };
            ui.label(RichText::new(label).color(color).size(10.0));
        }
    });
    ui.add_space(3.0);
}

fn world_to_radar(r: Rect, x: f32, y: f32) -> Pos2 {
    let fx = ((x + MAP_HALF) / (2.0 * MAP_HALF)).clamp(0.0, 1.0);
    let fy = ((y + MAP_HALF) / (2.0 * MAP_HALF)).clamp(0.0, 1.0);
    Pos2::new(r.left() + fx * r.width(), r.bottom() - fy * r.height())
}

fn radar(ui: &mut egui::Ui, me: Option<&Mate>, mates: &[Mate], callouts: &[crate::team::CalloutEv], now: Instant) {
    let size = 150.0;
    let (r, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    let p = ui.painter_at(r);
    p.rect_filled(r, CornerRadius::same(4), Color32::from_rgba_unmultiplied(20, 24, 30, 200));
    // River and the two bases, for orientation.
    p.line_segment([r.left_top(), r.right_bottom()], Stroke::new(6.0_f32, Color32::from_rgba_unmultiplied(40, 70, 110, 90)));
    p.circle_filled(Pos2::new(r.left() + 10.0, r.bottom() - 10.0), 6.0, Color32::from_rgba_unmultiplied(63, 185, 80, 70));
    p.circle_filled(Pos2::new(r.right() - 10.0, r.top() + 10.0), 6.0, Color32::from_rgba_unmultiplied(248, 81, 73, 70));

    // 0..6 px ring growth on an 800 ms cycle.
    let pulse = (protocol::now_ms() % 800) as f32 / 800.0 * 6.0;

    for c in callouts {
        let age = now.duration_since(c.at).as_secs_f32();
        if age < 8.0 && (c.x != 0.0 || c.y != 0.0) {
            let pos = world_to_radar(r, c.x, c.y);
            p.circle_stroke(pos, 6.0 + pulse, Stroke::new(1.5_f32, AMBER));
        }
    }
    let draw = |m: &Mate, is_me: bool| {
        let Some(st) = m.state.as_ref().filter(|s| s.in_match() && s.has_pos) else { return };
        let pos = world_to_radar(r, st.x, st.y);
        let status = m.status(now);
        if status == Status::UnderAttack {
            p.circle_stroke(pos, 5.0 + pulse, Stroke::new(2.0_f32, RED));
        }
        p.circle_filled(pos, 4.0, status_color(status));
        if is_me {
            p.circle_stroke(pos, 6.0, Stroke::new(1.5_f32, TEXT));
        }
        let tag: String = m.name.chars().take(3).collect();
        p.text(pos + Vec2::new(6.0, -6.0), egui::Align2::LEFT_BOTTOM, tag, egui::FontId::proportional(10.0), TEXT);
    };
    for m in mates {
        draw(m, false);
    }
    if let Some(me) = me {
        draw(me, true);
    }
}
