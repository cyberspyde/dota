//! Settings controls shared by the first-launch setup window and the in-game
//! menu, plus the glass window chrome both use.

use crate::{
    config::{Config, HotkeyConfig},
    setup::{relay_host, relay_url},
    theme::{self, Theme, THEMES},
};
use eframe::egui::{self, Color32, CornerRadius, RichText, Sense, Stroke, Vec2};

/// Keys offered for hotkeys. Numpad and navigation keys are unbound in Dota
/// by default, so they never steal a game key.
pub const KEYS: &[(&str, &str)] = &[
    ("Numpad0", "Num 0"), ("Numpad1", "Num 1"), ("Numpad2", "Num 2"), ("Numpad3", "Num 3"), ("Numpad4", "Num 4"),
    ("Numpad5", "Num 5"), ("Numpad6", "Num 6"), ("Numpad7", "Num 7"), ("Numpad8", "Num 8"), ("Numpad9", "Num 9"),
    ("NumpadMultiply", "Num *"), ("NumpadAdd", "Num +"), ("NumpadSubtract", "Num -"), ("NumpadDivide", "Num /"),
    ("NumpadDecimal", "Num ."), ("Insert", "Insert"), ("Home", "Home"), ("End", "End"), ("PageUp", "Page Up"),
    ("PageDown", "Page Down"), ("Pause", "Pause"), ("ScrollLock", "Scroll Lock"),
    ("F1", "F1"), ("F2", "F2"), ("F3", "F3"), ("F4", "F4"), ("F5", "F5"), ("F6", "F6"), ("F7", "F7"), ("F8", "F8"),
    ("F9", "F9"), ("F10", "F10"), ("F11", "F11"), ("F12", "F12"),
];

/// "ctrl+shift+Numpad1" -> "Ctrl+Shift+Num 1"
pub fn key_label(spec: &str) -> String {
    if spec.trim().is_empty() {
        return "not set".into();
    }
    spec.split('+')
        .map(|p| match p.to_lowercase().as_str() {
            "ctrl" | "control" => "Ctrl".to_string(),
            "shift" => "Shift".to_string(),
            "alt" => "Alt".to_string(),
            _ => KEYS.iter().find(|(k, _)| k.eq_ignore_ascii_case(p)).map(|(_, l)| l.to_string()).unwrap_or_else(|| p.to_string()),
        })
        .collect::<Vec<_>>()
        .join("+")
}

pub fn heading(ui: &mut egui::Ui, t: &Theme, text: &str) {
    ui.add_space(6.0);
    ui.label(RichText::new(text.to_uppercase()).size(11.5).strong().color(t.accent).extra_letter_spacing(1.2));
}

pub fn hint(ui: &mut egui::Ui, t: &Theme, text: &str) {
    ui.label(RichText::new(text).size(12.0).color(t.muted));
}

/// A card: soft fill and hairline border inside a glass window.
pub fn card<R>(ui: &mut egui::Ui, t: &Theme, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    // Inner width = what is left after the 12 px margins and the 1 px border on both sides.
    let w = (ui.available_width() - 26.0).max(50.0);
    egui::Frame::new()
        .fill(if t.light { Color32::from_rgba_unmultiplied(255, 255, 255, 120) } else { Color32::from_rgba_unmultiplied(255, 255, 255, 9) })
        .stroke(Stroke::new(1.0_f32, t.border))
        .corner_radius(10.0)
        .inner_margin(egui::Margin::same(12))
        .show(ui, |ui| {
            ui.set_width(w);
            add(ui)
        })
        .inner
}

/// Segmented control.
pub fn segmented(ui: &mut egui::Ui, t: &Theme, value: &mut usize, options: &[&str]) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        for (i, o) in options.iter().enumerate() {
            let on = *value == i;
            let text = RichText::new(*o).color(if on { if t.light { Color32::WHITE } else { Color32::from_rgb(6, 16, 20) } } else { t.text });
            let b = egui::Button::new(text)
                .fill(if on { t.accent } else { t.track })
                .stroke(Stroke::NONE)
                .corner_radius(7.0)
                .min_size(Vec2::new(78.0, 28.0));
            if ui.add(b).clicked() && !on {
                *value = i;
                changed = true;
            }
        }
    });
    changed
}

