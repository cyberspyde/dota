//! Transparent, click-through, always-on-top overlay plus the in-game menu.
//! Works with Dota in borderless-window mode (exclusive fullscreen draws over
//! every window).

use crate::{
    builds::{advise, BuildDb},
    coach::Coach,
    config::{self, Config},
    dota::{self, FeedStatus},
    forms,
    net::NetConfig,
    setup,
    team::{Mate, Notifier, Severity, SharedTeam, Status, MAP_HALF},
    theme::{self, Theme},
};
use eframe::egui::{self, Color32, CornerRadius, Pos2, Rect, RichText, Sense, Shape, Stroke, Vec2};
use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use protocol::{hero_display, is_key_item, item_short, CalloutKind, ClientMsg};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{mpsc, watch};

/// Tall enough for 5 detailed rows, alerts, radar and build help; the rest is transparent.
const H: f32 = 640.0;
const TITLE: &str = "cyberspyde overlay";
const MENU_W: f32 = 640.0;
const MENU_H: f32 = 600.0;

const MENU_GUTTER: i8 = 12;
const MENU_FOOTER_ROOM: f32 = 74.0;

fn menu_bounds(screen: Vec2, zoom: f32, saved: Vec2) -> (Vec2, Vec2) {
    let margin = Vec2::splat(12.0).min(screen * 0.05);
    let size = Vec2::new(MENU_W, MENU_H).min((screen - margin * 2.0) / zoom);
    let last = screen - size * zoom - margin;
    let position = if saved.x >= 0.0 && saved.y >= 0.0 {
        saved.max(margin).min(last)
    } else {
        (screen - size * zoom) / 2.0
    };
    (size, position / zoom)
}

fn menu_inset<R>(ui: &mut egui::Ui, content: impl FnOnce(&mut egui::Ui) -> R) -> egui::InnerResponse<R> {
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(MENU_GUTTER, 0))
        .show(ui, content)
}

fn menu_tabs(ui: &mut egui::Ui, t: &Theme, selected: &mut Tab) -> egui::Response {
    menu_inset(ui, |ui| {
        ui.horizontal(|ui| {
            const TABS: [(Tab, &str); 5] = [
                (Tab::General, "General"), (Tab::Look, "Look"), (Tab::Overlay, "Overlay"),
                (Tab::Hotkeys, "Hotkeys"), (Tab::About, "About"),
            ];
            let gap = 4.0;
            ui.spacing_mut().item_spacing.x = gap;
            ui.spacing_mut().button_padding.x = 7.0;
            let width = ((ui.available_width() - gap * (TABS.len() - 1) as f32) / TABS.len() as f32).max(1.0);
            for (tab, name) in TABS {
                let active = *selected == tab;
                let button = egui::Button::new(RichText::new(name).size(13.0).strong().color(if active { t.accent } else { t.muted }))
                    .fill(if active { t.accent.gamma_multiply(0.15) } else { t.track })
                    .stroke(Stroke::NONE)
                    .corner_radius(8.0);
                if ui.add_sized(Vec2::new(width, 32.0), button).clicked() {
                    *selected = tab;
                }
            }
        })
    }).response
}

fn menu_scroll<R>(ui: &mut egui::Ui, tab: Tab, content: impl FnOnce(&mut egui::Ui) -> R)
    -> egui::scroll_area::ScrollAreaOutput<egui::InnerResponse<R>>
{
    ui.scope(|ui| {
        // Keep the bar in the reserved right gutter without shifting the cards
        // between short tabs and scrollable ones.
        let scrollbar = &mut ui.spacing_mut().scroll;
        scrollbar.floating = true;
        scrollbar.floating_allocated_width = 0.0;
        scrollbar.bar_width = 6.0;
        let height = (ui.available_height() - MENU_FOOTER_ROOM).max(24.0);
        egui::ScrollArea::vertical()
            .id_salt(("settings-body", tab))
            .auto_shrink([false, false])
            .min_scrolled_height(0.0)
            .max_height(height)
            .show(ui, |ui| menu_inset(ui, content))
    }).inner
}

fn menu_footer(ui: &mut egui::Ui, t: &Theme, shortcut: &str) -> (bool, bool) {
    let mut done = false;
    let mut exit_app = false;
    ui.horizontal(|ui| {
        let size = Vec2::new(100.0, 34.0);
        let exit = egui::Button::new(RichText::new("Exit app").strong().color(t.bad)).corner_radius(8.0);
        if ui.add_sized(size, exit).on_hover_text("Close the overlay and stop the background connection").clicked() {
            exit_app = true;
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let color = if t.light { Color32::WHITE } else { Color32::from_rgb(4, 16, 20) };
            let button = egui::Button::new(RichText::new("Done").strong().color(color))
                .fill(t.accent).corner_radius(8.0);
            if ui.add_sized(size, button).clicked() {
                done = true;
            }
            let hint = format!("{} opens settings", forms::key_label(shortcut));
            let center = egui::Layout::centered_and_justified(egui::Direction::LeftToRight);
            ui.with_layout(center, |ui| {
                ui.add(egui::Label::new(RichText::new(&hint).size(12.0).color(t.muted))
                    .truncate()).on_hover_text(hint);
            });
        });
    });
    (done, exit_app)
}


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

/// Opens a link in the default browser.
pub fn open_url(url: &str) {
    #[cfg(windows)]
    let _ = std::process::Command::new("rundll32").args(["url.dll,FileProtocolHandler", url]).spawn();
    #[cfg(not(windows))]
    let _ = std::process::Command::new("xdg-open").arg(url).spawn();
}

