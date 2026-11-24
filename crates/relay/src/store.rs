//! Match history on disk: one report and one raw recording per match.
//!
//!   <data>/matches/<match_id>.json      analysed report (what the site shows)
//!   <data>/matches/<match_id>.rec.json  raw recording (for re-analysis)

use crate::{analysis::MatchReport, record::Recording};
use serde::Serialize;
use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};

#[derive(Serialize, Clone, Debug)]
pub struct Summary {
    pub match_id: String,
    pub started_ms: u64,
    pub duration: i32,
    pub won: Option<bool>,
    pub summary: String,
    pub players: Vec<SummaryPlayer>,
    pub enemies: Vec<String>,
    pub has_ai: bool,
    pub od: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct SummaryPlayer {
    pub name: String,
    pub hero: String,
    pub hero_name: String,
    pub title: String,
    pub app: bool,
    /// The fight-attendance fact ("3 of 7"), the first thing friends look at.
    pub fights: String,
    /// The most telling bad fact, if any.
    pub low: Option<String>,
}

fn summarize(r: &MatchReport) -> Summary {
    Summary {
        match_id: r.match_id.clone(),
        started_ms: r.started_ms,
        duration: r.duration,
        won: r.won,
        summary: r.summary.clone(),
        has_ai: r.ai_review.is_some(),
        enemies: r.enemies.clone(),
        od: r.od.state.clone(),
        players: r
            .players
            .iter()
            .map(|p| SummaryPlayer {
                name: p.name.clone(),
                hero: p.hero.clone(),
                hero_name: p.hero_name.clone(),
                title: p.title.clone(),
                app: p.app,
                fights: p
                    .cards
                    .iter()
                    .find(|c| c.id == "fights")
                    .and_then(|c| c.text.strip_prefix("Showed up to ").map(|s| s.replace(" team fights", "")))
                    .unwrap_or_default(),
                low: p.cards.iter().find(|c| c.tone == "bad").map(|c| c.text.clone()),
            })
            .collect(),
    }
}

pub struct Store {
    dir: PathBuf,
    index: Mutex<Vec<Summary>>,
}

/// Match ids are digits from Dota; anything else is refused before it can
/// touch a file path.
pub fn valid_id(id: &str) -> bool {
    !id.is_empty() && id.len() <= 24 && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
}

impl Store {
    pub fn open(data: &Path) -> Result<Store, String> {
        let dir = data.join("matches");
        std::fs::create_dir_all(&dir).map_err(|e| format!("create {}: {e}", dir.display()))?;
        let mut index = Vec::new();
        if let Ok(rd) = std::fs::read_dir(&dir) {
            for e in rd.flatten() {
                let p = e.path();
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if !name.ends_with(".json") || name.ends_with(".rec.json") || name.ends_with(".od.json") {
                    continue;
                }
                match std::fs::read_to_string(&p).ok().and_then(|t| serde_json::from_str::<MatchReport>(&t).ok()) {
                    Some(r) => index.push(summarize(&r)),
                    None => tracing::warn!("skipping unreadable report {}", p.display()),
                }
            }
        }
        index.sort_by(|a: &Summary, b: &Summary| b.started_ms.cmp(&a.started_ms));
        Ok(Store { dir, index: Mutex::new(index) })
    }

    pub fn list(&self) -> Vec<Summary> {
        self.index.lock().unwrap().clone()
    }

    pub fn get(&self, id: &str) -> Option<MatchReport> {
        if !valid_id(id) {
            return None;
        }
        let text = std::fs::read_to_string(self.dir.join(format!("{id}.json"))).ok()?;
        serde_json::from_str(&text).ok()
    }

    /// Full reports, newest first (for trends and "usually").
    pub fn history(&self, limit: usize) -> Vec<MatchReport> {
        let ids: Vec<String> = self.list().into_iter().take(limit).map(|s| s.match_id).collect();
        ids.iter().filter_map(|id| self.get(id)).collect()
    }

    pub fn get_recording(&self, id: &str) -> Option<Recording> {
        if !valid_id(id) {
            return None;
        }
        let text = std::fs::read_to_string(self.dir.join(format!("{id}.rec.json"))).ok()?;
        serde_json::from_str(&text).ok()
    }

    /// OpenDota's parsed match, kept so reports can be rebuilt later.
    pub fn save_opendota(&self, id: &str, m: &crate::opendota::Match) -> Result<(), String> {
        if !valid_id(id) {
            return Err(format!("bad match id {id:?}"));
        }
        write_atomic(&self.dir.join(format!("{id}.od.json")), &serde_json::to_vec(m).map_err(|e| e.to_string())?)
    }

    pub fn get_opendota(&self, id: &str) -> Option<crate::opendota::Match> {
        if !valid_id(id) {
            return None;
        }
        let text = std::fs::read_to_string(self.dir.join(format!("{id}.od.json"))).ok()?;
        serde_json::from_str(&text).ok()
    }

    pub fn save_recording(&self, rec: &Recording) -> Result<(), String> {
        if !valid_id(&rec.match_id) {
            return Err(format!("bad match id {:?}", rec.match_id));
        }
        write_atomic(&self.dir.join(format!("{}.rec.json", rec.match_id)), &serde_json::to_vec(rec).map_err(|e| e.to_string())?)
    }

    pub fn save(&self, r: &MatchReport) -> Result<(), String> {
        if !valid_id(&r.match_id) {
            return Err(format!("bad match id {:?}", r.match_id));
        }
        let body = serde_json::to_vec_pretty(r).map_err(|e| e.to_string())?;
        write_atomic(&self.dir.join(format!("{}.json", r.match_id)), &body)?;
        let mut idx = self.index.lock().unwrap();
        idx.retain(|s| s.match_id != r.match_id);
        idx.push(summarize(r));
        idx.sort_by(|a, b| b.started_ms.cmp(&a.started_ms));
        Ok(())
    }
}

fn write_atomic(path: &Path, body: &[u8]) -> Result<(), String> {
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, body).map_err(|e| format!("write {}: {e}", tmp.display()))?;
    std::fs::rename(&tmp, path).map_err(|e| format!("rename {}: {e}", path.display()))
}
