//! Looks for the overlay, the menu and the setup window.

use eframe::egui::{self, epaint, Color32, CornerRadius, Pos2, Rect, Shape, Stroke, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub id: &'static str,
    pub name: &'static str,
    /// Panel gradient, top and bottom (alpha is scaled by the opacity setting).
    pub top: Color32,
    pub bottom: Color32,
    pub border: Color32,
    /// Thin highlight along the top edge: the "glass" sheen.
    pub sheen: Color32,
    pub accent: Color32,
    pub text: Color32,
    pub muted: Color32,
    pub dim: Color32,
    pub good: Color32,
    pub warn: Color32,
    pub bad: Color32,
    pub track: Color32,
    pub mana: Color32,
    pub light: bool,
}

const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color32 {
    Color32::from_rgba_premultiplied(
        ((r as u16 * a as u16) / 255) as u8,
        ((g as u16 * a as u16) / 255) as u8,
        ((b as u16 * a as u16) / 255) as u8,
        a,
    )
}

pub const THEMES: [Theme; 5] = [
    Theme {
        id: "glass",
        name: "Glass",
        top: rgba(34, 44, 60, 215),
        bottom: rgba(12, 16, 24, 230),
        border: rgba(170, 200, 235, 46),
        sheen: rgba(255, 255, 255, 40),
        accent: Color32::from_rgb(53, 224, 189),
        text: Color32::from_rgb(236, 240, 246),
        muted: Color32::from_rgb(160, 170, 186),
        dim: Color32::from_rgb(98, 108, 124),
        good: Color32::from_rgb(70, 214, 120),
        warn: Color32::from_rgb(244, 186, 52),
        bad: Color32::from_rgb(255, 92, 96),
        track: rgba(255, 255, 255, 22),
        mana: Color32::from_rgb(80, 146, 255),
        light: false,
    },
    Theme {
        id: "midnight",
        name: "Midnight",
        top: rgba(16, 22, 44, 235),
        bottom: rgba(8, 10, 22, 240),
        border: rgba(110, 130, 255, 60),
        sheen: rgba(150, 170, 255, 34),
        accent: Color32::from_rgb(124, 140, 255),
        text: Color32::from_rgb(230, 234, 250),
        muted: Color32::from_rgb(150, 158, 190),
        dim: Color32::from_rgb(90, 98, 130),
        good: Color32::from_rgb(90, 220, 150),
        warn: Color32::from_rgb(250, 196, 80),
        bad: Color32::from_rgb(255, 104, 120),
        track: rgba(255, 255, 255, 20),
        mana: Color32::from_rgb(110, 150, 255),
        light: false,
    },
    Theme {
        id: "ember",
        name: "Ember",
        top: rgba(48, 24, 18, 225),
        bottom: rgba(18, 10, 8, 235),
        border: rgba(255, 140, 80, 60),
        sheen: rgba(255, 190, 140, 38),
        accent: Color32::from_rgb(255, 138, 76),
        text: Color32::from_rgb(250, 238, 230),
        muted: Color32::from_rgb(196, 168, 150),
        dim: Color32::from_rgb(130, 104, 92),
        good: Color32::from_rgb(140, 220, 100),
        warn: Color32::from_rgb(255, 200, 70),
        bad: Color32::from_rgb(255, 84, 84),
        track: rgba(255, 255, 255, 20),
        mana: Color32::from_rgb(110, 160, 255),
        light: false,
    },
    Theme {
        id: "neon",
        name: "Neon",
        top: rgba(14, 6, 24, 225),
        bottom: rgba(4, 2, 10, 235),
        border: rgba(255, 70, 200, 80),
        sheen: rgba(80, 240, 255, 48),
        accent: Color32::from_rgb(255, 72, 196),
        text: Color32::from_rgb(244, 240, 255),
        muted: Color32::from_rgb(176, 160, 206),
        dim: Color32::from_rgb(110, 96, 140),
        good: Color32::from_rgb(60, 240, 200),
        warn: Color32::from_rgb(255, 220, 60),
        bad: Color32::from_rgb(255, 60, 110),
        track: rgba(255, 255, 255, 22),
        mana: Color32::from_rgb(90, 160, 255),
        light: false,
    },
    Theme {
        id: "frost",
        name: "Frost",
        top: rgba(246, 249, 253, 220),
        bottom: rgba(222, 230, 240, 230),
        border: rgba(255, 255, 255, 170),
        sheen: rgba(255, 255, 255, 200),
        accent: Color32::from_rgb(0, 150, 136),
        text: Color32::from_rgb(20, 26, 36),
        muted: Color32::from_rgb(70, 80, 96),
        dim: Color32::from_rgb(120, 130, 146),
        good: Color32::from_rgb(20, 150, 70),
        warn: Color32::from_rgb(196, 120, 0),
        bad: Color32::from_rgb(214, 40, 50),
        track: rgba(0, 0, 0, 26),
        mana: Color32::from_rgb(40, 100, 220),
        light: true,
    },
];

pub fn get(id: &str) -> Theme {
    THEMES.iter().copied().find(|t| t.id == id).unwrap_or(THEMES[0])
}

/// Scales a colour's alpha by `k` (above 1 makes it more solid, up to opaque).
fn fade(c: Color32, k: f32) -> Color32 {
    let [r, g, b, a] = c.to_srgba_unmultiplied();
    Color32::from_rgba_unmultiplied(r, g, b, (a as f32 * k).clamp(0.0, 255.0) as u8)
}