#[derive(Clone, Copy)]
enum Action {
    Callout(CalloutKind),
    Edit,
    Toggle,
    Menu,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Tab {
    General,
    Look,
    Overlay,
    Hotkeys,
    About,
}

pub struct OverlayApp {
    #[cfg(windows)]
    tray: Option<crate::tray::Tray>,
    cfg: Config,
    cfg_path: PathBuf,
    team: SharedTeam,
    out: mpsc::Sender<ClientMsg>,
    net_tx: watch::Sender<NetConfig>,
    db: Arc<BuildDb>,
    hk: Option<GlobalHotKeyManager>,
    registered: Vec<HotKey>,
    hk_map: HashMap<u32, Action>,
    hk_errors: Vec<String>,
    edit: bool,
    hidden: bool,
    flash: Option<(String, Instant)>,
    coach: Coach,
    started: Instant,
    dota_checked_at: Option<Instant>,
    dota_running: Option<bool>,
    // Size currently applied to the window.
    applied: (f32, f32),
    applied_pos: (f32, f32),
    // Menu.
    menu_open: bool,
    menu_focus: bool,
    /// Last known menu window rectangle (UI points), saved when it closes.
    menu_rect: Option<Rect>,
    tab: Tab,
    show_key: bool,
    show_server: bool,
    host_edit: String,
    net_dirty: bool,
    keys_dirty: bool,
    /// The post-game card window.
    card: crate::postgame::CardView,
}

pub fn run(
    cfg: Config,
    cfg_path: PathBuf,
    team: SharedTeam,
    notify: Arc<Notifier>,
    out: mpsc::Sender<ClientMsg>,
    net_tx: watch::Sender<NetConfig>,
    db: Arc<BuildDb>,
    _enable_tray: bool,
) -> eframe::Result<()> {
    let w = cfg.overlay.width();
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(TITLE)
            .with_inner_size([w * cfg.overlay.scale, H * cfg.overlay.scale])
            .with_position([cfg.overlay.x, cfg.overlay.y])
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_mouse_passthrough(true)
            .with_taskbar(false)
            .with_resizable(false)
            .with_icon(crate::icon()),
        ..Default::default()
    };
    eframe::run_native(
        "cyberspyde overlay",
        opts,
        Box::new(move |cc| {
            notify.set(cc.egui_ctx.clone());
            cc.egui_ctx.set_zoom_factor(cfg.overlay.scale);
            theme::style(&cc.egui_ctx, &theme::get(&cfg.overlay.theme));
            #[allow(unused_mut)]
            let mut app = OverlayApp::new(cfg, cfg_path, team, out, net_tx, db);
            #[cfg(windows)]
            if _enable_tray {
                match crate::tray::Tray::new(cc.egui_ctx.clone()) {
                    Ok(tray) => app.tray = Some(tray),
                    Err(e) => tracing::warn!("could not create tray icon: {e}"),
                }
            }
            Ok(Box::new(app))
        }),
    )
}

impl OverlayApp {
    fn new(cfg: Config, cfg_path: PathBuf, team: SharedTeam, out: mpsc::Sender<ClientMsg>, net_tx: watch::Sender<NetConfig>, db: Arc<BuildDb>) -> Self {
        // Must be created on the UI thread: it relies on that thread's message loop.
        let (hk, err) = match GlobalHotKeyManager::new() {
            Ok(m) => (Some(m), None),
            Err(e) => (None, Some(format!("hotkeys unavailable: {e}"))),
        };
        let host_edit = setup::relay_host(&cfg.relay);
        let applied = (cfg.overlay.width(), cfg.overlay.scale);
        let applied_pos = (cfg.overlay.x, cfg.overlay.y);
        let mut app = OverlayApp {
            #[cfg(windows)]
            tray: None,
            cfg,
            cfg_path,
            team,
            out,
            net_tx,
            db,
            hk,
            registered: Vec::new(),
            hk_map: HashMap::new(),
            hk_errors: err.into_iter().collect(),
            edit: false,
            hidden: false,
            flash: None,
            coach: Coach::default(),
            started: Instant::now(),
            dota_checked_at: None,
            dota_running: None,
            applied,
            applied_pos,
            menu_open: false,
            menu_focus: false,
            menu_rect: None,
            // Debug aid for screenshots: CYBERSPYDE_MENU_TAB=look|overlay|hotkeys|about
            tab: match std::env::var("CYBERSPYDE_MENU_TAB").unwrap_or_default().as_str() {
                "look" => Tab::Look,
                "overlay" => Tab::Overlay,
                "hotkeys" => Tab::Hotkeys,
                "about" => Tab::About,
                _ => Tab::General,
            },
            show_key: false,
            show_server: false,
            host_edit,
            net_dirty: false,
            keys_dirty: false,
            card: Default::default(),
        };
        app.register_hotkeys();
        app
    }

