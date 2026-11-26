//! Post-game analysis. Dota already shows damage, KDA and net worth, so this
//! measures what it cannot: how each player behaved *as part of the team*.
//! Fights joined or skipped, help calls answered, deaths far from everyone,
//! gold left unspent, wards carried around, and the worst moments of the game.

use crate::record::{Recording, Sample, Track};
use protocol::{hero_display, item_short, CalloutKind};
use serde::{Deserialize, Serialize};

/// Ally within this distance of a fight counts as present.
const FIGHT_RADIUS: f32 = 3000.0;
/// Nearest living ally farther than this at death = died alone.
const ALONE_DEATH: f32 = 3000.0;
/// Farther than this from every ally counts as "alone" time.
const ALONE_TIME: f32 = 4000.0;
const HP_DROP: f32 = 0.12;
const HOARD_GOLD: u32 = 2000;
const HOARD_MIN_S: i32 = 120;
const WARD_HOLD_MIN_S: i32 = 180;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Moment {
    pub t: i32,
    pub clock: String,
    pub text: String,
    /// 1 (minor) .. 4 (game-losing); for good moments, how good.
    pub weight: u32,
    pub kind: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Metrics {
    pub fights_present: u32,
    pub fights_total: u32,
    pub fights_missed_tp_ready: u32,
    pub first_to_die: u32,
    pub calls_answered: u32,
    pub calls_total: u32,
    pub avg_answer_s: f32,
    pub deaths: u32,
    pub deaths_alone: u32,
    pub deaths_with_items_ready: u32,
    pub deaths_after_retreat_call: u32,
    pub deaths_no_buyback_late: u32,
    pub gold_lost: u32,
    pub hoard_s: i32,
    pub alone_pct: f32,
    pub dead_pct: f32,
    pub no_tp_pct: f32,
    pub ward_hold_s: i32,
    pub obs_placed: u32,
    pub sen_placed: u32,
    pub kill_participation: f32,
    pub avg_team_distance: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerReport {
    pub name: String,
    pub hero: String,
    pub hero_name: String,
    pub team: String,
    pub k: u32,
    pub d: u32,
    pub a: u32,
    pub lh: u32,
    pub gpm: u32,
    pub xpm: u32,
    pub level: u32,
    pub items: Vec<String>,
    pub title: String,
    pub metrics: Metrics,
    pub worst: Vec<Moment>,
    pub best: Vec<Moment>,
    /// Ran the app this game (so position-based facts exist).
    #[serde(default = "yes")]
    pub app: bool,
    /// Plain-language facts shown on the site and given to the AI coach.
    #[serde(default)]
    pub cards: Vec<crate::facts::Card>,
    /// Facts from OpenDota's replay parse, when available.
    #[serde(default)]
    pub od: Option<crate::facts::OdFacts>,
}

fn yes() -> bool {
    true
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Fight {
    pub start: i32,
    pub end: i32,
    pub clock: String,
    pub x: f32,
    pub y: f32,
    pub present: Vec<String>,
    pub absent: Vec<String>,
    pub our_deaths: u32,
    pub our_kills: u32,
    pub result: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TimelinePoint {
    pub t: i32,
    /// Average distance of living players from the team centre.
    pub spread: f32,
    pub alive: u32,
    pub gold: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MatchReport {
    pub match_id: String,
    pub started_ms: u64,
    pub duration: i32,
    pub team: String,
    pub win_team: String,
    pub won: Option<bool>,
    pub players: Vec<PlayerReport>,
    pub fights: Vec<Fight>,
    pub timeline: Vec<TimelinePoint>,
    pub summary: String,
    /// Enemy heroes (display names), from OpenDota.
    #[serde(default)]
    pub enemies: Vec<String>,
    /// Which enemy heroes killed us most, from OpenDota.
    #[serde(default)]
    pub killers: Vec<(String, u32)>,
    /// "measured" (our positions) or "replay" (OpenDota team fights).
    #[serde(default)]
    pub fights_source: String,
    #[serde(default)]
    pub od: crate::opendota::OdStatus,
    #[serde(default)]
    pub ai_review: Option<String>,
    #[serde(default)]
    pub ai_status: String,
}

pub fn clock(t: i32) -> String {
    let s = t.max(0);
    format!("{}:{:02}", s / 60, s % 60)
}

fn km(d: f32) -> String {
    format!("{:.1}k", d / 1000.0)
}

fn dist(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

/// Latest sample at or before `t`, if it is fresh (the agent was running).
fn at(track: &Track, t: i32) -> Option<&Sample> {
    let i = track.samples.partition_point(|s| s.t <= t);
    let s = track.samples.get(i.checked_sub(1)?)?;
    (t - s.t <= 6).then_some(s)
}

fn pos(s: &Sample) -> (f32, f32) {
    (s.x, s.y)
}

fn items_list(names: &[String]) -> String {
    let v: Vec<String> = names.iter().map(|n| item_short(n)).collect();
    v.join(", ")
}

struct Ctx<'a> {
    tracks: Vec<&'a Track>,
    start: i32,
    end: i32,
}

impl<'a> Ctx<'a> {
    fn nearest_ally(&self, me: usize, t: i32, from: (f32, f32)) -> Option<f32> {
        self.tracks
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != me)
            .filter_map(|(_, tr)| at(tr, t).filter(|s| s.alive))
            .map(|s| dist(from, pos(s)))
            .reduce(f32::min)
    }
}

pub fn analyze(rec: &Recording) -> MatchReport {
    // Our side: the team most recorded players were on.
    let mut sides: Vec<(&str, usize)> = Vec::new();
    for tr in rec.tracks.values() {
        match sides.iter_mut().find(|(s, _)| *s == tr.team) {
            Some(e) => e.1 += 1,
            None => sides.push((tr.team.as_str(), 1)),
        }
    }
    sides.sort_by(|a, b| b.1.cmp(&a.1));
    let team = sides.first().map(|s| s.0.to_string()).unwrap_or_default();
    let tracks: Vec<&Track> = rec.tracks.values().filter(|t| t.team == team && !t.samples.is_empty()).collect();
    let start = tracks.iter().filter_map(|t| t.samples.first()).map(|s| s.t).min().unwrap_or(0);
    let end = rec.duration();
    let ctx = Ctx { tracks, start, end };

    // ---- fights: seconds where an ally lost >=12% HP in 3 s with another ally near, or died.
    let mut hot: Vec<(i32, f32, f32)> = Vec::new();
    for t in ctx.start..=ctx.end {
        let mut pts = Vec::new();
        for (i, tr) in ctx.tracks.iter().enumerate() {
            let (Some(now), Some(before)) = (at(tr, t), at(tr, t - 3)) else { continue };
            let died = before.alive && !now.alive;
            let hurt = now.alive && before.alive && before.hp - now.hp >= HP_DROP;
            if !(died || hurt) {
                continue;
            }
            let p = if died { pos(before) } else { pos(now) };
            let near = ctx.nearest_ally(i, t, p).is_some_and(|d| d < FIGHT_RADIUS);
            if died || near {
                pts.push(p);
            }
        }
        if !pts.is_empty() {
            let x = pts.iter().map(|p| p.0).sum::<f32>() / pts.len() as f32;
            let y = pts.iter().map(|p| p.1).sum::<f32>() / pts.len() as f32;
            hot.push((t, x, y));
        }
    }
    let mut spans: Vec<Vec<(i32, f32, f32)>> = Vec::new();
    for h in hot {
        match spans.last_mut() {
            Some(s) if h.0 - s.last().unwrap().0 <= 8 => s.push(h),
            _ => spans.push(vec![h]),
        }
    }
    let mut fights = Vec::new();
    let mut fight_firsts: Vec<Option<usize>> = Vec::new();
    for s in spans {
        let (fs, fe) = (s[0].0, s.last().unwrap().0);
        let x = s.iter().map(|p| p.1).sum::<f32>() / s.len() as f32;
        let y = s.iter().map(|p| p.2).sum::<f32>() / s.len() as f32;
        let mut present = Vec::new();
        let mut absent = Vec::new();
        let mut deaths = 0;
        let mut kills = 0;
        let mut first: Option<(i32, usize)> = None;
        for (i, tr) in ctx.tracks.iter().enumerate() {
            let (Some(a), Some(b)) = (at(tr, fs - 3), at(tr, fe + 2)) else { continue };
            deaths += b.d.saturating_sub(a.d);
            kills += b.k.saturating_sub(a.k);
            let near = (fs - 3..=fe).any(|t| at(tr, t).is_some_and(|q| q.alive && dist(pos(q), (x, y)) < FIGHT_RADIUS))
                || b.d > a.d;
            if near {
                present.push(i);
            } else if a.alive {
                absent.push(i);
            }
            if b.d > a.d {
                if let Some(td) = (fs - 3..=fe + 2).find(|&t| at(tr, t).is_some_and(|q| !q.alive)) {
                    if first.is_none_or(|(ft, _)| td < ft) {
                        first = Some((td, i));
                    }
                }
            }
        }
        // One of us alone is a pick-off (reported as "died alone"), not a team fight.
        if present.len() < 2 {
            continue;
        }
        let result = match (kills, deaths) {
            (k, d) if k > d => "won",
            (k, d) if d > k => "lost",
            (0, 0) => "standoff",
            _ => "traded",
        };
        fights.push((fs, fe, x, y, present, absent, deaths, kills, result.to_string()));
        fight_firsts.push(first.map(|f| f.1));
    }

    // ---- per player
    let team_kills: u32 = ctx.tracks.iter().filter_map(|t| t.samples.last()).map(|s| s.k).sum();
    let mut reports = Vec::new();
    for (i, tr) in ctx.tracks.iter().enumerate() {
        let mut m = Metrics::default();
        let mut worst: Vec<Moment> = Vec::new();
        let mut best: Vec<Moment> = Vec::new();
        let last = tr.samples.last().cloned().unwrap_or_default();
        let push = |v: &mut Vec<Moment>, t: i32, kind: &str, weight: u32, text: String| {
            v.push(Moment { t, clock: clock(t), text, weight, kind: kind.into() });
        };

        // Fights.
        for (fi, f) in fights.iter().enumerate() {
            let (fs, _fe, x, y, present, absent, deaths, kills, result) = f;
            if present.contains(&i) {
                m.fights_present += 1;
                m.fights_total += 1;
                if *result == "won" && *deaths == 0 && *kills >= 2 {
                    push(&mut best, *fs, "fight_won", 3, format!("Clean fight: +{kills} kills, nobody died"));
                }
                if fight_firsts[fi] == Some(i) && present.len() >= 3 {
                    m.first_to_die += 1;
                }
            } else if absent.contains(&i) {
                m.fights_total += 1;
                if let Some(s) = at(tr, *fs) {
                    let d = dist(pos(s), (*x, *y));
                    if s.tp == 0 && d > 4500.0 {
                        m.fights_missed_tp_ready += 1;
                        let outcome = match result.as_str() {
                            "lost" => format!(", team lost it ({deaths} died)"),
                            _ => String::new(),
                        };
                        push(&mut worst, *fs, "missed_fight", if *result == "lost" { 4 } else { 3 },
                            format!("Team fought {} away while your TP was ready; you stayed out{outcome}", km(d)));
                    }
                }
            }
        }

        // Deaths.
        let mut prev: Option<&Sample> = None;
        let mut alive_s = 0i32;
        let mut dead_s = 0i32;
        let mut alone_s = 0i32;
        let mut no_tp_s = 0i32;
        let mut dist_sum = 0f32;
        let mut dist_n = 0u32;
        let mut hoard_from: Option<(i32, u32)> = None;
        let mut hoards: Vec<(i32, i32, u32)> = Vec::new();
        let mut ward_from: Option<i32> = None;
        for s in &tr.samples {
            if let Some(p) = prev {
                let dt = (s.t - p.t).clamp(0, 6);
                if p.alive {
                    alive_s += dt;
                    if p.t >= 600 {
                        if let Some(d) = ctx.nearest_ally(i, p.t, pos(p)) {
                            if d > ALONE_TIME {
                                alone_s += dt;
                            }
                        }
                    }
                    if p.t >= 300 && p.tp < 0 {
                        no_tp_s += dt;
                    }
                    // Distance to the team centre.
                    let others: Vec<(f32, f32)> = ctx.tracks.iter().enumerate()
                        .filter(|(j, _)| *j != i)
                        .filter_map(|(_, o)| at(o, p.t).filter(|q| q.alive).map(pos))
                        .collect();
                    if !others.is_empty() {
                        let cx = others.iter().map(|o| o.0).sum::<f32>() / others.len() as f32;
                        let cy = others.iter().map(|o| o.1).sum::<f32>() / others.len() as f32;
                        dist_sum += dist(pos(p), (cx, cy));
                        dist_n += 1;
                    }
                } else {
                    dead_s += dt;
                }
                // Wards used: count drops while alive (selling is rare).
                if p.alive && s.alive {
                    m.obs_placed += p.obs.saturating_sub(s.obs);
                    m.sen_placed += p.sen.saturating_sub(s.sen);
                }
                if p.alive && !s.alive {
                    m.deaths += 1;
                    let lost = p.unrel.saturating_sub(s.unrel);
                    m.gold_lost += lost;
                    let mut bits = Vec::new();
                    let mut weight = 1;
                    let mut kind = "death";
                    if let Some(d) = ctx.nearest_ally(i, p.t, pos(p)) {
                        if d > ALONE_DEATH {
                            m.deaths_alone += 1;
                            bits.push(format!("alone, nearest ally {} away", km(d)));
                            weight += 2;
                            kind = "death_alone";
                        }
                    }
                    if !p.ready.is_empty() && p.hp > 0.0 {
                        m.deaths_with_items_ready += 1;
                        bits.push(format!("{} ready and unused", items_list(&p.ready)));
                        weight += 1;
                    }
                    if p.gold >= 1500 {
                        bits.push(format!("holding {} gold", p.gold));
                        weight += 1;
                    }
                    let retreat = rec.callouts.iter().any(|c| {
                        c.kind == CalloutKind::Retreat && c.from != tr.name && p.t - c.t >= 0 && p.t - c.t <= 20
                    });
                    if retreat {
                        m.deaths_after_retreat_call += 1;
                        bits.push("seconds after a RETREAT call".into());
                        weight += 2;
                        kind = "death_ignored_retreat";
                    }
                    if p.t >= 1800 && p.gold < p.bb_cost && p.bb_cd == 0 {
                        m.deaths_no_buyback_late += 1;
                        bits.push(format!("no buyback gold ({}/{})", p.gold, p.bb_cost));
                        weight += 1;
                    }
                    if !bits.is_empty() {
                        push(&mut worst, s.t, kind, weight.min(4), format!("Died {}", bits.join(", ")));
                    }
                }
            }
            // Hoarding gold while alive.
            if s.alive && s.gold >= HOARD_GOLD && s.t >= 300 {
                let e = hoard_from.get_or_insert((s.t, s.gold));
                e.1 = e.1.max(s.gold);
            } else if let Some((from, peak)) = hoard_from.take() {
                let len = s.t - from;
                if len >= HOARD_MIN_S {
                    m.hoard_s += len;
                    hoards.push((from, len, peak));
                }
            }
            // Carrying observers without placing them.
            if s.alive && s.obs > 0 {
                ward_from.get_or_insert(s.t);
            } else if let Some(from) = ward_from.take() {
                let len = s.t - from;
                if len >= WARD_HOLD_MIN_S {
                    m.ward_hold_s += len;
                    push(&mut worst, from, "ward_hold", 1, format!("Carried observer wards for {} before using them", dur(len)));
                }
            }
            prev = Some(s);
        }
        if let Some((from, peak)) = hoard_from {
            let len = last.t - from;
            if len >= HOARD_MIN_S {
                m.hoard_s += len;
                hoards.push((from, len, peak));
            }
        }
        // One line for hoarding: the longest stretch, plus the total if it kept happening.
        if let Some(&(from, len, peak)) = hoards.iter().max_by_key(|h| h.1) {
            let more = if hoards.len() > 1 {
                format!(" ({} times, {} in total)", hoards.len(), dur(m.hoard_s))
            } else {
                String::new()
            };
            push(&mut worst, from, "hoard", if m.hoard_s > 300 { 3 } else { 2 },
                format!("Sat on {peak} gold for {}{more}", dur(len)));
        }
        let total = (alive_s + dead_s).max(1) as f32;
        m.dead_pct = dead_s as f32 / total * 100.0;
        let late_alive = (alive_s - 600).max(1) as f32;
        m.alone_pct = (alone_s as f32 / late_alive * 100.0).min(100.0);
        m.no_tp_pct = (no_tp_s as f32 / (alive_s - 300).max(1) as f32 * 100.0).min(100.0);
        m.avg_team_distance = if dist_n > 0 { dist_sum / dist_n as f32 } else { 0.0 };
        m.kill_participation = if team_kills > 0 { ((last.k + last.a) as f32 / team_kills as f32 * 100.0).min(100.0) } else { 0.0 };

        // Help / group calls from others.
        let mut answer_times = Vec::new();
        for c in rec.callouts.iter().filter(|c| matches!(c.kind, CalloutKind::Help | CalloutKind::Group) && c.from != tr.name) {
            let Some(s0) = at(tr, c.t).filter(|s| s.alive) else { continue };
            let d0 = dist(pos(s0), (c.x, c.y));
            if d0 < 1500.0 || (c.x == 0.0 && c.y == 0.0) {
                continue;
            }
            m.calls_total += 1;
            let target = (d0 * 0.5).max(1500.0);
            match (c.t..=c.t + 30).find(|&t| at(tr, t).is_some_and(|s| s.alive && dist(pos(s), (c.x, c.y)) <= target)) {
                Some(t) => {
                    m.calls_answered += 1;
                    answer_times.push((t - c.t) as f32);
                    if t - c.t <= 15 && d0 > 3000.0 {
                        push(&mut best, c.t, "answered", 2,
                            format!("Answered {}'s {} call from {} away in {}s", c.from, c.kind.label(), km(d0), t - c.t));
                    }
                }
                None => push(&mut worst, c.t, "ignored_call", 2,
                    format!("Ignored {}'s {} call ({} away)", c.from, c.kind.label(), km(d0))),
            }
        }
        if !answer_times.is_empty() {
            m.avg_answer_s = answer_times.iter().sum::<f32>() / answer_times.len() as f32;
        }

        if m.obs_placed >= 6 {
            push(&mut best, last.t, "wards", 2, format!("Placed {} observer and {} sentry wards", m.obs_placed, m.sen_placed));
        }
        if m.deaths <= 2 && last.t > 1500 {
            push(&mut best, last.t, "alive", 2, format!("Only {} deaths in {}", m.deaths, clock(last.t)));
        }

        worst.sort_by(|a, b| b.weight.cmp(&a.weight).then(a.t.cmp(&b.t)));
        worst.truncate(6);
        best.sort_by(|a, b| b.weight.cmp(&a.weight).then(a.t.cmp(&b.t)));
        best.truncate(3);
        reports.push(PlayerReport {
            name: tr.name.clone(),
            hero: tr.hero.clone(),
            hero_name: hero_display(&tr.hero),
            team: tr.team.clone(),
            k: last.k,
            d: last.d,
            a: last.a,
            lh: last.lh,
            gpm: last.gpm,
            xpm: last.xpm,
            level: last.lvl,
            items: tr.items.clone(),
            title: String::new(),
            metrics: m,
            worst,
            best,
            app: true,
            cards: Vec::new(),
            od: None,
        });
    }
    assign_titles(&mut reports);
    reports.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    // Team spread over time, every 30 s.
    let mut timeline = Vec::new();
    let mut t = ctx.start.max(0);
    while t <= ctx.end {
        let alive: Vec<&Sample> = ctx.tracks.iter().filter_map(|tr| at(tr, t)).filter(|s| s.alive).collect();
        let gold = ctx.tracks.iter().filter_map(|tr| at(tr, t)).map(|s| s.gold).sum();
        let spread = if alive.len() >= 2 {
            let cx = alive.iter().map(|s| s.x).sum::<f32>() / alive.len() as f32;
            let cy = alive.iter().map(|s| s.y).sum::<f32>() / alive.len() as f32;
            alive.iter().map(|s| dist(pos(s), (cx, cy))).sum::<f32>() / alive.len() as f32
        } else {
            0.0
        };
        timeline.push(TimelinePoint { t, spread, alive: alive.len() as u32, gold });
        t += 30;
    }

    let names: Vec<String> = ctx.tracks.iter().map(|t| t.name.clone()).collect();
    let fights: Vec<Fight> = fights
        .into_iter()
        .map(|(fs, fe, x, y, present, absent, deaths, kills, result)| Fight {
            start: fs,
            end: fe,
            clock: clock(fs),
            x,
            y,
            present: present.iter().map(|&i| names[i].clone()).collect(),
            absent: absent.iter().map(|&i| names[i].clone()).collect(),
            our_deaths: deaths,
            our_kills: kills,
            result,
        })
        .collect();

    let won = (!rec.win_team.is_empty() && !team.is_empty()).then(|| rec.win_team == team);
    let fw = fights.iter().filter(|f| f.result == "won").count();
    let fl = fights.iter().filter(|f| f.result == "lost").count();
    let mut summary = summary(won, end, fights.len(), fw, fl);
    if ctx.tracks.len() < 2 {
        // Fights are found from teammates' positions; one player can't show them.
        let head = summary.split(". ").next().unwrap_or("").to_string();
        summary = format!("{head}. Team fights show up when two or more of you run the app.");
    }
    let mut report = MatchReport {
        match_id: rec.match_id.clone(),
        started_ms: rec.started_ms,
        duration: end,
        team,
        win_team: rec.win_team.clone(),
        won,
        players: reports,
        fights,
        timeline,
        summary,
        enemies: Vec::new(),
        killers: Vec::new(),
        fights_source: "measured".into(),
        od: Default::default(),
        ai_review: None,
        ai_status: String::new(),
    };
    let apps = report.players.len();
    for p in &mut report.players {
        p.cards = crate::facts::cards(p, report.fights.len() as u32, apps.saturating_sub(1));
    }
    report
}

pub fn summary(won: Option<bool>, end: i32, n: usize, fw: usize, fl: usize) -> String {
    format!(
        "{} in {}. {} team fights: {} won, {} lost.",
        match won { Some(true) => "Victory", Some(false) => "Defeat", None => "Game" },
        clock(end),
        n,
        fw,
        fl
    )
}

pub fn dur(s: i32) -> String {
    if s >= 60 {
        format!("{}m {:02}s", s / 60, s % 60)
    } else {
        format!("{s}s")
    }
}

/// One nickname per player, from what stood out most in this game.
fn assign_titles(rs: &mut [PlayerReport]) {
    type Pick = (&'static str, fn(&Metrics) -> f32, f32);
    let picks: [Pick; 9] = [
        ("Lone Wolf", |m| m.deaths_alone as f32, 2.0),
        ("Dragon on a Gold Pile", |m| m.hoard_s as f32 / 60.0, 4.0),
        ("Farming Simulator", |m| m.fights_missed_tp_ready as f32, 2.0),
        ("Saved It For Later", |m| m.deaths_with_items_ready as f32, 2.0),
        ("Selective Hearing", |m| m.deaths_after_retreat_call as f32, 1.0),
        ("First Responder", |m| m.calls_answered as f32, 2.0),
        ("Lighthouse", |m| m.obs_placed as f32, 8.0),
        ("Always There", |m| if m.fights_total >= 4 { m.fights_present as f32 / m.fights_total as f32 * 10.0 } else { 0.0 }, 8.5),
        // Not dying only counts if you were actually in the fights.
        ("Unkillable", |m| {
            let present = m.fights_total == 0 || m.fights_present * 3 >= m.fights_total * 2;
            if m.deaths <= 1 && present { 5.0 } else { 0.0 }
        }, 5.0),
    ];
    let mut used = Vec::new();
    for r in rs.iter_mut() {
        let best = picks
            .iter()
            .filter(|p| !used.contains(&p.0))
            .map(|p| (p.0, (p.1)(&r.metrics) / p.2))
            .filter(|(_, v)| *v >= 1.0)
            .max_by(|a, b| a.1.total_cmp(&b.1));
        r.title = match best {
            Some((t, _)) => {
                used.push(t);
                t.to_string()
            }
            None => "Background Character".into(),
        };
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::record::{CalloutEvent, Recording, Track};

    fn sample(t: i32, x: f32, y: f32) -> Sample {
        Sample { t, alive: true, hp: 1.0, x, y, tp: 0, gold: 500, ..Default::default() }
    }

    /// Three players: A and B fight together at 700 s; C farms far away with
    /// TP ready, later dies alone holding gold and ignores a help call.
    pub fn scenario() -> Recording {
        let mut tracks = std::collections::BTreeMap::new();
        let mut a = Track { name: "A".into(), hero: "lina".into(), team: "radiant".into(), ..Default::default() };
        let mut b = Track { name: "B".into(), hero: "lich".into(), team: "radiant".into(), ..Default::default() };
        let mut c = Track { name: "C".into(), hero: "juggernaut".into(), team: "radiant".into(), ..Default::default() };
        for t in 0..=1200 {
            let mut sa = sample(t, 0.0, 0.0);
            let mut sb = sample(t, 500.0, 0.0);
            let mut sc = sample(t, 6000.0, 6000.0);
            if (700..710).contains(&t) {
                sa.hp = 1.0 - (t - 700) as f32 * 0.08;
                sb.hp = 1.0 - (t - 700) as f32 * 0.05;
            }
            if t >= 710 {
                sa.k = 2;
                sb.a = 2;
            }
            if (900..1100).contains(&t) {
                sc.gold = 2600;
            }
            if t >= 1100 {
                sc.alive = t > 1140;
                sc.d = 1;
                sc.gold = 300;
            }
            a.samples.push(sa);
            b.samples.push(sb);
            c.samples.push(sc);
        }
        tracks.insert("A".into(), a);
        tracks.insert("B".into(), b);
        tracks.insert("C".into(), c);
        Recording {
            match_id: "1".into(),
            win_team: "radiant".into(),
            tracks,
            callouts: vec![CalloutEvent { t: 1000, from: "A".into(), kind: CalloutKind::Help, x: 0.0, y: 100.0 }],
            ..Default::default()
        }
    }

    #[test]
    fn finds_team_behaviour() {
        let r = analyze(&scenario());
        assert_eq!(r.won, Some(true));
        assert_eq!(r.fights.len(), 1, "{:?}", r.fights);
        let f = &r.fights[0];
        assert_eq!(f.result, "won");
        assert!(f.absent.contains(&"C".to_string()));
        let c = r.players.iter().find(|p| p.name == "C").unwrap();
        assert_eq!(c.metrics.fights_missed_tp_ready, 1);
        assert_eq!(c.metrics.deaths_alone, 1);
        assert_eq!(c.metrics.calls_total, 1);
        assert_eq!(c.metrics.calls_answered, 0);
        assert!(c.metrics.hoard_s >= 120);
        assert!(c.worst.iter().any(|m| m.kind == "missed_fight"));
        assert!(c.worst.iter().any(|m| m.text.contains("alone")));
        assert!(!c.title.is_empty());
        // Plain-language facts instead of a score.
        assert!(c.cards.iter().any(|k| k.id == "skipped_tp" && k.num == 1.0), "{:?}", c.cards);
        assert!(c.cards.iter().any(|k| k.id == "fights" && k.tone == "bad"), "{:?}", c.cards);
        let a = r.players.iter().find(|p| p.name == "A").unwrap();
        assert!(a.cards.iter().any(|k| k.id == "fights" && k.tone == "good"), "{:?}", a.cards);
    }
}
