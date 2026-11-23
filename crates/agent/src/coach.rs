//! Personal coaching: turns the live team picture into short instructions,
//! with only the aids the host switched on for this player. Nothing here
//! says *why* an aid is on; players just see clear, timely prompts.

use crate::team::{Alert, CalloutEv, Severity, Status, Team};
use protocol::{CalloutKind, Feature};
use std::time::{Duration, Instant};

/// Allies this close to a damaged ally make it a team fight, not a pick-off.
const FIGHT_RADIUS: f32 = 2500.0;
/// "Near the fight" for nudges and focus mode.
const NEAR_FIGHT: f32 = 3500.0;
const ISOLATION_DIST: f32 = 4500.0;
const ISOLATION_AFTER: Duration = Duration::from_secs(8);
const ENGAGE_RADIUS: f32 = 1500.0;
const FIGHT_DRAGGING: Duration = Duration::from_secs(10);
/// Minimum movement over the 1.5 s window that counts as "going somewhere".
const MOVING: f32 = 200.0;
const NO_WARDS_AFTER: Duration = Duration::from_secs(60);
const HOLDING_WARD_AFTER: Duration = Duration::from_secs(120);
const BUYBACK_WARN_FOR: Duration = Duration::from_secs(8);
/// Approximate fountain positions, used to ignore "isolated" while at base.
const RADIANT_BASE: (f32, f32) = (-6900.0, -6400.0);
const DIRE_BASE: (f32, f32) = (6900.0, 6300.0);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TeamMode {
    Calm,
    Fighting { x: f32, y: f32 },
    Retreating,
    Grouping { x: f32, y: f32 },
}

#[derive(Default, Debug)]
pub struct CoachOut {
    /// One big instruction across the top, if any.
    pub banner: Option<(String, Severity)>,
    pub alerts: Vec<Alert>,
    /// Hide everything non-essential (fight focus).
    pub focus: bool,
    pub scale: f32,
}

#[derive(Default)]
pub struct Coach {
    fight_since: Option<Instant>,
    last_fight_at: Option<(f32, f32)>,
    isolated_since: Option<Instant>,
    no_obs_since: Option<Instant>,
    obs_held: Option<(u32, Instant)>,
    last_gold: Option<u32>,
    bb_spent_at: Option<Instant>,
}