    fn register_hotkeys(&mut self) {
        let Some(mgr) = &self.hk else { return };
        let _ = mgr.unregister_all(&self.registered);
        self.registered.clear();
        self.hk_map.clear();
        self.hk_errors.retain(|e| e.starts_with("hotkeys unavailable"));
        let h = &self.cfg.hotkeys;
        let binds = [
            (&h.menu, Action::Menu),
            (&h.toggle, Action::Toggle),
            (&h.edit_mode, Action::Edit),
            (&h.help, Action::Callout(CalloutKind::Help)),
            (&h.group, Action::Callout(CalloutKind::Group)),
            (&h.retreat, Action::Callout(CalloutKind::Retreat)),
            (&h.careful, Action::Callout(CalloutKind::Careful)),
            (&h.on_my_way, Action::Callout(CalloutKind::OnMyWay)),
            (&h.roshan, Action::Callout(CalloutKind::Roshan)),
        ];
        for (spec, action) in binds {
            if spec.trim().is_empty() {
                continue;
            }
            match spec.parse::<HotKey>() {
                Ok(hk) => match mgr.register(hk) {
                    Ok(()) => {
                        self.hk_map.insert(hk.id(), action);
                        self.registered.push(hk);
                    }
                    Err(e) => self.hk_errors.push(format!("{}: {e}", forms::key_label(spec))),
                },
                Err(e) => self.hk_errors.push(format!("{}: {e}", forms::key_label(spec))),
            }
        }
        for e in &self.hk_errors {
            tracing::warn!("hotkey: {e}");
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
                    self.flash(if sent { format!("sent: {}", kind.label()) } else { "callout not sent".into() });
                }
                Action::Toggle => self.hidden = !self.hidden,
                Action::Edit => self.set_edit(ctx, !self.edit),
                Action::Menu => {
                    if self.menu_open {
                        self.close_menu(ctx);
                    } else {
                        self.open_menu(ctx);
                    }
                }
            }
        }
    }

    fn flash(&mut self, msg: String) {
        self.flash = Some((msg, Instant::now()));
    }

    fn set_edit(&mut self, ctx: &egui::Context, on: bool) {
        self.edit = on;
        ctx.send_viewport_cmd_to(egui::ViewportId::ROOT, egui::ViewportCommand::MousePassthrough(!on));
        if !on {
            self.save_position(ctx);
        }
    }

    fn save_position(&mut self, ctx: &egui::Context) {
        if let Some(r) = ctx.input_for(egui::ViewportId::ROOT, |i| i.viewport().outer_rect) {
            // outer_rect is in zoomed UI points; the config stores logical pixels.
            let z = ctx.zoom_factor();
            self.cfg.overlay.x = r.min.x * z;
            self.cfg.overlay.y = r.min.y * z;
            self.applied_pos = (self.cfg.overlay.x, self.cfg.overlay.y);
            self.save("position saved");
        }
    }

    fn save(&mut self, ok: &str) {
        match config::save(&self.cfg_path, &self.cfg) {
            Ok(()) => self.flash(ok.into()),
            Err(e) => self.flash(format!("save failed: {e}")),
        }
    }

    /// Opening the menu also unlocks the overlay so it can be dragged into place.
    fn open_menu(&mut self, ctx: &egui::Context) {
        self.menu_open = true;
        self.menu_focus = true;
        self.menu_rect = None;
        ctx.send_viewport_cmd_to(egui::ViewportId::ROOT, egui::ViewportCommand::MousePassthrough(false));
    }

    fn close_menu(&mut self, ctx: &egui::Context) {
        self.menu_open = false;
        // Overlay back to click-through (unless move mode is on), remember both places.
        ctx.send_viewport_cmd_to(egui::ViewportId::ROOT, egui::ViewportCommand::MousePassthrough(!self.edit));
        let z = ctx.zoom_factor();
        if let Some(r) = ctx.input_for(egui::ViewportId::ROOT, |i| i.viewport().outer_rect) {
            self.cfg.overlay.x = r.min.x * z;
            self.cfg.overlay.y = r.min.y * z;
            self.applied_pos = (self.cfg.overlay.x, self.cfg.overlay.y);
        }
        if let Some(r) = self.menu_rect.take() {
            self.cfg.overlay.menu_x = (r.min.x * z).max(0.0);
            self.cfg.overlay.menu_y = (r.min.y * z).max(0.0);
        }
        if self.show_server && !self.host_edit.trim().is_empty() {
            let url = setup::relay_url(&self.host_edit);
            if url != self.cfg.relay {
                self.cfg.relay = url;
                self.net_dirty = true;
            }
            self.show_server = false;
        }
        self.cfg.name = self.cfg.name.trim().chars().take(24).collect();
        self.cfg.key = self.cfg.key.trim().to_string();
        if self.net_dirty {
            self.net_dirty = false;
            self.team.lock().unwrap().my_name = self.cfg.name.clone();
            let _ = self.net_tx.send(NetConfig { url: self.cfg.relay.clone(), key: self.cfg.key.clone(), name: self.cfg.name.clone() });
        }
        if self.keys_dirty {
            self.keys_dirty = false;
            self.register_hotkeys();
        }
        self.save("settings saved");
    }

    fn request_exit(&mut self, ctx: &egui::Context) {
        if self.menu_open {
            self.close_menu(ctx);
        }
        ctx.send_viewport_cmd_to(egui::ViewportId::ROOT, egui::ViewportCommand::Close);
    }

    /// Resizes / moves the window when size, layout or side changed in the menu.
    fn apply_geometry(&mut self, ctx: &egui::Context) {
        let want = (self.cfg.overlay.width(), self.cfg.overlay.scale);
        if want != self.applied {
            self.applied = want;
            ctx.set_zoom_factor(want.1);
            ctx.send_viewport_cmd_to(egui::ViewportId::ROOT, egui::ViewportCommand::InnerSize(Vec2::new(want.0, H)));
        }
        let pos = (self.cfg.overlay.x, self.cfg.overlay.y);
        if pos != self.applied_pos {
            self.applied_pos = pos;
            let z = ctx.zoom_factor();
            ctx.send_viewport_cmd_to(egui::ViewportId::ROOT, egui::ViewportCommand::OuterPosition(Pos2::new(pos.0 / z, pos.1 / z)));
        }
    }

    fn screen_w(&self, ctx: &egui::Context) -> f32 {
        ctx.input_for(egui::ViewportId::ROOT, |i| i.viewport().monitor_size)
            .map(|s| s.x * ctx.zoom_factor())
            .unwrap_or_else(forms::screen_width)
    }

    fn menu(&mut self, ctx: &egui::Context, feed_status: FeedStatus) {
        let t = theme::get(&self.cfg.overlay.theme);
        let z = ctx.zoom_factor();
        let (sw, sh) = ctx
            .input_for(egui::ViewportId::ROOT, |i| i.viewport().monitor_size)
            .map(|s| (s.x * z, s.y * z))
            .unwrap_or((forms::screen_width(), forms::screen_height()));
        let screen_w = self.screen_w(ctx);
        let id = egui::ViewportId::from_hash_of("cyberspyde-menu");
        let available = Vec2::new(sw, sh);
        let saved = Vec2::new(self.cfg.overlay.menu_x, self.cfg.overlay.menu_y);
        let (size, position) = menu_bounds(available, z, saved);
        let builder = egui::ViewportBuilder::default()
            .with_title("CYBERSPYDE settings")
            .with_inner_size(size)
            .with_position(position.to_pos2())
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_resizable(false)
            .with_icon(crate::icon());
        let (connected, err, rtt, update, latest) = {
            let tm = self.team.lock().unwrap();
            (tm.connected, tm.last_error.clone(), tm.rtt_ms, tm.update_available(), tm.latest_agent.clone())
        };
        let mut close = false;
        let mut quit = false;
        let mut move_mode = false;
        ctx.show_viewport_immediate(id, builder, |mctx, _class| {
            if self.menu_focus {
                self.menu_focus = false;
                mctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            }
            if let Some(r) = mctx.input(|i| i.viewport().outer_rect) {
                self.menu_rect = Some(r);
            }
            if mctx.input(|i| i.viewport().close_requested() || i.key_pressed(egui::Key::Escape)) {
                close = true;
            }
            close |= forms::window_chrome(mctx, &t, "Settings", |ui| {
                menu_tabs(ui, &t, &mut self.tab);
                ui.add_space(12.0);
                menu_scroll(ui, self.tab, |ui| match self.tab {
                    Tab::General => {
                        forms::heading(ui, &t, "You");
                        forms::card(ui, &t, |ui| {
                            if forms::you(ui, &t, &mut self.cfg, &mut self.show_key, &mut self.host_edit, &mut self.show_server) {
                                self.net_dirty = true;
                            }
                            if self.net_dirty {
                                forms::hint(ui, &t, "Reconnects with the new details when you close the menu.");
                            }
                        });
                        forms::heading(ui, &t, "Status");
                        forms::card(ui, &t, |ui| {
                            ui.horizontal(|ui| {
                                status_dot(ui, if connected { t.good } else { t.bad });
                                if connected {
                                    ui.label(format!("Connected to the team server, {} ms", rtt.map(|r| r.round() as i32).unwrap_or(0)));
                                } else {
                                    let e = err.clone().unwrap_or_else(|| "Not connected".into());
                                    ui.add(egui::Label::new(RichText::new(&e).color(t.bad)).truncate()).on_hover_text(&e);
                                }
                            });
                            ui.horizontal(|ui| {
                                let live = feed_status == FeedStatus::Live;
                                status_dot(ui, if live { t.good } else { t.warn });
                                if live {
                                    ui.label(feed_status.text());
                                } else {
                                    ui.label(RichText::new(feed_status.text()).color(t.warn));
                                }
                            });
                            if feed_status == FeedStatus::WaitingForMatch {
                                forms::hint(ui, &t, "If this stays here in a match, check the -gamestateintegration launch option in Steam.");
                            }
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                if ui.button("Open team site").clicked() {
                                    open_url(&format!("{}/#key={}", setup::site_url(&self.cfg.relay), self.cfg.key));
                                }
                                if ui.button("Move overlay").clicked() {
                                    move_mode = true;
                                }
                            });
                        });
                    }
                    Tab::Look => {
                        forms::heading(ui, &t, "Look");
                        forms::card(ui, &t, |ui| {
                            forms::look(ui, &t, &mut self.cfg, screen_w);
                        });
                    }
                    Tab::Overlay => {
                        forms::heading(ui, &t, "Show on the overlay");
                        forms::card(ui, &t, |ui| {
                            forms::panels(ui, &t, &mut self.cfg);
                        });
                    }
                    Tab::Hotkeys => {
                        forms::heading(ui, &t, "Hotkeys");
                        forms::card(ui, &t, |ui| {
                            if forms::hotkeys(ui, &t, &mut self.cfg.hotkeys) {
                                self.keys_dirty = true;
                            }
                        });
                        forms::hint(ui, &t, "Numpad and navigation keys are free in Dota by default. New keys apply when you close the menu.");
                        for e in &self.hk_errors {
                            ui.label(RichText::new(e).color(t.warn).size(12.0));
                        }
                    }
                    Tab::About => {
                        forms::heading(ui, &t, "CYBERSPYDE Team Link");
                        forms::card(ui, &t, |ui| {
                            ui.label(format!("Version {}", protocol::AGENT_VERSION));
                            if update {
                                ui.label(RichText::new(format!("Version {latest} is out.")).color(t.accent).strong());
                                if ui.button("Update now").clicked() {
                                    // The new copy installs the update on launch, once this one has closed.
                                    if crate::updater::restart() {
                                        quit = true;
                                    } else {
                                        open_url(&format!("{}/download/cyberspyde-agent.exe", setup::site_url(&self.cfg.relay)));
                                    }
                                }
                                forms::hint(ui, &t, "Quit this program, replace the old file with the new one, start it again.");
                            } else if !latest.is_empty() {
                                forms::hint(ui, &t, "You have the latest version.");
                            }
                            ui.add_space(4.0);
                            forms::hint(ui, &t, "Uses Valve's official Game State Integration: you share only your own data. Settings are saved next to the program in agent.toml.");
                        });
                    }
                });
                ui.add_space(5.0);
                let (done, exit) = menu_inset(ui, |ui| {
                    ui.separator();
                    menu_footer(ui, &t, &self.cfg.hotkeys.menu)
                }).inner;
                close |= done;
                quit |= exit;
            });
        });
        if move_mode {
            close = true;
        }
        if quit {
            self.request_exit(ctx);
            return;
        }
        if close {
            self.close_menu(ctx);
        }
        if move_mode {
            self.set_edit(ctx, true);
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
        #[cfg(windows)]
        match self.tray.as_ref().and_then(crate::tray::Tray::take_action) {
            Some(crate::tray::Action::Exit) => {
                self.request_exit(ctx);
                return;
            }
            Some(crate::tray::Action::OpenSettings) if self.menu_open => self.menu_focus = true,
            Some(crate::tray::Action::OpenSettings) => self.open_menu(ctx),
            None => {}
        }
        self.poll_hotkeys(ctx);
        self.apply_geometry(ctx);
        // Timers (respawn, cooldowns) tick between data updates; data updates
        // themselves repaint immediately through the Notifier.
        ctx.request_repaint_after(Duration::from_millis(if self.menu_open { 50 } else { 200 }));
        let t = theme::get(&self.cfg.overlay.theme);
        theme::style(ctx, &t);

        let now = Instant::now();
        if self.dota_checked_at.is_none_or(|checked| now.duration_since(checked) >= Duration::from_secs(1)) {
            self.dota_running = dota::is_running();
            self.dota_checked_at = Some(now);
        }
        let (alerts, coach, mates, me, connected, err, rtt, last_gsi, callouts, note, update) = {
            let mut tm = self.team.lock().unwrap();
            tm.prune(now);
            let coach = self.coach.evaluate(&tm, now);
            let mut alerts = coach.alerts.clone();
            alerts.extend(tm.alerts(now));
            alerts.sort_by_key(|a| a.severity);
            (
                alerts,
                coach,
                tm.mates.values().cloned().collect::<Vec<_>>(),
                tm.me.clone(),
                tm.connected,
                tm.last_error.clone(),
                tm.rtt_ms,
                tm.last_gsi,
                tm.callouts.iter().cloned().collect::<Vec<_>>(),
                tm.notes.back().cloned(),
                tm.update_available(),
            )
        };
        let in_match = me.as_ref().and_then(|m| m.state.as_ref()).is_some_and(|s| s.in_match());
        let feed_status = FeedStatus::from_observations(
            last_gsi.map(|g| now.saturating_duration_since(g)),
            in_match,
            self.dota_running,
        );
        let s = coach.scale;
        let ov = self.cfg.overlay.clone();
        let w = ov.width();

        egui::CentralPanel::default().frame(egui::Frame::NONE).show(ctx, |ui| {
            // Draggable in move mode and whenever the menu is open.
            if self.edit || self.menu_open {
                let r = ui.interact(ctx.screen_rect(), egui::Id::new("drag"), Sense::drag());
                if r.drag_started() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
            }
            if self.hidden && !self.edit && !self.menu_open {
                return;
            }
            // Entrance: fade in and glide in from the screen edge it sits on.
            let intro = {
                let x = (now.duration_since(self.started).as_secs_f32() / 0.7).clamp(0.0, 1.0);
                1.0 - (1.0 - x).powi(3)
            };
            if intro < 1.0 {
                ctx.request_repaint();
            }
            let from_right = self.cfg.overlay.x > self.screen_w(ctx) / 2.0;
            let slide = Vec2::new(if from_right { 1.0 } else { -1.0 } * 28.0 * (1.0 - intro), 0.0);
            ui.set_opacity(intro);
            let painter = ui.painter().clone();
            let bg = painter.add(Shape::Noop);
            let outer = Rect::from_min_size(ui.max_rect().min + Vec2::new(4.0, 4.0) + slide, Vec2::new(w - 8.0, 10.0));
            let inner_rect = outer.shrink2(Vec2::new(12.0, 10.0));
            let mut child = ui.new_child(egui::UiBuilder::new().max_rect(Rect::from_min_size(inner_rect.min, Vec2::new(inner_rect.width(), H))));
            let ui = &mut child;
            ui.spacing_mut().item_spacing = Vec2::new(5.0, 3.0);
            // The menu uses roomier controls; the overlay stays tight.
            ui.spacing_mut().interact_size.y = 14.0;

            // Header: brand mark, connection, game clock.
            ui.horizontal(|ui| {
                let (m, _) = ui.allocate_exact_size(Vec2::new(12.0, 12.0), Sense::hover());
                ui.painter().rect_filled(m, CornerRadius::same(3), t.accent);
                ui.painter().rect_filled(m.shrink(3.5), CornerRadius::same(1), t.bottom.to_opaque());
                ui.label(RichText::new("TEAM").size(10.5).strong().color(t.muted).extra_letter_spacing(1.5));
                if connected {
                    pill(ui, &t, &format!("{} ms", rtt.map(|r| r.round() as i32).unwrap_or(0)), t.good);
                    pill(ui, &t, &format!("{}/5", mates.len() + 1), t.muted);
                } else {
                    pill(ui, &t, "offline", t.bad);
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if let Some(st) = me.as_ref().and_then(|m| m.state.as_ref()).filter(|s| s.in_match()) {
                        ui.label(RichText::new(clock(st.clock)).size(11.5).strong().color(t.text).monospace());
                    }
                    if let Some((msg, at)) = &self.flash {
                        if now.duration_since(*at) < Duration::from_secs(2) {
                            ui.label(RichText::new(msg).size(10.0).color(t.good));
                        }
                    }
                });
            });
            if let Some(e) = err.as_ref().filter(|_| !connected) {
                one_line(ui, RichText::new(e).color(t.bad).size(10.5));
            }
            if feed_status != FeedStatus::Live {
                one_line(ui, RichText::new(feed_status.text()).color(t.warn).size(10.5));
            }
            if self.edit {
                one_line(ui, RichText::new(format!("MOVE: drag me, then press {}", forms::key_label(&self.cfg.hotkeys.edit_mode))).color(t.warn).size(11.0).strong());
            } else if self.menu_open {
                one_line(ui, RichText::new("Drag to move me; saved when the menu closes").color(t.accent).size(11.0).strong());
            }
            if now.duration_since(self.started) < Duration::from_secs(25) && !self.menu_open {
                one_line(ui, RichText::new(format!("{} menu  ·  {} hide", forms::key_label(&self.cfg.hotkeys.menu), forms::key_label(&self.cfg.hotkeys.toggle))).color(t.accent).size(10.5));
            }
            if update {
                one_line(ui, RichText::new("Update available - see menu > About").color(t.accent).size(10.5));
            }
            if let Some(e) = self.hk_errors.first() {
                one_line(ui, RichText::new(format!("hotkey {e}")).color(t.dim).size(9.5));
            }

            // One big instruction.
            if let Some((text, sev)) = &coach.banner {
                let c = if *sev == Severity::Danger { t.bad } else { t.warn };
                ui.add_space(3.0);
                egui::Frame::new().fill(c.gamma_multiply(0.18)).stroke(Stroke::new(1.0_f32, c.gamma_multiply(0.6))).corner_radius(7.0).inner_margin(egui::Margin::symmetric(8, 4)).show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(RichText::new(text).color(c).strong().size(14.0 * s));
                });
            }
            // Alerts: at most three, short.
            for a in alerts.iter().take(3) {
                let (c, size) = match a.severity {
                    Severity::Danger => (t.bad, 12.5),
                    Severity::Warn => (t.warn, 11.5),
                    Severity::Good => (t.good, 11.5),
                };
                ui.label(RichText::new(&a.text).color(c).size(size * s).strong());
            }

            // Team summary: what the team can do right now.
            if ov.show_summary && !coach.focus {
                let all: Vec<&Mate> = me.iter().chain(mates.iter()).collect();
                let live: Vec<_> = all.iter().filter_map(|m| m.state.as_ref()).filter(|s| s.in_match()).collect();
                if !live.is_empty() {
                    let n = live.len();
                    let ults = live.iter().filter(|s| s.alive && s.ultimate().is_some_and(|u| u.level > 0 && u.cooldown == 0)).count();
                    let tps = live.iter().filter(|s| s.tp_cooldown() == Some(0)).count();
                    let dead = live.iter().filter(|s| !s.alive).count();
                    let obs: u32 = live.iter().map(|s| s.ward_count().0).sum();
                    ui.add_space(3.0);
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 4.0;
                        chip(ui, &t, &format!("ULT {ults}/{n}"), if ults * 2 >= n { t.good } else { t.muted });
                        chip(ui, &t, &format!("TP {tps}/{n}"), if tps == n { t.good } else { t.warn });
                        if dead > 0 {
                            chip(ui, &t, &format!("{dead} dead"), t.bad);
                        }
                        chip(ui, &t, &format!("{obs} obs"), if obs > 0 { t.warn } else { t.dim });
                    });
                }
            }

            ui.add_space(4.0);
            // Teammates first; your own row always last, so nobody mistakes
            // the top name for a call-out about them.
            for m in &mates {
                if ov.compact() { row_compact(ui, &t, m, now, false) } else { row_detailed(ui, &t, m, now, false) }
            }
            if mates.is_empty() && connected {
                one_line(ui, RichText::new("No teammates online yet").color(t.dim).size(10.5));
            }
            if let Some(me) = &me {
                if !mates.is_empty() {
                    ui.add_space(2.0);
                    separator(ui, &t);
                }
                if ov.compact() { row_compact(ui, &t, me, now, true) } else { row_detailed(ui, &t, me, now, true) }
            }

            if ov.show_radar && !coach.focus {
                ui.add_space(5.0);
                radar(ui, &t, me.as_ref(), &mates, &callouts, now, w - 32.0);
            }

            // Build help.
            if ov.show_build && !coach.focus {
                if let Some(st) = me.as_ref().and_then(|m| m.state.as_ref()) {
                    if let Some(adv) = advise(&self.db, st, &self.cfg.build_mood) {
                        ui.add_space(4.0);
                        separator(ui, &t);
                        if let Some(h) = &adv.skill_hint {
                            one_line(ui, RichText::new(h).color(t.warn).strong().size(11.0));
                        }
                        if let Some(next) = &adv.next {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("NEXT").size(9.5).strong().color(t.dim));
                                one_line(ui, RichText::new(&next.name).color(t.text).size(11.5));
                                if adv.gold_needed == 0 {
                                    chip(ui, &t, "BUY", t.good);
                                } else {
                                    ui.label(RichText::new(format!("{} more", adv.gold_needed)).size(10.5).color(t.muted));
                                }
                            });
                        }
                        if !adv.skills.is_empty() {
                            let i = (st.level as usize).saturating_sub(1);
                            if let Some(k) = adv.skills.get(i) {
                                ui.label(RichText::new(format!("Skill this level: {k}")).size(10.5).color(t.muted));
                            }
                        }
                        if ov.show_tips {
                            if !adv.phase_tip.is_empty() {
                                ui.label(RichText::new(&adv.phase_tip).color(t.muted).size(10.5));
                            }
                            if let Some(tip) = &adv.tip {
                                ui.label(RichText::new(tip).color(t.dim).size(10.5));
                            }
                        }
                    }
                }
            }

            // Latest tip from the team site.
            if ov.show_notes {
                if let Some(n) = note.filter(|_| !coach.focus) {
                    ui.add_space(3.0);
                    let c = if n.to_me { t.warn } else { t.muted };
                    ui.label(RichText::new(format!("{}: {}", n.from, n.text)).color(c).size(11.0));
                }
            }

            let used = ui.min_rect();
            let panel = Rect::from_min_max(outer.min, Pos2::new(outer.max.x, used.max.y + 10.0));
            let mut shapes = theme::panel(panel, &t, ov.opacity, 12.0);
            if self.edit || self.menu_open {
                let c = if self.edit { t.warn } else { t.accent };
                shapes.push(Shape::Rect(egui::epaint::RectShape::stroke(panel, CornerRadius::same(12), Stroke::new(2.0_f32, c), egui::StrokeKind::Outside)));
            }
            painter.set(bg, Shape::Vec(shapes));
        });

        if self.menu_open {
            self.menu(ctx, feed_status);
        }

        // After a game: your card from the report, next to the overlay.
        let place = crate::postgame::Place {
            x: self.cfg.overlay.x,
            y: self.cfg.overlay.y,
            width: self.cfg.overlay.width() * self.cfg.overlay.scale,
            on_right: self.cfg.overlay.x > self.screen_w(ctx) / 2.0,
        };
        let (relay, key) = (self.cfg.relay.clone(), self.cfg.key.clone());
        self.card.show(ctx, &self.team, &t, &place, &relay, &key);
    }
}

