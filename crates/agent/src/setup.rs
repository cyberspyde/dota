//! First-launch setup in the console: a few questions instead of editing
//! agent.toml by hand. Runs automatically when there is no team key yet,
//! and again with `--setup`.

use crate::config::Config;
use std::io::{self, BufRead, Write};

/// Overlay width in logical pixels at scale 1.0 (keep in sync with overlay.rs).
pub const OVERLAY_W: f32 = 244.0;

/// What people type -> relay URL.
/// - a domain ("dota.cyberspyde.uz") -> wss://domain/ws (public, TLS edge)
/// - an IP, localhost or host:port -> ws://host:4747/ws (LAN or test relay)
/// - full ws:// or wss:// URLs pass through.
pub fn relay_url(input: &str) -> String {
    let s = input.trim().trim_end_matches('/');
    if s.starts_with("ws://") || s.starts_with("wss://") {
        return s.to_string();
    }
    let host = s.strip_prefix("http://").or_else(|| s.strip_prefix("https://")).unwrap_or(s);
    let is_ip = host.parse::<std::net::IpAddr>().is_ok();
    if host.contains(':') {
        format!("ws://{host}/ws")
    } else if is_ip || host == "localhost" {
        format!("ws://{host}:4747/ws")
    } else {
        format!("wss://{host}/ws")
    }
}

/// Host part of a relay URL, for showing as the default answer.
fn relay_host(url: &str) -> String {
    let s = url.trim_start_matches("ws://").trim_start_matches("wss://");
    let s = s.trim_end_matches("/ws");
    s.strip_suffix(":4747").unwrap_or(s).to_string()
}

fn ask(prompt: &str, default: &str) -> String {
    if default.is_empty() {
        print!("{prompt}: ");
    } else {
        print!("{prompt} [{default}]: ");
    }
    let _ = io::stdout().flush();
    let mut line = String::new();
    match io::stdin().lock().read_line(&mut line) {
        Ok(0) | Err(_) => default.to_string(), // no console input: keep defaults
        Ok(_) => {
            let t = line.trim();
            if t.is_empty() {
                default.to_string()
            } else {
                t.to_string()
            }
        }
    }
}

fn ask_choice(prompt: &str, options: &[&str], default: usize) -> usize {
    println!("{prompt}");
    for (i, o) in options.iter().enumerate() {
        println!("  {}) {o}", i + 1);
    }
    loop {
        let a = ask("Choose", &(default + 1).to_string());
        match a.parse::<usize>() {
            Ok(n) if (1..=options.len()).contains(&n) => return n - 1,
            _ => println!("Type a number from 1 to {}.", options.len()),
        }
    }
}

fn ask_yes(prompt: &str, default: bool) -> bool {
    let a = ask(&format!("{prompt} (y/n)"), if default { "y" } else { "n" });
    matches!(a.to_lowercase().as_str(), "y" | "yes" | "ha" | "1")
}

fn screen_width() -> f32 {
    #[cfg(windows)]
    {
        use windows_sys::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN};
        // This console process is not DPI-aware, so this is the logical
        // (scaled) width - the same units the overlay position uses.
        let w = unsafe { GetSystemMetrics(SM_CXSCREEN) };
        if w > 0 {
            return w as f32;
        }
    }
    1920.0
}

/// Asks the questions and updates `cfg`. Returns false if nothing usable was entered.
pub fn run(cfg: &mut Config) -> bool {
    println!();
    println!("=== cyberspyde agent setup ===");
    println!("Press Enter to keep the value in [brackets].");
    println!();

    let name = loop {
        let n = ask("Your name (exactly as your team knows you)", &cfg.name);
        if !n.trim().is_empty() {
            break n;
        }
    };
    cfg.name = name.trim().chars().take(24).collect();

    let host = ask("Team host (press Enter for the team server)", &relay_host(&cfg.relay));
    if !host.trim().is_empty() {
        cfg.relay = relay_url(&host);
    }

    loop {
        let key = ask("Team key (from the host)", &cfg.key);
        if !key.trim().is_empty() {
            cfg.key = key.trim().to_string();
            break;
        }
        println!("The team key is required; ask the host.");
    }

    println!();
    let size = ask_choice("Overlay size:", &["Small", "Normal", "Large"], match cfg.overlay.scale {
        s if s < 0.95 => 0,
        s if s > 1.1 => 2,
        _ => 1,
    });
    cfg.overlay.scale = [0.85, 1.0, 1.2][size];

    let side = ask_choice(
        "Overlay position (you can drag it later with the move key, Numpad 0):",
        &["Left side", "Right side"],
        usize::from(cfg.overlay.x > screen_width() / 2.0),
    );
    cfg.overlay.y = 160.0;
    cfg.overlay.x = if side == 0 { 12.0 } else { screen_width() - OVERLAY_W * cfg.overlay.scale - 12.0 };

    cfg.overlay.show_build = ask_yes("Show next-item help for your hero?", cfg.overlay.show_build);
    cfg.overlay.show_radar = ask_yes("Show the mini radar?", cfg.overlay.show_radar);

    println!();
    println!("Saved. Host: {}  Name: {}", cfg.relay, cfg.name);
    println!("Run with --setup any time to change these.");
    println!();
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relay_addresses() {
        assert_eq!(relay_url("203.0.113.10"), "ws://203.0.113.10:4747/ws");
        assert_eq!(relay_url(" team.example.com:9000 "), "ws://team.example.com:9000/ws");
        assert_eq!(relay_url("http://1.2.3.4/"), "ws://1.2.3.4:4747/ws");
        assert_eq!(relay_url("wss://relay.example.com/ws"), "wss://relay.example.com/ws");
        assert_eq!(relay_host("ws://203.0.113.10:4747/ws"), "203.0.113.10");
        assert_eq!(relay_host("ws://h:9000/ws"), "h:9000");
        assert_eq!(relay_url("dota.cyberspyde.uz"), "wss://dota.cyberspyde.uz/ws");
        assert_eq!(relay_url("localhost"), "ws://localhost:4747/ws");
        assert_eq!(relay_host("wss://dota.cyberspyde.uz/ws"), "dota.cyberspyde.uz");
    }
}
