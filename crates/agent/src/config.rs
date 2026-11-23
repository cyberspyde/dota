use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hasher},
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct Config {
    /// Name your teammates see.
    pub name: String,
    /// Relay WebSocket URL, e.g. ws://203.0.113.10:4747/ws
    pub relay: String,
    /// Team key; must match the relay's --key.
    pub key: String,
    /// Local port Dota posts game state to.
    pub gsi_port: u16,
    /// Shared secret between Dota and this agent (written into the GSI cfg).
    pub gsi_token: String,
    /// Optional override, e.g. "D:/SteamLibrary/steamapps/common/dota 2 beta".
    pub dota_path: String,
    /// Preferred build mood from the build database when a hero has several.
    pub build_mood: String,
    pub overlay: OverlayConfig,
    pub hotkeys: HotkeyConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct OverlayConfig {
    pub enabled: bool,
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    /// Panel background opacity, 0.0-1.0.
    pub opacity: f32,
    pub show_radar: bool,
    pub show_build: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct HotkeyConfig {
    pub help: String,
    pub group: String,
    pub retreat: String,
    pub careful: String,
    pub on_my_way: String,
    pub roshan: String,
    /// Toggles move mode (overlay becomes draggable, position is saved).
    pub edit_mode: String,
    /// Hides / shows the overlay.
    pub toggle: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: std::env::var("USERNAME").unwrap_or_else(|_| "player".into()),
            relay: "ws://127.0.0.1:4747/ws".into(),
            key: String::new(),
            gsi_port: 4646,
            gsi_token: random_token(),
            dota_path: String::new(),
            build_mood: String::new(),
            overlay: OverlayConfig::default(),
            hotkeys: HotkeyConfig::default(),
        }
    }
}

impl Default for OverlayConfig {
    fn default() -> Self {
        OverlayConfig {
            enabled: true,
            x: 16.0,
            y: 200.0,
            scale: 1.0,
            opacity: 0.72,
            show_radar: true,
            show_build: true,
        }
    }
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        // Numpad keys are unbound in Dota by default, so registering them as
        // global hotkeys does not steal a game key.
        HotkeyConfig {
            help: "Numpad1".into(),
            group: "Numpad2".into(),
            retreat: "Numpad3".into(),
            careful: "Numpad4".into(),
            on_my_way: "Numpad5".into(),
            roshan: "Numpad6".into(),
            edit_mode: "Numpad0".into(),
            toggle: "NumpadDecimal".into(),
        }
    }
}

fn random_token() -> String {
    // RandomState is seeded from the OS RNG; two rounds give 128 bits.
    let mut out = String::new();
    for _ in 0..2 {
        let mut h = RandomState::new().build_hasher();
        h.write_u128(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos());
        out.push_str(&format!("{:016x}", h.finish()));
    }
    out
}

pub fn default_path() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("agent.toml")))
        .unwrap_or_else(|| PathBuf::from("agent.toml"))
}

/// Loads the config, creating it with defaults if missing. Returns whether it was created.
pub fn load_or_create(path: &Path) -> Result<(Config, bool), String> {
    if path.exists() {
        let text = std::fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
        let cfg: Config = toml::from_str(&text).map_err(|e| format!("parse {}: {e}", path.display()))?;
        Ok((cfg, false))
    } else {
        let cfg = Config::default();
        save(path, &cfg)?;
        Ok((cfg, true))
    }
}

pub fn save(path: &Path, cfg: &Config) -> Result<(), String> {
    let body = toml::to_string_pretty(cfg).map_err(|e| e.to_string())?;
    let text = format!(
        "# cyberspyde-agent settings. Restart the agent after editing.\n\
         # name  = what teammates see\n\
         # relay = ws://<host-ip>:4747/ws from the team host\n\
         # key   = the team key the host gave you\n\
         # Hotkey names: Numpad0-9, F1-F12, KeyA, Digit1, with modifiers like \"ctrl+shift+KeyH\".\n\n{body}"
    );
    std::fs::write(path, text).map_err(|e| format!("write {}: {e}", path.display()))
}
