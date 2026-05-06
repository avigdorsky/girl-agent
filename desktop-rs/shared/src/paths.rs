//! OS-aware paths for app data, settings, profile data and bundled binaries.
//!
//! On Windows the canonical layout is:
//!
//! ```text
//! %APPDATA%/girl-agent/
//!     settings.json          // tray choice, ports, last-used profile
//!     data/                  // bot's GIRL_AGENT_DATA root, one folder per profile
//!     log/
//! ```
//!
//! On Linux/macOS we fall back to the `directories` XDG-style locations.

use std::path::PathBuf;

use directories::ProjectDirs;

/// Application identifier used for `directories`.
const QUALIFIER: &str = "com";
const ORG: &str = "TheSashaDev";
const APP: &str = "girl-agent";

fn project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from(QUALIFIER, ORG, APP)
}

/// `%APPDATA%/girl-agent` on Windows, `~/.config/girl-agent` on Linux,
/// `~/Library/Application Support/com.TheSashaDev.girl-agent` on macOS.
pub fn app_dir() -> PathBuf {
    project_dirs()
        .map(|d| d.config_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Where profile markdown / JSON live. Passed to the bot via
/// `GIRL_AGENT_DATA`.
pub fn data_dir() -> PathBuf {
    let p = app_dir().join("data");
    let _ = std::fs::create_dir_all(&p);
    p
}

/// Path to `settings.json`.
pub fn settings_path() -> PathBuf {
    let _ = std::fs::create_dir_all(app_dir());
    app_dir().join("settings.json")
}

/// Path used by the desktop app for its own logs (separate from per-profile
/// chat logs).
pub fn app_log_dir() -> PathBuf {
    let p = app_dir().join("log");
    let _ = std::fs::create_dir_all(&p);
    p
}