fn clock(t: i32) -> String {
    let a = t.unsigned_abs();
    format!("{}{}:{:02}", if t < 0 { "-" } else { "" }, a / 60, a % 60)
}

fn one_line(ui: &mut egui::Ui, text: RichText) {
    ui.add(egui::Label::new(text).truncate());
}

fn separator(ui: &mut egui::Ui, t: &Theme) {
    let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 5.0), Sense::hover());
    ui.painter().line_segment([Pos2::new(r.left(), r.center().y), Pos2::new(r.right(), r.center().y)], Stroke::new(1.0_f32, t.border));
}

fn status_dot(ui: &mut egui::Ui, c: Color32) {
    let (r, _) = ui.allocate_exact_size(Vec2::new(10.0, 14.0), Sense::hover());
    ui.painter().circle_filled(r.center(), 4.0, c);
}

fn dot(ui: &mut egui::Ui, c: Color32, r: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(r * 2.0 + 3.0, 14.0), Sense::hover());
    ui.painter().circle_filled(rect.center(), r, c);
}

/// Tinted rounded label: `ULT`, `TP 12`, `BKB`.
fn chip(ui: &mut egui::Ui, t: &Theme, text: &str, c: Color32) {
    let font = egui::FontId::proportional(10.0);
    let galley = ui.painter().layout_no_wrap(text.to_string(), font, c);
    let size = galley.size() + Vec2::new(10.0, 4.0);
    let (r, _) = ui.allocate_exact_size(size, Sense::hover());
    let fill = if t.light { c.gamma_multiply(0.14) } else { c.gamma_multiply(0.16) };
    ui.painter().rect_filled(r, CornerRadius::same(5), fill);
    ui.painter().galley(r.min + Vec2::new(5.0, 2.0), galley, c);
}

