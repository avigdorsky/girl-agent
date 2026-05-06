//! Pre-flight checks: Node + npm presence, version constraints, network reach.

use std::process::Command;

#[derive(Debug, Clone)]
pub struct PreflightReport {
    pub node_path: Option<String>,
    pub node_version: Option<String>,
    pub node_ok: bool,
    pub npm_path: Option<String>,
    pub npm_version: Option<String>,
}

pub fn run() -> PreflightReport {
    let node_path = which::which("node").ok().map(|p| p.display().to_string());
    let npm_path = which::which("npm").ok().map(|p| p.display().to_string());

    let node_version = node_path.as_ref().and_then(|p| {
        Command::new(p)
            .arg("-v")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
    });
    let node_ok = matches!(&node_version, Some(v) if parse_major(v).unwrap_or(0) >= 20);

    let npm_version = npm_path.as_ref().and_then(|p| {
        Command::new(p)
            .arg("-v")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
    });

    PreflightReport {
        node_path,
        node_version,
        node_ok,
        npm_path,
        npm_version,
    }
}

fn parse_major(v: &str) -> Option<u32> {
    // Node prints `v20.10.0`.
    let stripped = v.trim_start_matches('v');
    let major = stripped.split('.').next()?;
    major.parse().ok()
}
