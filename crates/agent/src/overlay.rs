//! Transparent, click-through, always-on-top overlay. Works with Dota in
//! borderless-window mode (exclusive fullscreen draws over every window).

use crate::{
    coach::Coach,
    builds::{advise, BuildDb},
    config::{self, Config},
    team::{Mate, Notifier, Severity, SharedTeam, Status, MAP_HALF},
};
use eframe::egui::{self, Color32, CornerRadius, Pos2, Rect, RichText, Sense, Stroke, Vec2};
use global_hotkey::{
    hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use protocol::{is_key_item, item_short, CalloutKind, ClientMsg};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::mpsc;

const W: f32 = crate::setup::OVERLAY_W;
/// Tall enough for 5 rows, alerts, radar and build help; the rest is transparent.
const H: f32 = 520.0;

const RED: Color32 = Color32::from_rgb(248, 81, 73);
const AMBER: Color32 = Color32::from_rgb(240, 180, 41);
const GREEN: Color32 = Color32::from_rgb(63, 185, 80);
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
    coach: Coach,
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
            coach: Coach::default(),
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
        let (alerts, coach, mates, me, connected, err, rtt, last_gsi, callouts, note) = {
            let mut t = self.team.lock().unwrap();
            t.prune(now);
            let coach = self.coach.evaluate(&t, now);
            let mut alerts = coach.alerts.clone();
            alerts.extend(t.alerts(now));
            alerts.sort_by_key(|a| a.severity);
            (
                alerts,
                coach,
                t.mates.values().cloned().collect::<Vec<_>>(),
                t.me.clone(),
                t.connected,
                t.last_error.clone(),
                t.rtt_ms,
                t.last_gsi,
                t.callouts.iter().cloned().collect::<Vec<_>>(),
                t.notes.back().cloned(),
            )
        };
        let s = coach.scale;

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
                .fill(Color32::from_rgba_unmultiplied(10, 12, 16, alpha))
                .corner_radius(6.0)
                .inner_margin(egui::Margin::symmetric(8, 6));
            if self.edit {
                frame = frame.stroke(Stroke::new(1.5_f32, AMBER));
            }
            frame.show(ui, |ui| {
                ui.set_width(W - 16.0);
                ui.spacing_mut().item_spacing = Vec2::new(4.0, 2.0);

                // Status: one tiny line; words only when something is wrong.
                ui.horizontal(|ui| {
                    if connected {
                        dot(ui, GREEN, 3.0);
                        let ms = rtt.map(|r| format!("{r:.0}ms")).unwrap_or_default();
                        small(ui, &format!("{ms}  {}/5", mates.len() + 1), DIM);
                    } else {
                        dot(ui, RED, 3.0);
                        small(ui, "offline", RED);
                    }
                    if last_gsi.is_none_or(|t| now.duration_since(t) > Duration::from_secs(20)) {
                        small(ui, "waiting for Dota", AMBER);
                    }
                    if let Some((msg, at)) = &self.flash {
                        if now.duration_since(*at) < Duration::from_secs(2) {
                            small(ui, msg, GREEN);
                        }
                    }
                });
                if let Some(e) = err.as_ref().filter(|_| !connected) {
                    one_line(ui, RichText::new(e).color(RED).size(10.0));
                }
                if self.edit {
                    one_line(ui, RichText::new("MOVE: drag, then press the move key").color(AMBER).size(10.5));
                }
                if let Some(e) = self.hk_errors.first() {
                    one_line(ui, RichText::new(format!("hotkey {e}")).color(DIM).size(9.5));
                }

                // One big instruction.
                if let Some((text, sev)) = &coach.banner {
                    let c = if *sev == Severity::Danger { RED } else { AMBER };
                    ui.add_space(2.0);
                    egui::Frame::new()
                        .fill(c.gamma_multiply(0.2))
                        .corner_radius(4.0)
                        .inner_margin(egui::Margin::symmetric(6, 3))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.label(RichText::new(text).color(c).strong().size(13.5 * s));
                        });
                }

                // Alerts: at most three, short.
                for a in alerts.iter().take(3) {
                    let (c, size) = match a.severity {
                        Severity::Danger => (RED, 12.0),
                        Severity::Warn => (AMBER, 11.0),
                        Severity::Good => (GREEN, 11.0),
                    };
                    ui.label(RichText::new(&a.text).color(c).size(size * s).strong());
                }

                // One line per player.
                ui.add_space(3.0);
                if let Some(me) = &me {
                    row(ui, me, now, true);
                }
                for m in &mates {
                    row(ui, m, now, false);
                }
                if mates.is_empty() && connected {
                    one_line(ui, RichText::new("no teammates yet").color(DIM).size(10.0));
                }

                if self.cfg.overlay.show_radar && !coach.focus {
                    ui.add_space(4.0);
                    radar(ui, me.as_ref(), &mates, &callouts, now);
                }

                // Build help: one line (plus tips if switched on).
                if self.cfg.overlay.show_build && !coach.focus {
                    if let Some(st) = me.as_ref().and_then(|m| m.state.as_ref()) {
                        if let Some(adv) = advise(&self.db, st, &self.cfg.build_mood) {
                            ui.add_space(3.0);
                            if let Some(h) = &adv.skill_hint {
                                one_line(ui, RichText::new(h).color(AMBER).strong().size(11.0));
                            }
                            if let Some(next) = &adv.next {
                                ui.horizontal(|ui| {
                                    small(ui, "Next", DIM);
                                    one_line(ui, RichText::new(&next.name).color(TEXT).size(11.0));
                                    if adv.gold_needed == 0 {
                                        small(ui, "BUY", GREEN);
                                    } else {
                                        small(ui, &format!("need {}", adv.gold_needed), MUTED);
                                    }
                                });
                            }
                            if !adv.skills.is_empty() {
                                let i = (st.level as usize).saturating_sub(1);
                                if let Some(k) = adv.skills.get(i) {
                                    small(ui, &format!("Skill this level: {k}"), MUTED);
                                }
                            }
                            if self.cfg.overlay.show_tips {
                                if !adv.phase_tip.is_empty() {
                                    ui.label(RichText::new(&adv.phase_tip).color(MUTED).size(10.0));
                                }
                                if let Some(t) = &adv.tip {
                                    ui.label(RichText::new(t).color(DIM).size(10.0));
                                }
                            }
                        }
                    }
                }

                // Latest tip from the team board.
                if let Some(n) = note.filter(|_| !coach.focus) {
                    ui.add_space(2.0);
                    let c = if n.to_me { AMBER } else { MUTED };
                    ui.label(RichText::new(format!("{}: {}", n.from, n.text)).color(c).size(10.5));
                }
            });
        });
    }
}