fn dist(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

/// +1 moving toward `target`, -1 moving away, 0 standing still or sideways.
fn heading(movement: Option<(f32, f32)>, from: (f32, f32), target: (f32, f32)) -> i32 {
    let Some((dx, dy)) = movement else { return 0 };
    let len = (dx * dx + dy * dy).sqrt();
    if len < MOVING {
        return 0;
    }
    let (tx, ty) = (target.0 - from.0, target.1 - from.1);
    let tlen = (tx * tx + ty * ty).sqrt().max(1.0);
    let cos = (dx * tx + dy * ty) / (len * tlen);
    if cos > 0.5 {
        1
    } else if cos < -0.5 {
        -1
    } else {
        0
    }
}

fn recent(callouts: &[CalloutEv], kind: CalloutKind, within: Duration, now: Instant) -> Option<&CalloutEv> {
    callouts.iter().rev().find(|c| c.kind == kind && now.duration_since(c.at) < within)
}

/// What the team as a whole is doing right now.
pub fn team_mode(team: &Team, now: Instant) -> TeamMode {
    let callouts: Vec<CalloutEv> = team.callouts.iter().cloned().collect();
    let everyone: Vec<&crate::team::Mate> = team.me.iter().chain(team.mates.values()).collect();
    let dead = everyone.iter().filter(|m| m.status(now) == Status::Dead).count();
    // Same lifetime as the on-screen callout, so banner and callout agree.
    if recent(&callouts, CalloutKind::Retreat, crate::team::CALLOUT_TTL, now).is_some() || dead >= 2 {
        return TeamMode::Retreating;
    }
    let hit: Vec<(f32, f32)> = everyone.iter().filter(|m| m.under_attack(now)).filter_map(|m| m.pos()).collect();
    if !hit.is_empty() {
        let c = (
            hit.iter().map(|p| p.0).sum::<f32>() / hit.len() as f32,
            hit.iter().map(|p| p.1).sum::<f32>() / hit.len() as f32,
        );
        let involved = everyone.iter().filter_map(|m| m.pos()).filter(|p| dist(*p, c) <= FIGHT_RADIUS).count();
        if involved >= 2 {
            return TeamMode::Fighting { x: c.0, y: c.1 };
        }
    }
    if let Some(g) = recent(&callouts, CalloutKind::Group, Duration::from_secs(10), now) {
        if g.x != 0.0 || g.y != 0.0 {
            return TeamMode::Grouping { x: g.x, y: g.y };
        }
    }
    TeamMode::Calm
}

impl Coach {
    pub fn evaluate(&mut self, team: &Team, now: Instant) -> CoachOut {
        let c = &team.coaching;
        let mut out = CoachOut { scale: c.scale(), ..Default::default() };
        let Some(me_mate) = team.me.as_ref() else { return out };
        let Some(me) = me_mate.state.as_ref().filter(|s| s.in_match()) else { return out };

        let mode = team_mode(team, now);
        match mode {
            TeamMode::Fighting { x, y } => {
                self.fight_since.get_or_insert(now);
                self.last_fight_at = Some((x, y));
            }
            _ => self.fight_since = None,
        }
        let my_pos = me_mate.pos();
        let my_move = me_mate.movement();
        let my_hp = me.hp_frac();
        let me_hit = me_mate.under_attack(now);
        let fight_dist = match (mode, my_pos) {
            (TeamMode::Fighting { x, y }, Some(p)) => Some(dist(p, (x, y))),
            _ => None,
        };
        let near_fight = fight_dist.is_some_and(|d| d <= NEAR_FIGHT);

        // Team mode banner, and catching the player moving against the team.
        if c.has(Feature::TeamModeBanner) && me.alive {
            match mode {
                TeamMode::Fighting { x, y } => {
                    let against = my_pos.is_some_and(|p| near_fight && my_hp > 0.5 && heading(my_move, p, (x, y)) < 0);
                    out.banner = Some(if against {
                        ("TEAM IS FIGHTING - TURN AROUND, GO IN".into(), Severity::Danger)
                    } else {
                        ("TEAM IS FIGHTING".into(), Severity::Warn)
                    });
                }
                TeamMode::Retreating => {
                    let toward = match (my_pos, self.last_fight_at) {
                        (Some(p), Some(f)) => heading(my_move, p, f) > 0,
                        _ => false,
                    };
                    out.banner = Some(if toward {
                        ("STOP - TEAM IS RETREATING, RUN BACK".into(), Severity::Danger)
                    } else {
                        ("TEAM IS RETREATING - BACK OFF".into(), Severity::Warn)
                    });
                }
                TeamMode::Grouping { x, y } => {
                    let d = my_pos.map(|p| dist(p, (x, y))).unwrap_or(0.0);
                    if d > 1500.0 {
                        out.banner = Some((format!("TEAM IS GROUPING - go, {d:.0} away"), Severity::Warn));
                    }
                }
                TeamMode::Calm => {}
            }
        }

        // Team fighting without you.
        if c.has(Feature::JoinFightNudge) && me.alive {
            if let Some(d) = fight_dist.filter(|d| *d > NEAR_FIGHT) {
                let how = match me.tp_cooldown() {
                    Some(0) => "TP in now",
                    Some(_) => "walk in now, TP on cooldown",
                    None => "walk in now, buy a TP",
                };
                out.alerts.push(Alert { severity: Severity::Danger, text: format!("Team is fighting {d:.0} away - {how}") });
            }
        }

        // Alone, far from everyone.
        let isolated = me.alive && me.clock >= 600 && {
            match my_pos {
                Some(p) => {
                    let base = if me.team == "dire" { DIRE_BASE } else { RADIANT_BASE };
                    let nearest = team
                        .mates
                        .values()
                        .filter_map(|m| m.pos().map(|q| (dist(p, q), m.name.clone())))
                        .min_by(|a, b| a.0.total_cmp(&b.0));
                    match nearest {
                        Some((d, name)) if d > ISOLATION_DIST && dist(p, base) > 3500.0 => {
                            let since = *self.isolated_since.get_or_insert(now);
                            if c.has(Feature::IsolationWarning) && now.duration_since(since) >= ISOLATION_AFTER {
                                out.alerts.push(Alert {
                                    severity: Severity::Warn,
                                    text: format!("You're alone - nearest ally {name} is {d:.0} away"),
                                });
                            }
                            true
                        }
                        _ => false,
                    }
                }
                None => false,
            }
        };
        if !isolated {
            self.isolated_since = None;
        }

        // Short, winnable fights - and getting out of long ones.
        if c.has(Feature::EngageAdvisor) && me.alive {
            if let (TeamMode::Calm | TeamMode::Grouping { .. }, Some(p)) = (mode, my_pos) {
                let close: Vec<&crate::team::Mate> =
                    team.mates.values().filter(|m| m.pos().is_some_and(|q| dist(p, q) <= ENGAGE_RADIUS)).collect();
                let ult_ready = |s: &protocol::PlayerState| s.ultimate().is_some_and(|u| u.level > 0 && u.cooldown == 0);
                let ults = close.iter().filter_map(|m| m.state.as_ref()).filter(|s| ult_ready(s)).count()
                    + usize::from(ult_ready(me));
                if close.len() >= 2 && ults >= 2 && my_hp > 0.6 {
                    out.alerts.push(Alert {
                        severity: Severity::Good,
                        text: format!("Good fight window: {} of you close, {} ults ready - go short and fast", close.len() + 1, ults),
                    });
                }
            }
            if let Some(since) = self.fight_since {
                if near_fight && now.duration_since(since) >= FIGHT_DRAGGING && my_hp < 0.45 {
                    out.alerts.push(Alert { severity: Severity::Danger, text: "Fight is dragging - disengage now".into() });
                }
            }
        }

        if c.has(Feature::LowHpRetreat) && me.alive && my_hp < 0.35 && (me_hit || near_fight) {
            out.alerts.push(Alert { severity: Severity::Danger, text: format!("LOW HP {:.0}% - back off now", my_hp * 100.0) });
        }

        // Wards.
        let team_obs: u32 = team
            .me
            .iter()
            .chain(team.mates.values())
            .filter_map(|m| m.state.as_ref())
            .filter(|s| s.in_match())
            .map(|s| s.ward_count().0)
            .sum();
        if me.clock >= 180 && team_obs == 0 {
            let since = *self.no_obs_since.get_or_insert(now);
            if c.has(Feature::WardReminders) && now.duration_since(since) >= NO_WARDS_AFTER {
                out.alerts.push(Alert { severity: Severity::Warn, text: "No observer wards on the team - buy some".into() });
            }
        } else {
            self.no_obs_since = None;
        }
        let my_obs = me.ward_count().0;
        match self.obs_held {
            Some((n, _)) if n == my_obs && my_obs > 0 => {}
            _ => self.obs_held = (my_obs > 0).then_some((my_obs, now)),
        }
        if let Some((_, since)) = self.obs_held {
            if c.has(Feature::WardReminders) && me.alive && now.duration_since(since) >= HOLDING_WARD_AFTER {
                out.alerts.push(Alert { severity: Severity::Warn, text: "Place your observer ward".into() });
            }
        }

        // Buyback gold, late game.
        if let Some(prev) = self.last_gold {
            if me.alive && me.clock >= 1800 && prev >= me.buyback_cost && me.gold < me.buyback_cost && prev - me.gold >= 300 {
                self.bb_spent_at = Some(now);
            }
        }
        self.last_gold = Some(me.gold);
        if let Some(at) = self.bb_spent_at {
            if c.has(Feature::BuybackGuard) && now.duration_since(at) < BUYBACK_WARN_FOR {
                out.alerts.push(Alert {
                    severity: Severity::Warn,
                    text: format!("You spent below buyback gold ({}/{})", me.gold, me.buyback_cost),
                });
            }
        }

        out.focus = c.has(Feature::FightFocus) && (near_fight || me_hit);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team::Mate;
    use protocol::{Ability, Coaching, Item, PlayerState};

    fn st(x: f32, y: f32, hp: u32) -> PlayerState {
        PlayerState {
            hero: "axe".into(),
            alive: true,
            health: hp,
            max_health: 1000,
            has_pos: true,
            x,
            y,
            clock: 900,
            team: "radiant".into(),
            ..Default::default()
        }
    }

    fn team_with(features: &[Feature]) -> Team {
        let mut t = Team::new("me".into());
        t.coaching = Coaching { features: features.to_vec(), alert_scale: 0.0 };
        t
    }

    fn put(t: &mut Team, id: u32, s: PlayerState, at: Instant) {
        if id == 0 {
            t.me.get_or_insert_with(|| Mate::new(0, "me".into())).update(s, at);
        } else {
            t.mates.entry(id).or_insert_with(|| Mate::new(id, format!("m{id}"))).update(s, at);
        }
    }

    #[test]
    fn two_allies_taking_damage_is_a_team_fight() {
        let t0 = Instant::now();
        let mut t = team_with(&[]);
        put(&mut t, 0, st(0.0, 0.0, 1000), t0);
        put(&mut t, 1, st(500.0, 0.0, 1000), t0);
        let later = t0 + Duration::from_millis(500);
        put(&mut t, 1, st(500.0, 0.0, 700), later);
        assert!(matches!(team_mode(&t, later), TeamMode::Fighting { .. }));

        // A lone pick-off far from everyone is not a team fight.
        let mut t = team_with(&[]);
        put(&mut t, 0, st(0.0, 0.0, 1000), t0);
        put(&mut t, 1, st(6000.0, 0.0, 1000), t0);
        put(&mut t, 1, st(6000.0, 0.0, 700), later);
        assert_eq!(team_mode(&t, later), TeamMode::Calm);
    }

    #[test]
    fn running_away_from_a_team_fight_is_called_out() {
        let t0 = Instant::now();
        let mut t = team_with(&[Feature::TeamModeBanner, Feature::FightFocus]);
        put(&mut t, 0, st(1000.0, 0.0, 1000), t0);
        put(&mut t, 1, st(0.0, 0.0, 1000), t0);
        let t1 = t0 + Duration::from_millis(700);
        put(&mut t, 1, st(0.0, 0.0, 700), t1);
        put(&mut t, 0, st(1600.0, 0.0, 1000), t1); // moving away from the fight at x=0
        let out = Coach::default().evaluate(&t, t1);
        let (text, sev) = out.banner.expect("banner");
        assert_eq!(sev, Severity::Danger, "{text}");
        assert!(text.contains("GO IN"));
        assert!(out.focus, "fight focus while near a fight");
    }

    #[test]
    fn running_into_a_retreat_is_called_out() {
        let t0 = Instant::now();
        let mut t = team_with(&[Feature::TeamModeBanner]);
        let mut coach = Coach::default();
        put(&mut t, 0, st(2000.0, 0.0, 1000), t0);
        put(&mut t, 1, st(0.0, 0.0, 1000), t0);
        put(&mut t, 2, st(300.0, 0.0, 1000), t0);
        let t1 = t0 + Duration::from_millis(500);
        put(&mut t, 1, st(0.0, 0.0, 600), t1);
        coach.evaluate(&t, t1); // remembers where the fight was
        let t2 = t1 + Duration::from_millis(700);
        t.callouts.push_back(CalloutEv { from: "m2".into(), kind: CalloutKind::Retreat, x: 0.0, y: 0.0, at: t2 });
        put(&mut t, 0, st(1400.0, 0.0, 1000), t2); // walking toward the old fight
        let (text, sev) = coach.evaluate(&t, t2).banner.expect("banner");
        assert_eq!(sev, Severity::Danger);
        assert!(text.contains("RETREATING"), "{text}");
    }

    #[test]
    fn isolation_needs_time_and_distance() {
        let t0 = Instant::now();
        let mut t = team_with(&[Feature::IsolationWarning]);
        let mut coach = Coach::default();
        put(&mut t, 0, st(5000.0, -5000.0, 1000), t0);
        put(&mut t, 1, st(-3000.0, 2000.0, 1000), t0);
        assert!(coach.evaluate(&t, t0).alerts.is_empty(), "not immediately");
        let t1 = t0 + Duration::from_secs(9);
        put(&mut t, 0, st(5000.0, -5000.0, 1000), t1);
        put(&mut t, 1, st(-3000.0, 2000.0, 1000), t1);
        let out = coach.evaluate(&t, t1);
        assert!(out.alerts.iter().any(|a| a.text.contains("alone") && a.text.contains("m1")));
    }

    #[test]
    fn engage_window_and_feature_switch() {
        let t0 = Instant::now();
        let ult = |cd| vec![Ability { name: "r".into(), level: 1, ultimate: true, cooldown: cd, ..Default::default() }];
        let setup = |features: &[Feature]| {
            let mut t = team_with(features);
            let mut me = st(0.0, 0.0, 1000);
            me.abilities = ult(0);
            put(&mut t, 0, me, t0);
            let mut a = st(800.0, 0.0, 1000);
            a.abilities = ult(0);
            put(&mut t, 1, a, t0);
            put(&mut t, 2, st(0.0, 900.0, 1000), t0);
            t
        };
        let out = Coach::default().evaluate(&setup(&[Feature::EngageAdvisor]), t0);
        assert!(out.alerts.iter().any(|a| a.severity == Severity::Good && a.text.contains("3 of you")));
        let out = Coach::default().evaluate(&setup(&[]), t0);
        assert!(out.alerts.is_empty(), "nothing without the switch");
    }

    #[test]
    fn ward_reminders() {
        let t0 = Instant::now();
        let mut t = team_with(&[Feature::WardReminders]);
        let mut coach = Coach::default();
        put(&mut t, 0, st(0.0, 0.0, 1000), t0);
        coach.evaluate(&t, t0);
        let out = coach.evaluate(&t, t0 + Duration::from_secs(61));
        assert!(out.alerts.iter().any(|a| a.text.contains("No observer")));

        let mut s = st(0.0, 0.0, 1000);
        s.items.push(Item { name: "ward_observer".into(), charges: Some(2), cooldown: 0 });
        put(&mut t, 0, s, t0);
        let mut coach = Coach::default();
        coach.evaluate(&t, t0);
        let out = coach.evaluate(&t, t0 + Duration::from_secs(121));
        assert!(out.alerts.iter().any(|a| a.text.contains("Place your observer")));
    }
}
