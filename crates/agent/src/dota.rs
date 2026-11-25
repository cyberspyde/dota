//! Finds the Dota 2 install and writes the Game State Integration config.

use std::{path::{Path, PathBuf}, time::Duration};

pub const CFG_NAME: &str = "gamestate_integration_cyberspyde.cfg";

/// Connection state shown by both the compact overlay and the settings menu.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeedStatus {
    Live,
    WaitingForMatch,
    NotRunning,
    Unknown,
}

impl FeedStatus {
    pub fn from_observations(age: Option<Duration>, in_match: bool, process: Option<bool>) -> Self {
        if age.is_some_and(|age| age < Duration::from_secs(20)) {
            // A menu heartbeat confirms that Dota is open, not that a match started.
            return if in_match { Self::Live } else { Self::WaitingForMatch };
        }
        match process {
            Some(true) => Self::WaitingForMatch,
            Some(false) => Self::NotRunning,
            None => Self::Unknown,
        }
    }

    pub fn text(self) -> &'static str {
        match self {
            Self::Live => "Dota is sending game data",
            Self::WaitingForMatch => "Waiting for a match or Hero Demo",
            Self::NotRunning => "Dota 2 isn't running",
            Self::Unknown => "Waiting for Dota game data",
        }
    }
}

#[cfg(any(windows, test))]
fn is_dota_executable(name: &[u16]) -> bool {
    name.iter()
        .copied()
        .take_while(|&ch| ch != 0)
        .map(|ch| if (65..=90).contains(&ch) { ch + 32 } else { ch })
        .eq(b"dota2.exe".iter().copied().map(u16::from))
}

/// Read the process list without opening Dota or querying its memory. Compare
/// Windows sessions so a different logged-in user does not affect this overlay.
#[cfg(windows)]
pub fn is_running() -> Option<bool> {
    use windows_sys::Win32::{
        Foundation::{CloseHandle, GetLastError, ERROR_NO_MORE_FILES, INVALID_HANDLE_VALUE},
        System::{
            Diagnostics::ToolHelp::{
                CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
                TH32CS_SNAPPROCESS,
            },
            RemoteDesktop::ProcessIdToSessionId,
        },
    };

    // SAFETY: use a valid snapshot handle and properly sized process entry.
    // Both session ID outputs are writable u32 values; the handle is closed once.
    unsafe {
        let mut own_session = 0;
        if ProcessIdToSessionId(std::process::id(), &mut own_session) == 0 {
            return None;
        }
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            return None;
        }
        let mut entry: PROCESSENTRY32W = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
        let mut found = Process32FirstW(snapshot, &mut entry);
        let mut unknown_session = false;
        let result = loop {
            if found == 0 {
                break (GetLastError() == ERROR_NO_MORE_FILES && !unknown_session).then_some(false);
            }
            if is_dota_executable(&entry.szExeFile) {
                let mut session = 0;
                if ProcessIdToSessionId(entry.th32ProcessID, &mut session) == 0 {
                    unknown_session = true;
                } else if session == own_session {
                    break Some(true);
                }
            }
            found = Process32NextW(snapshot, &mut entry);
        };
        CloseHandle(snapshot);
        result
    }
}

#[cfg(not(windows))]
pub fn is_running() -> Option<bool> {
    None
}

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
    fn feed_status_tracks_launch_data_expiry_and_exit() {
        use super::{Duration, FeedStatus};

        let status = FeedStatus::from_observations;
        assert_eq!(status(None, false, Some(false)).text(), "Dota 2 isn't running");
        assert_eq!(status(None, false, Some(true)).text(), "Waiting for a match or Hero Demo");
        assert_eq!(status(Some(Duration::from_secs(5)), true, Some(true)), FeedStatus::Live);
        assert_eq!(status(Some(Duration::from_millis(19_999)), true, Some(false)), FeedStatus::Live);
        assert_eq!(status(Some(Duration::from_secs(20)), true, Some(true)), FeedStatus::WaitingForMatch);
        assert_eq!(status(Some(Duration::from_secs(21)), true, Some(false)), FeedStatus::NotRunning);
        assert_eq!(status(Some(Duration::from_secs(5)), true, None), FeedStatus::Live);
        assert_eq!(status(Some(Duration::from_secs(5)), false, None), FeedStatus::WaitingForMatch);
        assert_eq!(status(Some(Duration::from_secs(21)), true, None), FeedStatus::Unknown);
    }

    #[test]
    fn process_name_must_match_dota_not_steam_or_another_helper() {
        use super::is_dota_executable;

        let encode = |name: &str| name.encode_utf16().collect::<Vec<_>>();
        assert!(is_dota_executable(&encode("DOTA2.EXE\0\0")));
        assert!(!is_dota_executable(&encode("steam.exe\0")));
        assert!(!is_dota_executable(&encode("dota2-helper.exe\0")));
    }

    #[test]
    fn cfg_has_token_and_port() {
        let c = super::gsi_cfg(4646, "abc");
        assert!(c.contains("http://127.0.0.1:4646/"));
        assert!(c.contains("\"abc\""));
        assert!(c.contains("\"buffer\"        \"0.0\""));
    }
}