fn small(ui: &mut egui::Ui, text: &str, c: Color32) {
    ui.label(RichText::new(text).color(c).size(10.0));
}

fn one_line(ui: &mut egui::Ui, text: RichText) {
    ui.add(egui::Label::new(text).truncate());
}

fn dot(ui: &mut egui::Ui, c: Color32, r: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(r * 2.0 + 3.0, 12.0), Sense::hover());
    ui.painter().circle_filled(rect.center(), r, c);
}

fn status_color(s: Status) -> Color32 {
    match s {
        Status::UnderAttack => RED,
        Status::Low => AMBER,
        Status::Ok => GREEN,
        Status::Dead | Status::Stale | Status::InMenu | Status::NoData => DIM,
    }
}

fn hp_bar(ui: &mut egui::Ui, frac: f32, color: Color32, width: f32) {
    let (r, _) = ui.allocate_exact_size(Vec2::new(width, 12.0), Sense::hover());
    let bar = Rect::from_center_size(r.center(), Vec2::new(width, 4.0));
    let p = ui.painter();
    p.rect_filled(bar, CornerRadius::same(2), Color32::from_rgb(34, 38, 46));
    let mut fill = bar;
    fill.set_width(bar.width() * frac.clamp(0.0, 1.0));
    p.rect_filled(fill, CornerRadius::same(2), color);
}