/// iOS-style switch with a label.
pub fn toggle(ui: &mut egui::Ui, t: &Theme, on: &mut bool, label: &str) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        let (rect, resp) = ui.allocate_exact_size(Vec2::new(36.0, 20.0), Sense::click());
        if resp.clicked() {
            *on = !*on;
            changed = true;
        }
        let k = ui.ctx().animate_bool(resp.id, *on);
        let p = ui.painter();
        p.rect_filled(rect, CornerRadius::same(10), t.track.lerp_to_gamma(t.accent, k));
        let x = egui::lerp(rect.left() + 10.0..=rect.right() - 10.0, k);
        p.circle_filled(egui::pos2(x, rect.center().y), 7.0, if t.light { Color32::WHITE } else { Color32::from_rgb(240, 244, 248) });
        if ui.add(egui::Label::new(RichText::new(label).color(t.text)).sense(Sense::click())).clicked() {
            *on = !*on;
            changed = true;
        }
    });
    changed
}

/// Name, team key and server.
pub fn you(ui: &mut egui::Ui, t: &Theme, cfg: &mut Config, show_key: &mut bool, host_edit: &mut String, show_server: &mut bool) -> bool {
    let mut changed = false;
    ui.label(RichText::new("Your name").color(t.muted).size(12.0));
    let r = ui.add(egui::TextEdit::singleline(&mut cfg.name).hint_text(RichText::new("as your team knows you").color(t.dim)).char_limit(24).margin(egui::Margin::symmetric(10, 7)).desired_width(f32::INFINITY));
    changed |= r.changed();
    ui.label(RichText::new("Team key").color(t.muted).size(12.0));
    ui.horizontal(|ui| {
        let margin = egui::Margin::symmetric(10, 7);
        let button = Vec2::new(84.0, 32.0);
        let w = (ui.available_width() - button.x - ui.spacing().item_spacing.x - margin.sum().x).max(24.0);
        let r = ui.add(egui::TextEdit::singleline(&mut cfg.key).password(!*show_key).hint_text(RichText::new("from your host").color(t.dim)).margin(margin).desired_width(w));
        changed |= r.changed();
        if ui.add_sized(button, egui::Button::new(if *show_key { "Hide" } else { "Show" })).clicked() {
            *show_key = !*show_key;
        }
    });
    ui.horizontal(|ui| {
        let button = Vec2::new(84.0, 28.0);
        let server = format!("Server: {}", relay_host(&cfg.relay));
        let label_w = (ui.available_width() - button.x - ui.spacing().item_spacing.x).max(20.0);
        ui.add_sized(
            [label_w, button.y],
            egui::Label::new(RichText::new(&server).size(12.0).color(t.muted)).halign(egui::Align::Min).truncate(),
        ).on_hover_text(server);
        if ui.add_sized(button, egui::Button::new(if *show_server { "Done" } else { "Change" })).clicked() {
            if *show_server && !host_edit.trim().is_empty() {
                let url = relay_url(host_edit);
                if url != cfg.relay {
                    cfg.relay = url;
                    changed = true;
                }
            }
            *show_server = !*show_server;
            *host_edit = relay_host(&cfg.relay);
        }
    });
    if *show_server {
        ui.add(egui::TextEdit::singleline(host_edit).hint_text(RichText::new("dota.cyberspyde.uz").color(t.dim)).margin(egui::Margin::symmetric(10, 7)).desired_width(f32::INFINITY));
        hint(ui, t, "A domain uses an encrypted connection. An IP address connects to port 4747.");
    }
    changed
}

/// Theme swatches, size, layout, side and panels.
pub fn look(ui: &mut egui::Ui, t: &Theme, cfg: &mut Config, screen_w: f32) -> bool {
    let a = look_basic(ui, t, cfg);
    let b = look_more(ui, t, cfg, screen_w);
    a || b
}

/// Theme and size: the easy first choices.
pub fn look_basic(ui: &mut egui::Ui, t: &Theme, cfg: &mut Config) -> bool {
    let mut changed = false;
    ui.label(RichText::new("Theme").color(t.muted).size(12.0));
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
        for th in THEMES.iter() {
            let on = cfg.overlay.theme == th.id;
            let (rect, resp) = ui.allocate_exact_size(Vec2::new(88.0, 58.0), Sense::click());
            let p = ui.painter();
            for s in theme::panel(rect, th, 0.9, 9.0) {
                p.add(s);
            }
            p.circle_filled(rect.left_top() + Vec2::new(14.0, 15.0), 4.0, th.accent);
            p.rect_filled(egui::Rect::from_min_size(rect.left_top() + Vec2::new(24.0, 12.0), Vec2::new(46.0, 6.0)), CornerRadius::same(3), th.good);
            p.rect_filled(egui::Rect::from_min_size(rect.left_top() + Vec2::new(24.0, 22.0), Vec2::new(30.0, 6.0)), CornerRadius::same(3), th.warn);
            p.text(rect.left_bottom() + Vec2::new(10.0, -10.0), egui::Align2::LEFT_BOTTOM, th.name, egui::FontId::proportional(12.0), th.text);
            if on {
                p.rect_stroke(rect.expand(2.0), CornerRadius::same(11), Stroke::new(2.0_f32, t.accent), egui::StrokeKind::Outside);
            } else if resp.hovered() {
                p.rect_stroke(rect.expand(1.0), CornerRadius::same(10), Stroke::new(1.0_f32, t.muted), egui::StrokeKind::Outside);
            }
            if resp.clicked() && !on {
                cfg.overlay.theme = th.id.to_string();
                changed = true;
            }
        }
    });

    ui.add_space(4.0);
    ui.label(RichText::new("Size").color(t.muted).size(12.0));
    let mut size = match cfg.overlay.scale {
        s if s < 0.95 => 0,
        s if s > 1.1 => 2,
        _ => 1,
    };
    if segmented(ui, t, &mut size, &["Small", "Normal", "Large"]) {
        cfg.overlay.scale = [0.85, 1.0, 1.2][size];
        changed = true;
    }
    changed
}

