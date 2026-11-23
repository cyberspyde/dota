//! The team as this agent sees it, plus the coordination logic: who is under
//! attack, who is dead, who can help. Everything here is computed locally
//! from each teammate's own state stream, so an alert appears on screen the
//! moment that teammate's health update arrives -- no extra round trip.

use protocol::{hero_display, CalloutKind, Coaching, PlayerState, ServerMsg};
use std::{
    collections::{BTreeMap, VecDeque},
    sync::{Arc, Mutex, OnceLock},
    time::{Duration, Instant},
};

/// HP drop (fraction of max) within `ATTACK_WINDOW` that counts as being attacked.
pub const ATTACK_DROP: f32 = 0.12;
pub const ATTACK_WINDOW: Duration = Duration::from_millis(2500);
/// Keep an under-attack alert up this long after damage stops.
pub const ATTACK_HOLD: Duration = Duration::from_millis(3000);
pub const LOW_HP: f32 = 0.30;
pub const STALE_AFTER: Duration = Duration::from_secs(5);
pub const CALLOUT_TTL: Duration = Duration::from_secs(8);
pub const NOTE_TTL: Duration = Duration::from_secs(45);
/// Rough Dota map half-size in world units.
pub const MAP_HALF: f32 = 8200.0;
/// How far back movement is measured.
pub const MOVE_WINDOW: Duration = Duration::from_millis(1500);

#[derive(Clone, Debug)]
pub struct Mate {
    #[allow(dead_code)] // kept for debugging and future per-mate actions
    pub id: u32,
    pub name: String,
    pub state: Option<PlayerState>,
    pub updated: Instant,
    hp_hist: VecDeque<(Instant, f32)>,
    attacked_until: Option<Instant>,
    attack_drop: f32,
    pos_hist: VecDeque<(Instant, f32, f32)>,
}

impl Mate {
    pub fn new(id: u32, name: String) -> Self {
        Mate {
            id,
            name,
            state: None,
            updated: Instant::now(),
            hp_hist: VecDeque::new(),
            attacked_until: None,
            attack_drop: 0.0,
            pos_hist: VecDeque::new(),
        }
    }

    /// Position change over the last ~1.5 s, in world units.
    pub fn movement(&self) -> Option<(f32, f32)> {
        let (_, x0, y0) = self.pos_hist.front()?;
        let (_, x1, y1) = self.pos_hist.back()?;
        Some((x1 - x0, y1 - y0))
    }

    pub fn pos(&self) -> Option<(f32, f32)> {
        self.state.as_ref().filter(|s| s.in_match() && s.alive && s.has_pos).map(|s| (s.x, s.y))
    }

    pub fn under_attack(&self, now: Instant) -> bool {
        self.status(now) == Status::UnderAttack
    }

    pub fn update(&mut self, st: PlayerState, now: Instant) {
        // A respawn or new match resets the damage history.
        let reset = match &self.state {
            Some(old) => (!old.alive && st.alive) || old.hero != st.hero,
            None => true,
        };
        if reset {
            self.hp_hist.clear();
            self.pos_hist.clear();
            self.attacked_until = None;
        }
        if st.alive && st.has_pos {
            self.pos_hist.push_back((now, st.x, st.y));
            while self.pos_hist.front().is_some_and(|(t, _, _)| now.duration_since(*t) > MOVE_WINDOW) {
                self.pos_hist.pop_front();
            }
        } else {
            self.pos_hist.clear();
        }
        if st.alive && st.max_health > 0 {
            let frac = st.hp_frac();
            self.hp_hist.push_back((now, frac));
            while let Some((t, _)) = self.hp_hist.front() {
                if now.duration_since(*t) > ATTACK_WINDOW {
                    self.hp_hist.pop_front();
                } else {
                    break;
                }
            }
            let peak = self.hp_hist.iter().map(|(_, f)| *f).fold(0.0, f32::max);
            let drop = peak - frac;
            if drop >= ATTACK_DROP {
                self.attacked_until = Some(now + ATTACK_HOLD);
                self.attack_drop = drop;
            }
        } else {
            self.attacked_until = None;
        }
        self.state = Some(st);
        self.updated = now;
    }

