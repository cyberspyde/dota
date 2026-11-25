//! The post-game card: right after a game, a small window next to the
//! overlay with your own few lines from the report: the facts that matter
//! most, your worst moment, and the coach's line for you. It never appears
//! during a game, and it updates itself when the coach's line arrives.

use crate::{forms, overlay::open_url, setup::site_url, team::SharedTeam, theme::Theme};
use eframe::egui::{self, Color32, Pos2, RichText, Sense, Stroke, Vec2};
use protocol::{PostGameCard, GAME_IN_PROGRESS};
use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

const W: f32 = 400.0;
const H: f32 = 500.0;
/// Room kept free for the buttons at the bottom.
const FOOTER: f32 = 72.0;

/// Keeps long coach text to a few lines on the card; the site has it all.
fn trim(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let cut: String = s.chars().take(max).collect();
    let cut = cut.rsplit_once(' ').map(|(a, _)| a).unwrap_or(&cut);
    format!("{}...", cut.trim_end_matches([',', '.', ';', ' ']))
}
const ENTER: f32 = 0.45;

fn ease(x: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    1.0 - (1.0 - x).powi(3)
}

#[derive(Default)]
pub struct CardView {
    seen_seq: u64,
    card: Option<PostGameCard>,
    /// Cards closed by the player (by match id): updates never reopen them.
    dismissed: HashSet<String>,
    visible: bool,
    shown_at: Option<Instant>,
}

pub struct Place {
    /// Overlay position and width in logical pixels, and which side it is on.
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub on_right: bool,
}

impl CardView {
    /// Picks up a new or updated card from the team state.
    fn poll(&mut self, team: &SharedTeam) {
        let t = team.lock().unwrap();
        if t.postgame_seq == self.seen_seq {
            return;
        }
        self.seen_seq = t.postgame_seq;
        let Some(card) = t.postgame.clone() else { return };
        drop(t);
        let same = self.card.as_ref().is_some_and(|c| c.match_id == card.match_id);
        if !self.dismissed.contains(&card.match_id) && !(same && self.visible) {
            self.visible = true;
            self.shown_at = None;
        }
        self.card = Some(card);
    }

    fn in_game(team: &SharedTeam) -> bool {
        team.lock().unwrap().my_state().is_some_and(|s| s.in_match() && s.game_state == GAME_IN_PROGRESS)
    }