/// Layout, side and opacity: finer choices.
pub fn look_more(ui: &mut egui::Ui, t: &Theme, cfg: &mut Config, screen_w: f32) -> bool {
    let mut changed = false;
    ui.label(RichText::new("Layout").color(t.muted).size(12.0));
    let mut dens = usize::from(cfg.overlay.compact());
    if segmented(ui, t, &mut dens, &["Detailed", "Compact"]) {
        cfg.overlay.density = if dens == 1 { "compact".into() } else { "detailed".into() };
        changed = true;
    }
    ui.label(RichText::new("Position").color(t.muted).size(12.0));
    let w = cfg.overlay.width() * cfg.overlay.scale;
    let mut side = usize::from(cfg.overlay.x + w / 2.0 > screen_w / 2.0);
    if segmented(ui, t, &mut side, &["Left side", "Right side"]) {
        cfg.overlay.x = if side == 0 { 12.0 } else { screen_w - w - 12.0 };
        cfg.overlay.y = 160.0;
        changed = true;
    }
    ui.label(RichText::new(format!("Background opacity {}%", (cfg.overlay.opacity * 100.0).round())).color(t.muted).size(12.0));
    ui.spacing_mut().slider_width = (ui.available_width() - 10.0).min(320.0);
    changed |= ui.add(egui::Slider::new(&mut cfg.overlay.opacity, 0.2..=1.0).show_value(false)).changed();
    changed
}

pub fn panels(ui: &mut egui::Ui, t: &Theme, cfg: &mut Config) -> bool {
    let mut changed = false;
    changed |= toggle(ui, t, &mut cfg.overlay.show_summary, "Team summary (ultimates and TPs ready)");
    changed |= toggle(ui, t, &mut cfg.overlay.show_build, "Next-item help for your hero");
    changed |= toggle(ui, t, &mut cfg.overlay.show_tips, "Game-plan tips under the next item");
    changed |= toggle(ui, t, &mut cfg.overlay.show_radar, "Mini radar");
    changed |= toggle(ui, t, &mut cfg.overlay.show_enemy, "Enemy intel (from the in-game bridge)");
    changed |= toggle(ui, t, &mut cfg.overlay.show_notes, "Tips from the team site");
    changed
}