fn pill(ui: &mut egui::Ui, t: &Theme, text: &str, c: Color32) {
    chip(ui, t, text, c);
}

fn status_color(t: &Theme, s: Status) -> Color32 {
    match s {
        Status::UnderAttack => t.bad,
        Status::Low => t.warn,
        Status::Ok => t.good,
        Status::Dead | Status::Stale | Status::InMenu | Status::NoData => t.dim,
    }
}

fn bar(ui: &mut egui::Ui, t: &Theme, frac: f32, color: Color32, width: f32, height: f32) -> Rect {
    let (r, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    let p = ui.painter();
    p.rect_filled(r, CornerRadius::same((height / 2.0) as u8), t.track);
    let mut fill = r;
    fill.set_width(r.width() * frac.clamp(0.0, 1.0));
    if fill.width() > 0.5 {
        p.rect_filled(fill, CornerRadius::same((height / 2.0) as u8), color);
    }
    r
}

fn ult_tp_chips(ui: &mut egui::Ui, t: &Theme, st: &protocol::PlayerState) {
    if let Some(u) = st.ultimate().filter(|u| u.level > 0) {
        if u.cooldown == 0 {
            chip(ui, t, "ULT", t.good);
        } else {
            chip(ui, t, &format!("ULT {}", u.cooldown), t.dim);
        }
    }
    match st.tp_cooldown() {
        Some(0) => chip(ui, t, "TP", t.good),
        Some(cd) => chip(ui, t, &format!("TP {cd}"), t.dim),
        None => chip(ui, t, "NO TP", t.bad),
    }
}

/// Two lines per player:
///   ● Name  Lina 14                     ULT  TP
///   [=========-----]  BKB  Blink  W2
fn row_detailed(ui: &mut egui::Ui, t: &Theme, m: &Mate, now: Instant, is_me: bool) {
    let status = m.status(now);
    let st = m.state.as_ref().filter(|s| s.in_match());
    ui.add_space(2.0);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 5.0;
        dot(ui, status_color(t, status), 4.0);
        let name_color = if status == Status::Dead { t.dim } else { t.text };
        let mut nt = RichText::new(m.name.chars().take(14).collect::<String>()).color(name_color).size(13.0).strong();
        if is_me {
            nt = nt.underline();
        }
        ui.label(nt);
        if let Some(st) = st {
            ui.label(RichText::new(format!("{} {}", hero_display(&st.hero), st.level)).color(t.muted).size(10.5));
            if st.alive && status != Status::Stale {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    // Right-to-left: push in reverse order.
                    let mut chips: Vec<(String, Color32)> = Vec::new();
                    if let Some(u) = st.ultimate().filter(|u| u.level > 0) {
                        chips.push(if u.cooldown == 0 { ("ULT".into(), t.good) } else { (format!("ULT {}", u.cooldown), t.dim) });
                    }
                    chips.push(match st.tp_cooldown() {
                        Some(0) => ("TP".into(), t.good),
                        Some(cd) => (format!("TP {cd}"), t.dim),
                        None => ("NO TP".into(), t.bad),
                    });
                    for (text, c) in chips.iter().rev() {
                        chip(ui, t, text, *c);
                    }
                });
            }
        } else {
            ui.label(RichText::new("waiting").color(t.dim).size(10.5));
        }
    });
    let Some(st) = st else { return };
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        ui.add_space(14.0);
        if status == Status::Stale {
            ui.label(RichText::new(format!("no data for {}s", now.duration_since(m.updated).as_secs())).color(t.dim).size(10.5));
            return;
        }
        if !st.alive {
            chip(ui, t, &format!("DEAD {}s", st.respawn_seconds), t.bad);
            let bb_ok = st.buyback_cooldown == 0 && st.gold >= st.buyback_cost;
            chip(ui, t, if bb_ok { "BUYBACK READY" } else if st.buyback_cooldown > 0 { "BB on cooldown" } else { "no BB gold" }, if bb_ok { t.good } else { t.dim });
            return;
        }
        let hp_color = match status {
            Status::UnderAttack => t.bad,
            Status::Low => t.warn,
            _ => t.good,
        };
        let w = 104.0;
        let r = bar(ui, t, st.hp_frac(), hp_color, w, 7.0);
        // Thin mana line under the health bar.
        let mr = Rect::from_min_size(Pos2::new(r.left(), r.bottom() + 1.5), Vec2::new(w * st.mana_frac().clamp(0.0, 1.0), 2.0));
        ui.painter().rect_filled(mr, CornerRadius::same(1), t.mana);
        let (obs, _) = st.ward_count();
        if obs > 0 {
            chip(ui, t, &format!("W{obs}"), t.warn);
        }
        for it in st.items.iter().filter(|i| is_key_item(&i.name)).take(3) {
            let label = item_short(&it.name);
            if it.cooldown == 0 {
                chip(ui, t, &label, t.warn);
            } else {
                chip(ui, t, &format!("{label} {}", it.cooldown), t.dim);
            }
        }
    });
}

