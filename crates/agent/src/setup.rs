//! First-launch setup window: name, team key, look. Opens automatically when
//! there is no team key yet, and with `--setup`. Everything here can be
//! changed later in the in-game menu (Numpad *).

use crate::{config::Config, forms, theme};
use eframe::egui::{self, RichText, Vec2};
use std::sync::{Arc, Mutex};

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

struct SetupApp {
    cfg: Config,
    out: Arc<Mutex<Option<Config>>>,
    show_key: bool,
    show_server: bool,
    host_edit: String,
    tried: bool,
}

impl eframe::App for SetupApp {
    fn clear_color(&self, _v: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let t = theme::get(&self.cfg.overlay.theme);
        theme::style(ctx, &t);
        let screen_w = ctx.input(|i| i.viewport().monitor_size).map(|s| s.x).unwrap_or_else(forms::screen_width);
        let mut save = false;
        let close = forms::window_chrome(ctx, &t, "Team Link setup", |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).max_height(ui.available_height() - 64.0).show(ui, |ui| {
                ui.label(RichText::new("Welcome").size(24.0).strong().color(t.text));
                forms::hint(ui, &t, "Three things and you're in. You can change all of it later in game with Numpad *.");
                forms::heading(ui, &t, "You");
                forms::card(ui, &t, |ui| {
                    forms::you(ui, &t, &mut self.cfg, &mut self.show_key, &mut self.host_edit, &mut self.show_server);
                });
                forms::heading(ui, &t, "Look");
                forms::card(ui, &t, |ui| {
                    forms::look(ui, &t, &mut self.cfg, screen_w);
                });
                forms::heading(ui, &t, "Show on the overlay");
                forms::card(ui, &t, |ui| {
                    forms::panels(ui, &t, &mut self.cfg);
                });
                forms::heading(ui, &t, "Dota needs one launch option");
                forms::card(ui, &t, |ui| {
                    forms::hint(ui, &t, "Steam > Dota 2 > Properties > Launch options:");
                    ui.label(RichText::new("-gamestateintegration").monospace().size(14.0).color(t.accent));
                    forms::hint(ui, &t, "And in Dota's video settings use Borderless window, so the overlay can draw over the game.");
                });
            });
            ui.add_space(8.0);
            let name_ok = !self.cfg.name.trim().is_empty();
            let key_ok = !self.cfg.key.trim().is_empty();
            ui.horizontal(|ui| {
                if self.tried && !(name_ok && key_ok) {
                    ui.label(RichText::new(if !name_ok { "Type your name." } else { "The team key is needed; ask your host." }).color(t.bad));
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let start = egui::Button::new(RichText::new("Save and start").strong().color(if t.light { egui::Color32::WHITE } else { egui::Color32::from_rgb(4, 16, 20) }))
                        .fill(t.accent)
                        .corner_radius(9.0)
                        .min_size(Vec2::new(150.0, 36.0));
                    if ui.add(start).clicked() {
                        self.tried = true;
                        if name_ok && key_ok {
                            save = true;
                        }
                    }
                });
            });
        });
        if save {
            if self.show_server && !self.host_edit.trim().is_empty() {
                self.cfg.relay = relay_url(&self.host_edit);
            }
            self.cfg.name = self.cfg.name.trim().chars().take(24).collect();
            self.cfg.key = self.cfg.key.trim().to_string();
            *self.out.lock().unwrap() = Some(self.cfg.clone());
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if close {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

/// Shows the setup window. Returns the new settings, or None if it was closed.
pub fn run(cfg: &Config) -> Option<Config> {
    let out = Arc::new(Mutex::new(None));
    let (w, h) = (560.0, 720.0f32.min(forms::screen_height() - 80.0));
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
    let app = SetupApp {
        cfg: cfg.clone(),
        out: out.clone(),
        show_key: false,
        show_server: false,
        host_edit: relay_host(&cfg.relay),
        tried: false,
    };
    if let Err(e) = eframe::run_native("cyberspyde setup", opts, Box::new(move |_cc| Ok(Box::new(app)))) {
        tracing::error!("setup window failed: {e}");
    }
    let result = out.lock().unwrap().take();
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
}