    pub fn status(&self, now: Instant) -> Status {
        let Some(st) = &self.state else { return Status::NoData };
        if !st.in_match() {
            return Status::InMenu;
        }
        if now.duration_since(self.updated) > STALE_AFTER {
            return Status::Stale;
        }
        if !st.alive {
            return Status::Dead;
        }
        if self.attacked_until.is_some_and(|t| t > now) {
            return Status::UnderAttack;
        }
        if st.hp_frac() < LOW_HP {
            return Status::Low;
        }
        Status::Ok
    }

}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    NoData,
    InMenu,
    Stale,
    Dead,
    UnderAttack,
    Low,
    Ok,
}

#[derive(Clone, Debug)]
pub struct CalloutEv {
    pub from: String,
    pub kind: CalloutKind,
    pub x: f32,
    pub y: f32,
    pub at: Instant,
}

#[derive(Clone, Debug)]
pub struct NoteEv {
    pub from: String,
    pub to_me: bool,
    pub text: String,
    pub at: Instant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Danger,
    Warn,
    /// A positive opportunity, e.g. a good moment to fight.
    Good,
}

#[derive(Clone, Debug)]
pub struct Alert {
    pub severity: Severity,
    pub text: String,
}

#[derive(Default)]
pub struct Team {
    pub my_name: String,
    pub my_id: Option<u32>,
    pub connected: bool,
    pub last_error: Option<String>,
    pub rtt_ms: Option<f32>,
    /// Teammates by relay id (excluding me).
    pub mates: BTreeMap<u32, Mate>,
    pub me: Option<Mate>,
    pub last_gsi: Option<Instant>,
    pub callouts: VecDeque<CalloutEv>,
    pub notes: VecDeque<NoteEv>,
    /// This player's coaching switches from the host (never shown as such).
    pub coaching: Coaching,
}

impl Team {
    pub fn new(name: String) -> Self {
        Team { my_name: name, ..Default::default() }
    }

    pub fn set_my_state(&mut self, st: PlayerState) {
        let now = Instant::now();
        self.last_gsi = Some(now);
        let name = self.my_name.clone();
        self.me.get_or_insert_with(|| Mate::new(0, name)).update(st, now);
    }

    pub fn my_state(&self) -> Option<&PlayerState> {
        self.me.as_ref().and_then(|m| m.state.as_ref())
    }

    fn name_of(&self, id: u32) -> String {
        if Some(id) == self.my_id {
            return self.my_name.clone();
        }
        // Viewers (the team board) are not in the mate list.
        self.mates.get(&id).map(|m| m.name.clone()).unwrap_or_else(|| "Team board".to_string())
    }

