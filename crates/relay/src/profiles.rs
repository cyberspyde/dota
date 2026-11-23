//! Private per-player coaching profiles, kept only on the host.
//!
//! team.toml (never committed; see examples/team.example.toml):
//!
//!   [[player]]
//!   names = ["Ilhom"]                 # matched case-insensitively to the agent's name
//!   features = ["engage_advisor", "isolation_warning"]
//!   alert_scale = 1.0
//!   notes = "anything; never sent to anyone"
//!
//! Only `features` and `alert_scale` for the connecting player are sent, and
//! only to that player.

use protocol::{Coaching, Feature};
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct File {
    #[serde(default)]
    player: Vec<Profile>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Profile {
    names: Vec<String>,
    #[serde(default)]
    features: Vec<Feature>,
    #[serde(default)]
    alert_scale: f32,
    /// Host-only notes. Parsed so typos elsewhere are caught, never used.
    #[serde(default)]
    #[allow(dead_code)]
    notes: String,
}

pub fn load(path: &Path) -> Result<Vec<(Vec<String>, Coaching)>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let file: File = toml::from_str(&text).map_err(|e| format!("parse {}: {e}", path.display()))?;
    for p in &file.player {
        if p.features.contains(&Feature::Unknown) {
            return Err(format!("{}: unknown feature name for {:?}", path.display(), p.names));
        }
    }
    Ok(file
        .player
        .into_iter()
        .map(|p| {
            (
                p.names.iter().map(|n| n.trim().to_lowercase()).collect(),
                Coaching { features: p.features, alert_scale: p.alert_scale },
            )
        })
        .collect())
}

/// Coaching for `name`, re-reading the file so edits apply on the next connect.
pub fn for_player(path: Option<&Path>, name: &str) -> Coaching {
    let Some(path) = path else { return Coaching::default() };
    match load(path) {
        Ok(list) => {
            let key = name.trim().to_lowercase();
            list.into_iter()
                .find(|(names, _)| names.contains(&key))
                .map(|(_, c)| c)
                .unwrap_or_default()
        }
        Err(e) => {
            tracing::warn!("team profiles not applied: {e}");
            Coaching::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_any_alias_and_rejects_typos() {
        let dir = std::env::temp_dir().join(format!("relay-profiles-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let good = dir.join("team.toml");
        std::fs::write(
            &good,
            "[[player]]\nnames = [\"PlayerTwo\", \"p2\"]\nfeatures = [\"fight_focus\"]\nalert_scale = 1.3\nnotes = \"private\"\n",
        )
        .unwrap();
        let c = for_player(Some(&good), " P2 ");
        assert_eq!(c.features, vec![Feature::FightFocus]);
        assert_eq!(c.alert_scale, 1.3);
        assert!(for_player(Some(&good), "someone").features.is_empty());

        let bad = dir.join("bad.toml");
        std::fs::write(&bad, "[[player]]\nnames = [\"x\"]\nfeatures = [\"fight_fokus\"]\n").unwrap();
        assert!(load(&bad).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
