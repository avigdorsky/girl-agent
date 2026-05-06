//! Final install step: write `config.json` + `npm i -g @thesashadev/girl-agent`
//! + create Start Menu / Desktop shortcuts on Windows.

use std::path::PathBuf;
use std::process::{Command, Stdio};

use anyhow::{anyhow, Context, Result};
use girl_agent_shared::paths;
use serde_json::json;

use crate::config::WizardData;

pub struct InstallReport {
    pub config_path: PathBuf,
    pub npm_log: String,
    pub npm_ok: bool,
}

pub fn run(data: &WizardData) -> Result<InstallReport> {
    // 1) Write profile config.
    let data_root = paths::data_dir();
    let profile_dir = data_root.join(&data.slug);
    std::fs::create_dir_all(&profile_dir)
        .with_context(|| format!("create {}", profile_dir.display()))?;

    let cfg = build_config_json(data);
    let config_path = profile_dir.join("config.json");
    std::fs::write(&config_path, serde_json::to_string_pretty(&cfg)?)?;

    // 2) Persist last-profile setting.
    let mut s = girl_agent_shared::settings::Settings::load();
    s.last_profile = Some(data.slug.clone());
    s.save().ok();

    // 3) Install npm package globally.
    let mut npm_log = String::new();
    let npm_ok = run_npm_install(&mut npm_log).is_ok();

    Ok(InstallReport {
        config_path,
        npm_log,
        npm_ok,
    })
}

fn build_config_json(d: &WizardData) -> serde_json::Value {
    let now = chrono_now();

    let telegram = if d.mode == "bot" {
        json!({ "token": d.tg_token })
    } else {
        json!({
            "apiId": d.tg_api_id,
            "apiHash": d.tg_api_hash,
            "phone": d.tg_phone,
        })
    };

    json!({
        "slug": d.slug,
        "name": d.name,
        "age": d.age,
        "nationality": d.nationality,
        "tz": d.tz,
        "mode": d.mode,
        "stage": d.stage,
        "communicationPreset": d.communication,
        "createdAt": now,
        "llm": {
            "presetId": d.llm_preset,
            "proto": "openai",
            "baseURL": null,
            "apiKey": d.llm_api_key,
            "model": d.llm_model,
        },
        "telegram": telegram,
        "vibe": "",
        "personaNotes": "",
        "notifications": "normal",
        "privacy": "owner-only",
    })
}

fn chrono_now() -> String {
    chrono::Utc::now().to_rfc3339()
}

fn run_npm_install(log: &mut String) -> Result<()> {
    let npm = which::which(if cfg!(target_os = "windows") { "npm.cmd" } else { "npm" })
        .or_else(|_| which::which("npm"))
        .map_err(|_| anyhow!("npm not found in PATH"))?;
    log.push_str(&format!("$ {} install -g @thesashadev/girl-agent\n", npm.display()));
    let output = Command::new(&npm)
        .arg("install")
        .arg("-g")
        .arg("@thesashadev/girl-agent")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("invoke npm install")?;
    log.push_str(&String::from_utf8_lossy(&output.stdout));
    log.push_str(&String::from_utf8_lossy(&output.stderr));
    if !output.status.success() {
        return Err(anyhow!("npm install exited with {}", output.status));
    }
    Ok(())
}