    pub fn apply(&mut self, msg: ServerMsg) {
        let now = Instant::now();
        match msg {
            ServerMsg::Welcome { you, members, coaching, .. } => {
                self.my_id = Some(you);
                self.coaching = coaching;
                self.connected = true;
                self.last_error = None;
                self.mates.clear();
                for m in members.into_iter().filter(|m| m.role == protocol::Role::Player) {
                    let mut mate = Mate::new(m.id, m.name);
                    if let Some(st) = m.state {
                        mate.update(st, now);
                    }
                    self.mates.insert(m.id, mate);
                }
            }
            ServerMsg::Joined { member } => {
                if member.role == protocol::Role::Player {
                    self.mates.insert(member.id, Mate::new(member.id, member.name));
                }
            }
            ServerMsg::Left { id } => {
                self.mates.remove(&id);
            }
            ServerMsg::State { id, state, .. } => {
                if Some(id) != self.my_id {
                    self.mates.entry(id).or_insert_with(|| Mate::new(id, format!("#{id}"))).update(state, now);
                }
            }
            ServerMsg::Callout { id, kind, x, y, .. } => {
                let from = self.name_of(id);
                self.callouts.push_back(CalloutEv { from, kind, x, y, at: now });
                while self.callouts.len() > 8 {
                    self.callouts.pop_front();
                }
            }
            ServerMsg::Note { id, to, text, .. } => {
                let to_me = to.is_none() || to == self.my_id;
                if to_me {
                    let from = self.name_of(id);
                    // A repeated tip refreshes the old one instead of stacking.
                    self.notes.retain(|n| !(n.from == from && n.text == text));
                    self.notes.push_back(NoteEv { from, to_me: to.is_some(), text, at: now });
                    while self.notes.len() > 5 {
                        self.notes.pop_front();
                    }
                }
            }
            ServerMsg::Pong { .. } => {}
            ServerMsg::Error { message } => {
                self.last_error = Some(message);
            }
        }
    }

    pub fn prune(&mut self, now: Instant) {
        self.callouts.retain(|c| now.duration_since(c.at) < CALLOUT_TTL);
        self.notes.retain(|n| now.duration_since(n.at) < NOTE_TTL);
    }

    /// The alerts that matter right now, most urgent first.
    pub fn alerts(&self, now: Instant) -> Vec<Alert> {
        let mine = self.my_state().filter(|s| s.in_match());
        let mut out = Vec::new();
        for m in self.mates.values() {
            let Some(st) = &m.state else { continue };
            match m.status(now) {
                Status::UnderAttack => {
                    let mut t = format!(
                        "{} ({}) UNDER ATTACK {:.0}% HP",
                        m.name,
                        hero_display(&st.hero),
                        st.hp_frac() * 100.0
                    );
                    if let Some(me) = mine.filter(|s| s.alive && s.has_pos && st.has_pos) {
                        let d = ((me.x - st.x).powi(2) + (me.y - st.y).powi(2)).sqrt();
                        if d <= 1500.0 {
                            t.push_str(" - right next to you");
                        } else {
                            t.push_str(&format!(" - {:.0} away", d));
                            // TP only matters when walking would take too long.
                            if d > 3000.0 {
                                match me.tp_cooldown() {
                                    Some(0) => t.push_str(", your TP is ready"),
                                    Some(cd) => t.push_str(&format!(", your TP {cd}s")),
                                    None => {}
                                }
                            }
                        }
                    }
                    out.push(Alert { severity: Severity::Danger, text: t });
                }
                Status::Dead => {
                    let bb = if st.buyback_cooldown > 0 {
                        format!("buyback cd {}s", st.buyback_cooldown)
                    } else if st.gold >= st.buyback_cost {
                        "buyback READY".to_string()
                    } else {
                        format!("no buyback gold ({}/{})", st.gold, st.buyback_cost)
                    };
                    out.push(Alert {
                        severity: Severity::Warn,
                        text: format!("{} dead - back in {}s, {}", m.name, st.respawn_seconds, bb),
                    });
                }
                _ => {}
            }
        }
        let dead = self.mates.values().filter(|m| m.status(now) == Status::Dead).count();
        if dead >= 2 {
            out.push(Alert { severity: Severity::Danger, text: format!("{dead} teammates dead - play safe, don't take fights") });
        }
        for c in &self.callouts {
            out.push(Alert { severity: Severity::Warn, text: format!("{}: {}", c.from, c.kind.label()) });
        }
        out.sort_by_key(|a| a.severity);
        out
    }
}

/// Wakes the overlay when new data arrives so it redraws immediately
/// instead of on its next idle tick.
#[derive(Default)]
pub struct Notifier {
    ctx: OnceLock<eframe::egui::Context>,
}