/// `● Name  [====    ]  R  TP  W2  BKB Blink`
fn row(ui: &mut egui::Ui, m: &Mate, now: Instant, is_me: bool) {
    let status = m.status(now);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 5.0;
        dot(ui, status_color(status), 3.5);
        let name: String = m.name.chars().take(9).collect();
        let name_color = if status == Status::Dead { DIM } else { TEXT };
        let mut name_text = RichText::new(name).color(name_color).size(11.0);
        if is_me {
            name_text = name_text.strong();
        }
        // Fixed-width, left-aligned name column so the bars line up.
        ui.allocate_ui_with_layout(Vec2::new(66.0, 14.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.set_min_width(66.0);
            ui.add(egui::Label::new(name_text).truncate());
        });

        let Some(st) = m.state.as_ref().filter(|s| s.in_match()) else {
            small(ui, "waiting", DIM);
            return;
        };
        if status == Status::Stale {
            small(ui, &format!("no data {}s", now.duration_since(m.updated).as_secs()), DIM);
            return;
        }
        if !st.alive {
            small(ui, &format!("dead {}s", st.respawn_seconds), RED);
            let bb_ok = st.buyback_cooldown == 0 && st.gold >= st.buyback_cost;
            small(ui, "BB", if bb_ok { GREEN } else { DIM });
            return;
        }
        let hp_color = match status {
            Status::UnderAttack => RED,
            Status::Low => AMBER,
            _ => GREEN,
        };
        hp_bar(ui, st.hp_frac(), hp_color, 40.0);

        if let Some(u) = st.ultimate().filter(|u| u.level > 0) {
            if u.cooldown == 0 {
                ui.label(RichText::new("ULT").color(GREEN).strong().size(10.5));
            } else {
                small(ui, &format!("ULT {}", u.cooldown), DIM);
            }
        }
        match st.tp_cooldown() {
            Some(0) => small(ui, "TP", GREEN),
            Some(cd) => small(ui, &format!("TP {cd}"), DIM),
            None => small(ui, "TP", RED),
        }
        let (obs, _) = st.ward_count();
        if obs > 0 {
            small(ui, &format!("W{obs}"), AMBER);
        }
        for it in st.items.iter().filter(|i| is_key_item(&i.name)).take(2) {
            let label = item_short(&it.name);
            if it.cooldown == 0 {
                small(ui, &label, AMBER);
            } else {
                small(ui, &format!("{label}{}", it.cooldown), DIM);
            }
        }
    });
}

fn world_to_radar(r: Rect, x: f32, y: f32) -> Pos2 {
    let fx = ((x + MAP_HALF) / (2.0 * MAP_HALF)).clamp(0.0, 1.0);
    let fy = ((y + MAP_HALF) / (2.0 * MAP_HALF)).clamp(0.0, 1.0);
    Pos2::new(r.left() + fx * r.width(), r.bottom() - fy * r.height())
}

fn radar(ui: &mut egui::Ui, me: Option<&Mate>, mates: &[Mate], callouts: &[crate::team::CalloutEv], now: Instant) {
    let size = 110.0;
    let (r, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    let p = ui.painter_at(r);
    p.rect_filled(r, CornerRadius::same(4), Color32::from_rgba_unmultiplied(20, 24, 30, 190));
    p.line_segment([r.left_top(), r.right_bottom()], Stroke::new(4.0_f32, Color32::from_rgba_unmultiplied(40, 70, 110, 80)));
    // 0..5 px ring growth on an 800 ms cycle.
    let pulse = (protocol::now_ms() % 800) as f32 / 800.0 * 5.0;
    for c in callouts {
        if now.duration_since(c.at).as_secs_f32() < 8.0 && (c.x != 0.0 || c.y != 0.0) {
            p.circle_stroke(world_to_radar(r, c.x, c.y), 5.0 + pulse, Stroke::new(1.5_f32, AMBER));
        }
    }
    let draw = |m: &Mate, is_me: bool| {
        let Some(st) = m.state.as_ref().filter(|s| s.in_match() && s.has_pos) else { return };
        let pos = world_to_radar(r, st.x, st.y);
        let status = m.status(now);
        if status == Status::UnderAttack {
            p.circle_stroke(pos, 4.0 + pulse, Stroke::new(1.5_f32, RED));
        }
        p.circle_filled(pos, 3.0, status_color(status));
        if is_me {
            p.circle_stroke(pos, 5.0, Stroke::new(1.0_f32, TEXT));
        }
    };
    for m in mates {
        draw(m, false);
    }
    if let Some(me) = me {
        draw(me, true);
    }
}