    /// Draws the card window when there is one to show.
    pub fn show(&mut self, ctx: &egui::Context, team: &SharedTeam, t: &Theme, place: &Place, relay: &str, key: &str) {
        self.poll(team);
        if !self.visible || Self::in_game(team) {
            return;
        }
        let Some(card) = self.card.clone() else { return };
        let shown_at = *self.shown_at.get_or_insert_with(Instant::now);
        let since = shown_at.elapsed().as_secs_f32();
        let k = ease(since / ENTER);
        if k < 1.0 || card.coach_pending {
            ctx.request_repaint_after(Duration::from_millis(16));
        }

        let z = ctx.zoom_factor();
        let x = if place.on_right { place.x - W * z - 14.0 } else { place.x + place.width + 14.0 };
        let builder = egui::ViewportBuilder::default()
            .with_title("CYBERSPYDE match report")
            .with_inner_size([W, H])
            .with_position([x.max(0.0) / z, place.y / z])
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_resizable(false)
            .with_taskbar(false)
            // Never steal the keyboard from Dota.
            .with_active(false)
            .with_icon(crate::icon());
        let mut close = false;
        let mut open = false;
        let slide = if place.on_right { 1.0 } else { -1.0 } * 36.0 * (1.0 - k);
        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of("cyberspyde-postgame"), builder, |c, _| {
            if c.input(|i| i.viewport().close_requested() || i.key_pressed(egui::Key::Escape)) {
                close = true;
            }
            close |= forms::window_chrome(c, t, "Match report", |ui| {
                ui.set_opacity(k);
                let mut body = ui.available_rect_before_wrap();
                body.max.y = (body.max.y - FOOTER).max(body.min.y + 50.0);
                let mut child = ui.new_child(egui::UiBuilder::new().max_rect(body.translate(Vec2::new(slide, 0.0))));
                // Text never runs into the buttons below.
                child.set_clip_rect(body.expand2(Vec2::new(8.0, 0.0)));
                let ui = &mut child;
                // Result.
                let (word, color) = match card.won {
                    Some(true) => ("VICTORY", t.good),
                    Some(false) => ("DEFEAT", t.bad),
                    None => ("GAME OVER", t.text),
                };
                ui.horizontal(|ui| {
                    ui.label(RichText::new(word).size(24.0).strong().color(color).extra_letter_spacing(1.5));
                    ui.label(RichText::new(clock(card.duration)).size(15.0).color(t.muted));
                });
                ui.label(RichText::new(&card.team_line).size(12.5).color(t.muted));
                ui.add_space(8.0);

                forms::heading(ui, t, "Your game");
                for (i, (tone, text)) in card.facts.iter().enumerate() {
                    // Each line arrives a beat after the one before.
                    let row_k = ease((since - 0.15 - i as f32 * 0.09) / 0.35);
                    ui.scope(|ui| {
                        ui.multiply_opacity(row_k);
                        fact_row(ui, t, tone, text);
                    });
                }
                if card.facts.is_empty() {
                    ui.label(RichText::new("Not enough data from this game.").color(t.dim));
                }
                if let Some(w) = &card.worst {
                    ui.add_space(6.0);
                    forms::heading(ui, t, "Worst moment");
                    ui.label(RichText::new(w).size(13.5).color(t.text));
                }
                ui.add_space(6.0);
                forms::heading(ui, t, "Coach");
                match (&card.coach, card.coach_pending) {
                    (Some(line), _) => {
                        ui.label(RichText::new(trim(line, 210)).size(13.5).italics().color(t.text));
                    }
                    (None, true) => {
                        ui.horizontal(|ui| {
                            spinner(ui, t, since);
                            ui.label(RichText::new("The coach is writing your line...").size(13.0).color(t.muted));
                        });
                    }
                    (None, false) => {
                        ui.label(RichText::new("No line for you this time.").size(13.0).color(t.dim));
                    }
                }
                if let Some(l) = &card.lesson {
                    ui.add_space(4.0);
                    ui.label(RichText::new(format!("Team lesson: {}", trim(l, 140))).size(12.5).color(t.accent));
                }
            });
            // Buttons pinned to the bottom of the window.
            egui::Area::new(egui::Id::new("postgame-buttons"))
                .fixed_pos(Pos2::new(24.0, H - 56.0))
                .show(c, |ui| {
                    ui.set_opacity(k);
                    ui.set_width(W - 48.0);
                    ui.horizontal(|ui| {
                        let ink = if t.light { Color32::WHITE } else { Color32::from_rgb(4, 16, 20) };
                        let full = egui::Button::new(RichText::new("Full report").strong().color(ink)).fill(t.accent).corner_radius(9.0).min_size(Vec2::new(140.0, 34.0));
                        if ui.add(full).clicked() {
                            open = true;
                        }
                        if ui.add(egui::Button::new("Close").corner_radius(9.0).min_size(Vec2::new(90.0, 34.0))).clicked() {
                            close = true;
                        }
                        if card.replay_facts {
                            ui.label(RichText::new("with replay facts").size(11.0).color(t.dim));
                        }
                    });
                });
        });
        if open {
            open_url(&format!("{}/#/match/{}&key={}", site_url(relay), card.match_id, key));
        }
        if close || open {
            self.visible = false;
            self.dismissed.insert(card.match_id.clone());
        }
    }
}

fn fact_row(ui: &mut egui::Ui, t: &Theme, tone: &str, text: &str) {
    ui.horizontal_top(|ui| {
        let (r, _) = ui.allocate_exact_size(Vec2::new(12.0, 18.0), Sense::hover());
        let c = match tone {
            "good" => t.good,
            "bad" => t.bad,
            _ => t.dim,
        };
        ui.painter().circle_filled(Pos2::new(r.center().x, r.top() + 9.0), 4.0, c);
        ui.add(egui::Label::new(RichText::new(text).size(13.5).color(t.text)).wrap());
    });
}

fn spinner(ui: &mut egui::Ui, t: &Theme, since: f32) {
    let (r, _) = ui.allocate_exact_size(Vec2::splat(16.0), Sense::hover());
    let p = ui.painter();
    p.circle_stroke(r.center(), 6.0, Stroke::new(2.0_f32, t.track));
    let a = since * 5.0;
    let pts: Vec<Pos2> = (0..=12).map(|i| r.center() + Vec2::angled(a + i as f32 / 12.0 * 1.8) * 6.0).collect();
    p.add(egui::Shape::line(pts, Stroke::new(2.0_f32, t.accent)));
}

fn clock(s: i32) -> String {
    let s = s.max(0);
    format!("{}:{:02}", s / 60, s % 60)
}