/// Glass panel: soft shadow, vertical gradient, hairline border, top sheen.
/// `opacity` is the overlay setting (0.62 = the theme as designed, 1.0 = solid).
pub fn panel(rect: Rect, t: &Theme, opacity: f32, radius: f32) -> Vec<Shape> {
    let k = (opacity / 0.62).clamp(0.15, 2.5);
    let r = CornerRadius::same(radius as u8);
    let mut shapes = Vec::new();
    shapes.push(Shape::Rect(epaint::RectShape::filled(
        rect.translate(Vec2::new(0.0, 4.0)).expand(2.0),
        r,
        Color32::from_black_alpha((60.0 * k.min(1.0)) as u8),
    )));
    // Gradient body: one rounded-rectangle fan, each vertex coloured by its
    // height, so translucent colours never overlap and double up.
    let top = fade(t.top, k);
    let bot = fade(t.bottom, k);
    let color_at = |y: f32| {
        let f = ((y - rect.top()) / rect.height().max(1.0)).clamp(0.0, 1.0);
        top.lerp_to_gamma(bot, f)
    };
    let rad = radius.min(rect.width() / 2.0).min(rect.height() / 2.0);
    let corners = [
        (Pos2::new(rect.right() - rad, rect.top() + rad), -90.0f32),
        (Pos2::new(rect.right() - rad, rect.bottom() - rad), 0.0),
        (Pos2::new(rect.left() + rad, rect.bottom() - rad), 90.0),
        (Pos2::new(rect.left() + rad, rect.top() + rad), 180.0),
    ];
    let mut mesh = epaint::Mesh::default();
    let c = rect.center();
    mesh.vertices.push(epaint::Vertex { pos: c, uv: epaint::WHITE_UV, color: color_at(c.y) });
    for (centre, start) in corners {
        for s in 0..=8 {
            let a = (start + s as f32 * 90.0 / 8.0).to_radians();
            let p = Pos2::new(centre.x + a.cos() * rad, centre.y + a.sin() * rad);
            mesh.vertices.push(epaint::Vertex { pos: p, uv: epaint::WHITE_UV, color: color_at(p.y) });
        }
    }
    let n = mesh.vertices.len() as u32 - 1;
    for i in 1..=n {
        mesh.indices.extend_from_slice(&[0, i, if i == n { 1 } else { i + 1 }]);
    }
    shapes.push(Shape::mesh(mesh));
    shapes.push(Shape::Rect(epaint::RectShape::stroke(rect, r, Stroke::new(1.0_f32, t.border), egui::StrokeKind::Inside)));
    shapes.push(Shape::line_segment(
        [Pos2::new(rect.left() + radius, rect.top() + 1.0), Pos2::new(rect.right() - radius, rect.top() + 1.0)],
        Stroke::new(1.0_f32, t.sheen),
    ));
    shapes
}

/// egui visuals matching a theme, for the menu and the setup window.
pub fn visuals(t: &Theme) -> egui::Visuals {
    let mut v = if t.light { egui::Visuals::light() } else { egui::Visuals::dark() };
    v.override_text_color = Some(t.text);
    v.panel_fill = Color32::TRANSPARENT;
    v.window_fill = t.bottom;
    v.extreme_bg_color = if t.light { Color32::from_rgba_unmultiplied(255, 255, 255, 200) } else { Color32::from_rgba_unmultiplied(0, 0, 0, 110) };
    v.faint_bg_color = t.track;
    v.selection.bg_fill = t.accent.gamma_multiply(0.45);
    v.selection.stroke = Stroke::new(1.0_f32, t.accent);
    v.hyperlink_color = t.accent;
    let w = &mut v.widgets;
    for (wv, fill) in [
        (&mut w.inactive, t.track),
        (&mut w.hovered, t.accent.gamma_multiply(0.22)),
        (&mut w.active, t.accent.gamma_multiply(0.35)),
        (&mut w.open, t.accent.gamma_multiply(0.25)),
    ] {
        wv.bg_fill = fill;
        wv.weak_bg_fill = fill;
        wv.corner_radius = CornerRadius::same(7);
        wv.fg_stroke = Stroke::new(1.0_f32, t.text);
    }
    w.noninteractive.bg_stroke = Stroke::new(1.0_f32, t.border);
    w.noninteractive.fg_stroke = Stroke::new(1.0_f32, t.muted);
    w.inactive.bg_stroke = Stroke::new(1.0_f32, t.border);
    w.hovered.bg_stroke = Stroke::new(1.0_f32, t.accent);
    v.window_corner_radius = CornerRadius::same(12);
    v
}

/// Slightly rounder, roomier spacing for the menu and setup window.
pub fn style(ctx: &egui::Context, t: &Theme) {
    ctx.set_visuals(visuals(t));
    ctx.style_mut(|s| {
        s.spacing.item_spacing = Vec2::new(8.0, 8.0);
        s.spacing.button_padding = Vec2::new(12.0, 6.0);
        s.spacing.interact_size.y = 28.0;
        use egui::{FontId, TextStyle};
        s.text_styles.insert(TextStyle::Heading, FontId::proportional(20.0));
        s.text_styles.insert(TextStyle::Body, FontId::proportional(14.0));
        s.text_styles.insert(TextStyle::Button, FontId::proportional(14.0));
        s.text_styles.insert(TextStyle::Small, FontId::proportional(11.5));
    });
}