/// `● Name  [====    ]  ULT  TP  W2  BKB`
fn row_compact(ui: &mut egui::Ui, t: &Theme, m: &Mate, now: Instant, is_me: bool) {
    let status = m.status(now);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        dot(ui, status_color(t, status), 3.5);
        let name: String = m.name.chars().take(9).collect();
        let mut nt = RichText::new(name).color(if status == Status::Dead { t.dim } else { t.text }).size(11.5);
        if is_me {
            nt = nt.strong();
        }
        ui.allocate_ui_with_layout(Vec2::new(66.0, 16.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.set_min_width(66.0);
            ui.add(egui::Label::new(nt).truncate());
        });
        let Some(st) = m.state.as_ref().filter(|s| s.in_match()) else {
            ui.label(RichText::new("waiting").color(t.dim).size(10.0));
            return;
        };
        if status == Status::Stale {
            ui.label(RichText::new(format!("no data {}s", now.duration_since(m.updated).as_secs())).color(t.dim).size(10.0));
            return;
        }
        if !st.alive {
            chip(ui, t, &format!("dead {}s", st.respawn_seconds), t.bad);
            let bb_ok = st.buyback_cooldown == 0 && st.gold >= st.buyback_cost;
            chip(ui, t, "BB", if bb_ok { t.good } else { t.dim });
            return;
        }
        let hp_color = match status {
            Status::UnderAttack => t.bad,
            Status::Low => t.warn,
            _ => t.good,
        };
        bar(ui, t, st.hp_frac(), hp_color, 40.0, 5.0);
        ult_tp_chips(ui, t, st);
        if let Some(it) = st.items.iter().find(|i| is_key_item(&i.name)) {
            chip(ui, t, &item_short(&it.name), if it.cooldown == 0 { t.warn } else { t.dim });
        }
    });
}