impl Notifier {
    pub fn set(&self, ctx: eframe::egui::Context) {
        let _ = self.ctx.set(ctx);
    }
    pub fn poke(&self) {
        if let Some(c) = self.ctx.get() {
            c.request_repaint();
        }
    }
}

pub type SharedTeam = Arc<Mutex<Team>>;

#[cfg(test)]
mod tests {
    use super::*;

    fn st(hp: u32, alive: bool) -> PlayerState {
        PlayerState { hero: "axe".into(), alive, health: hp, max_health: 1000, ..Default::default() }
    }

    #[test]
    fn detects_burst_damage() {
        let t0 = Instant::now();
        let mut m = Mate::new(1, "a".into());
        m.update(st(1000, true), t0);
        assert_eq!(m.status(t0), Status::Ok);
        m.update(st(950, true), t0 + Duration::from_millis(300));
        assert_eq!(m.status(t0 + Duration::from_millis(300)), Status::Ok, "chip damage is not an alert");
        m.update(st(820, true), t0 + Duration::from_millis(700));
        assert_eq!(m.status(t0 + Duration::from_millis(700)), Status::UnderAttack);
        // Alert holds briefly after damage stops, then clears.
        assert_eq!(m.status(t0 + Duration::from_millis(3000)), Status::UnderAttack);
        m.update(st(820, true), t0 + Duration::from_millis(4000));
        assert_eq!(m.status(t0 + Duration::from_millis(4000)), Status::Ok);
    }

    #[test]
    fn slow_regen_loss_is_not_attack() {
        let t0 = Instant::now();
        let mut m = Mate::new(1, "a".into());
        // Losing 4% per second (e.g. a DoT or creep wave) stays under 12% per window.
        for i in 0..5 {
            m.update(st(1000 - i * 40, true), t0 + Duration::from_secs(i as u64));
        }
        assert_eq!(m.status(t0 + Duration::from_secs(4)), Status::Ok);
    }

    #[test]
    fn dead_and_respawn() {
        let t0 = Instant::now();
        let mut m = Mate::new(1, "a".into());
        m.update(st(1000, true), t0);
        m.update(st(100, true), t0 + Duration::from_millis(500));
        m.update(st(0, false), t0 + Duration::from_millis(900));
        assert_eq!(m.status(t0 + Duration::from_millis(900)), Status::Dead);
        m.update(st(1000, true), t0 + Duration::from_secs(30));
        assert_eq!(m.status(t0 + Duration::from_secs(30)), Status::Ok, "respawn clears attack history");
    }

    #[test]
    fn alert_mentions_distance_and_tp() {
        let mut team = Team::new("me".into());
        team.my_id = Some(9);
        let mut mine = st(1000, true);
        mine.has_pos = true;
        mine.tp = Some(protocol::Item { name: "tpscroll".into(), charges: Some(1), cooldown: 0 });
        team.set_my_state(mine);
        let mut a = st(1000, true);
        a.has_pos = true;
        a.x = 4000.0;
        team.apply(ServerMsg::State { id: 1, server_ms: 0, state: a.clone() });
        a.health = 700;
        team.apply(ServerMsg::State { id: 1, server_ms: 0, state: a });
        let alerts = team.alerts(Instant::now());
        assert_eq!(alerts[0].severity, Severity::Danger);
        assert!(alerts[0].text.contains("4000 away"), "{}", alerts[0].text);
        assert!(alerts[0].text.contains("TP is ready"));

        // Close by: no distance or TP advice, just "right next to you".
        let mut b = st(1000, true);
        b.has_pos = true;
        b.x = 600.0;
        team.apply(ServerMsg::State { id: 2, server_ms: 0, state: b.clone() });
        b.health = 700;
        team.apply(ServerMsg::State { id: 2, server_ms: 0, state: b });
        let alerts = team.alerts(Instant::now());
        assert!(alerts.iter().any(|a| a.text.contains("right next to you") && !a.text.contains("TP")));
    }
}