/// One row per hotkey: modifier switches and a key picker.
pub fn hotkeys(ui: &mut egui::Ui, t: &Theme, h: &mut HotkeyConfig) -> bool {
    let mut changed = false;
    let rows: [(&str, &mut String); 9] = [
        ("Open this menu", &mut h.menu),
        ("Hide / show overlay", &mut h.toggle),
        ("Move overlay", &mut h.edit_mode),
        ("Callout: need help", &mut h.help),
        ("Callout: group up", &mut h.group),
        ("Callout: retreat", &mut h.retreat),
        ("Callout: careful", &mut h.careful),
        ("Callout: on my way", &mut h.on_my_way),
        ("Callout: Roshan", &mut h.roshan),
    ];
    let mut controls = |ui: &mut egui::Ui, label: &str, spec: &mut String| {
        ui.horizontal(|ui| {
            let parts: Vec<String> = spec.split('+').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            let has = |m: &str| parts.iter().any(|p| p.eq_ignore_ascii_case(m));
            let (mut ctrl, mut shift, mut alt) = (has("ctrl"), has("shift"), has("alt"));
            let mut key = parts
                .iter()
                .find(|p| !["ctrl", "shift", "alt", "control"].contains(&p.to_lowercase().as_str()))
                .cloned()
                .unwrap_or_default();
            let mut c = false;
            c |= ui.selectable_label(ctrl, "Ctrl").clicked().then(|| ctrl = !ctrl).is_some();
            c |= ui.selectable_label(shift, "Shift").clicked().then(|| shift = !shift).is_some();
            c |= ui.selectable_label(alt, "Alt").clicked().then(|| alt = !alt).is_some();
            egui::ComboBox::from_id_salt(label)
                .width(110.0)
                .truncate()
                .selected_text(KEYS.iter().find(|(k, _)| k.eq_ignore_ascii_case(&key)).map(|(_, l)| *l).unwrap_or(if key.is_empty() { "none" } else { key.as_str() }))
                .show_ui(ui, |ui| {
                    if ui.selectable_label(key.is_empty(), "none").clicked() {
                        key.clear();
                        c = true;
                    }
                    for (k, l) in KEYS {
                        if ui.selectable_label(key == *k, *l).clicked() {
                            key = k.to_string();
                            c = true;
                        }
                    }
                });
            if c {
                let mut s = Vec::new();
                if !key.is_empty() {
                    if ctrl { s.push("ctrl".to_string()); }
                    if shift { s.push("shift".to_string()); }
                    if alt { s.push("alt".to_string()); }
                    s.push(key.clone());
                }
                *spec = s.join("+");
                changed = true;
            }
        });
    };
    // Stack the labels on narrow displays instead of widening the card into its gutter.
    const CONTROLS_WIDTH: f32 = 292.0;
    let label_width = ui.available_width() - CONTROLS_WIDTH - ui.spacing().item_spacing.x;
    let wide = label_width >= 132.0;
    for (label, spec) in rows {
        if wide {
            ui.horizontal(|ui| {
                ui.add_sized(
                    [label_width, 26.0],
                    egui::Label::new(RichText::new(label).color(t.text)).halign(egui::Align::Min).truncate(),
                );
                controls(ui, label, spec);
            });
        } else {
            ui.label(RichText::new(label).color(t.text));
            controls(ui, label, spec);
            ui.add_space(4.0);
        }
    }
    changed
}

/// Frameless glass window: title bar you can drag, a close button, content.
/// Returns true when the close button was pressed.
pub fn window_chrome(ctx: &egui::Context, t: &Theme, title: &str, add: impl FnOnce(&mut egui::Ui)) -> bool {
    let mut close = false;
    egui::CentralPanel::default().frame(egui::Frame::NONE).show(ctx, |ui| {
        let rect = ui.max_rect().shrink(6.0);
        for s in theme::panel(rect, t, 1.6, 14.0) {
            ui.painter().add(s);
        }
        let bar = egui::Rect::from_min_size(rect.min, Vec2::new(rect.width(), 46.0));
        let drag = ui.interact(bar, egui::Id::new("titlebar"), Sense::click_and_drag());
        if drag.drag_started() {
            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(rect.shrink2(Vec2::new(18.0, 12.0))));
        child.horizontal(|ui| {
            let (m, _) = ui.allocate_exact_size(Vec2::new(18.0, 18.0), Sense::hover());
            ui.painter().rect_filled(m, CornerRadius::same(5), t.accent);
            ui.painter().rect_filled(m.shrink(5.0), CornerRadius::same(2), t.bottom.to_opaque());
            ui.label(RichText::new("CYBERSPYDE").strong().size(14.0).color(t.text).extra_letter_spacing(1.5));
            ui.label(RichText::new(title).size(13.0).color(t.muted));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Drawn by hand: the built-in font has no multiplication-x glyph.
                let (r, resp) = ui.allocate_exact_size(Vec2::new(26.0, 26.0), Sense::click());
                let c = if resp.hovered() { t.bad } else { t.muted };
                if resp.hovered() {
                    ui.painter().rect_filled(r, CornerRadius::same(7), t.bad.gamma_multiply(0.15));
                }
                let k = r.shrink(8.5);
                ui.painter().line_segment([k.left_top(), k.right_bottom()], Stroke::new(1.6_f32, c));
                ui.painter().line_segment([k.right_top(), k.left_bottom()], Stroke::new(1.6_f32, c));
                if resp.on_hover_text("Close").clicked() {
                    close = true;
                }
            });
        });
        child.add_space(8.0);
        add(&mut child);
    });
    close
}

/// Logical screen width, for placing the overlay on the right side.
pub fn screen_width() -> f32 {
    #[cfg(windows)]
    {
        use windows_sys::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN};
        let w = unsafe { GetSystemMetrics(SM_CXSCREEN) };
        if w > 0 {
            return w as f32;
        }
    }
    1920.0
}

pub fn screen_height() -> f32 {
    #[cfg(windows)]
    {
        use windows_sys::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CYSCREEN};
        let h = unsafe { GetSystemMetrics(SM_CYSCREEN) };
        if h > 0 {
            return h as f32;
        }
    }
    1080.0
}