fn world_to_radar(r: Rect, x: f32, y: f32) -> Pos2 {
    let fx = ((x + MAP_HALF) / (2.0 * MAP_HALF)).clamp(0.0, 1.0);
    let fy = ((y + MAP_HALF) / (2.0 * MAP_HALF)).clamp(0.0, 1.0);
    Pos2::new(r.left() + fx * r.width(), r.bottom() - fy * r.height())
}

fn radar(ui: &mut egui::Ui, t: &Theme, me: Option<&Mate>, mates: &[Mate], callouts: &[crate::team::CalloutEv], now: Instant, max: f32) {
    let size = max.min(130.0);
    let (r, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    let p = ui.painter_at(r);
    p.rect_filled(r, CornerRadius::same(8), t.track);
    p.line_segment([r.left_bottom() + Vec2::new(8.0, -8.0), r.right_top() + Vec2::new(-8.0, 8.0)], Stroke::new(5.0_f32, t.accent.gamma_multiply(0.15)));
    // 0..5 px ring growth on an 800 ms cycle.
    let pulse = (protocol::now_ms() % 800) as f32 / 800.0 * 5.0;
    for c in callouts {
        if now.duration_since(c.at).as_secs_f32() < 8.0 && (c.x != 0.0 || c.y != 0.0) {
            p.circle_stroke(world_to_radar(r, c.x, c.y), 5.0 + pulse, Stroke::new(1.5_f32, t.warn));
        }
    }
    let draw = |m: &Mate, is_me: bool| {
        let Some(st) = m.state.as_ref().filter(|s| s.in_match() && s.has_pos) else { return };
        let pos = world_to_radar(r, st.x, st.y);
        let status = m.status(now);
        if status == Status::UnderAttack {
            p.circle_stroke(pos, 4.0 + pulse, Stroke::new(1.5_f32, t.bad));
        }
        p.circle_filled(pos, 3.5, status_color(t, status));
        if is_me {
            p.circle_stroke(pos, 6.0, Stroke::new(1.0_f32, t.text));
        }
    };
    for m in mates {
        draw(m, false);
    }
    if let Some(me) = me {
        draw(me, true);
    }
}

#[cfg(test)]
mod menu_layout_tests {
    use super::*;

    #[test]
    fn menu_fits_small_display_at_large_zoom_and_centers_by_default() {
        let screen = Vec2::new(640.0, 480.0);
        let scale = 1.2;
        let (size, centered) = menu_bounds(screen, scale, Vec2::splat(-1.0));
        assert!((centered * scale * 2.0 + size * scale - screen).length() < 0.1);
        let (size, saved) = menu_bounds(screen, scale, Vec2::splat(10_000.0));
        assert!((saved * scale + size * scale).x <= 628.1);
        assert!((saved * scale + size * scale).y <= 468.1);
    }

    #[test]
    fn all_tabs_keep_balanced_gutters_and_footer_clear_when_scrolling() {
        for size in [Vec2::new(MENU_W, MENU_H), Vec2::new(420.0, 375.0)] {
            for tab in [Tab::General, Tab::Look, Tab::Overlay, Tab::Hotkeys, Tab::About] {
                let ctx = egui::Context::default();
                let t = theme::get("default");
                theme::style(&ctx, &t);
                let input = egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
                    ..Default::default()
                };
                let _ = ctx.run(input, |ctx| {
                    forms::window_chrome(ctx, &t, "Settings", |ui| {
                        let inner = ui.max_rect();
                        let mut selected = tab;
                        let tabs = menu_tabs(ui, &t, &mut selected);
                        ui.add_space(12.0);
                        let scroll = menu_scroll(ui, tab, |ui| {
                            let body = ui.max_rect();
                            forms::card(ui, &t, |ui| {
                                let mut cfg = Config::default();
                                match tab {
                                    Tab::General => {
                                        // Exercise the narrow row with a long server and team key.
                                        cfg.relay = format!("wss://{}.example.test/ws", "long".repeat(15));
                                        forms::you(ui, &t, &mut cfg, &mut false, &mut String::new(), &mut false);
                                    }
                                    Tab::Look => { forms::look(ui, &t, &mut cfg, size.x); }
                                    Tab::Overlay => { forms::panels(ui, &t, &mut cfg); }
                                    Tab::Hotkeys => { forms::hotkeys(ui, &t, &mut cfg.hotkeys); }
                                    Tab::About => { ui.label("About"); }
                                }
                            });
                            if matches!(tab, Tab::Hotkeys) {
                                ui.add_space(500.0);
                            }
                            (body, ui.min_rect())
                        });
                        ui.add_space(5.0);
                        let footer = menu_inset(ui, |ui| {
                            ui.separator();
                            menu_footer(ui, &t, "Ctrl+Shift+NumpadMultiply");
                        });
                        let (body, painted) = scroll.inner.inner;
                        assert!((body.left() - inner.left() - MENU_GUTTER as f32).abs() < 1.0);
                        assert!((inner.right() - body.right() - MENU_GUTTER as f32).abs() < 1.0, "{tab:?} {size:?}: gutter body={body:?} window={inner:?}, tabs={:?}", tabs.rect);
                        assert!(painted.right() <= body.right() + 1.0, "{tab:?} content overflowed: {painted:?} vs {body:?}");
                        assert!(tabs.rect.right() <= inner.right() + 1.0, "tabs overflowed: {tabs:?}");
                        assert!(scroll.inner_rect.bottom() <= footer.response.rect.top() + 1.0);
                        assert!(footer.response.rect.bottom() <= inner.bottom() + 1.0,
                            "footer overflowed: {:?} vs {inner:?}", footer.response.rect);
                    });
                });
            }
        }
    }
}
