//! Finds the Dota 2 install and writes the Game State Integration config.

use std::path::{Path, PathBuf};

pub const CFG_NAME: &str = "gamestate_integration_cyberspyde.cfg";

fn steam_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    #[cfg(windows)]
    {
        use winreg::{enums::HKEY_CURRENT_USER, RegKey};
        if let Ok(k) = RegKey::predef(HKEY_CURRENT_USER).open_subkey("Software\\Valve\\Steam") {
            if let Ok(p) = k.get_value::<String, _>("SteamPath") {
                roots.push(PathBuf::from(p));
            }
        }
        roots.push(PathBuf::from("C:\\Program Files (x86)\\Steam"));
    }
    #[cfg(not(windows))]
    if let Some(home) = std::env::var_os("HOME") {
        roots.push(PathBuf::from(home).join(".steam/steam"));
    }
    roots
}

/// Library folders listed in steamapps/libraryfolders.vdf.
fn libraries(root: &Path) -> Vec<PathBuf> {
    let mut libs = vec![root.to_path_buf()];
    if let Ok(text) = std::fs::read_to_string(root.join("steamapps").join("libraryfolders.vdf")) {
        for line in text.lines() {
            let parts: Vec<&str> = line.split('"').collect();
            // Lines look like:   "path"		"D:\\SteamLibrary"
            if parts.len() >= 4 && parts[1] == "path" {
                libs.push(PathBuf::from(parts[3].replace("\\\\", "\\")));
            }
        }
    }
    libs
}

pub fn find_dota(override_path: &str) -> Option<PathBuf> {
    if !override_path.is_empty() {
        let p = PathBuf::from(override_path);
        return p.join("game").join("dota").is_dir().then_some(p);
    }
    for root in steam_roots() {
        for lib in libraries(&root) {
            let p = lib.join("steamapps").join("common").join("dota 2 beta");
            if p.join("game").join("dota").is_dir() {
                return Some(p);
            }
        }
    }
    None
}

pub fn gsi_cfg(port: u16, token: &str) -> String {
    // throttle 0.05 = Dota may post every 50 ms; buffer 0 = no batching delay.
    format!(
        r#""cyberspyde-dota2"
{{
    "uri"           "http://127.0.0.1:{port}/"
    "timeout"       "1.0"
    "buffer"        "0.0"
    "throttle"      "0.05"
    "heartbeat"     "5.0"
    "data"
    {{
        "provider"      "1"
        "map"           "1"
        "player"        "1"
        "hero"          "1"
        "abilities"     "1"
        "items"         "1"
    }}
    "auth"
    {{
        "token"         "{token}"
    }}
}}
"#
    )
}

/// Writes (or refreshes) the cfg. Returns the path and whether it changed.
pub fn install(dota: &Path, port: u16, token: &str) -> std::io::Result<(PathBuf, bool)> {
    let dir = dota.join("game").join("dota").join("cfg").join("gamestate_integration");
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(CFG_NAME);
    let body = gsi_cfg(port, token);
    if std::fs::read_to_string(&path).ok().as_deref() == Some(body.as_str()) {
        return Ok((path, false));
    }
    std::fs::write(&path, body)?;
    Ok((path, true))
}

#[cfg(test)]
mod tests {
    #[test]
    fn cfg_has_token_and_port() {
        let c = super::gsi_cfg(4646, "abc");
        assert!(c.contains("http://127.0.0.1:4646/"));
        assert!(c.contains("\"abc\""));
        assert!(c.contains("\"buffer\"        \"0.0\""));
    }
}
