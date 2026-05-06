//! girl-agent installer (iced wizard).
//!
//! Walks the user through pre-flight, Telegram credentials, LLM provider,
//! persona basics, then runs `npm install -g @thesashadev/girl-agent` and
//! writes a profile config. The desktop app picks the new profile up
//! automatically on next launch.

mod config;
mod data;
mod install;
mod preflight;
mod ui;

use std::path::PathBuf;

use girl_agent_shared::fonts;
use iced::Task;

use crate::config::WizardData;
use crate::ui::{InstallOutcome, Msg, Step};

fn main() -> iced::Result {
    init_tracing();

    iced::application("girl-agent installer", App::update, App::view)
        .theme(|_| girl_agent_shared::theme::iced_theme())
        .font(fonts::UNBOUNDED_REGULAR_TTF)
        .font(fonts::UNBOUNDED_BOLD_TTF)
        .font(fonts::ONEST_REGULAR_TTF)
        .font(fonts::ONEST_MEDIUM_TTF)
        .font(fonts::ONEST_BOLD_TTF)
        .font(fonts::JETBRAINS_MONO_TTF)
        .font(fonts::INSTRUMENT_SERIF_ITALIC_TTF)
        .default_font(fonts::ONEST)
        .window(window_settings())
        .run_with(App::new)
}

struct App {
    model: ui::Model,
}

impl App {
    fn new() -> (Self, Task<Msg>) {
        let preflight = preflight::run();
        let model = ui::Model {
            step: Step::Welcome,
            data: WizardData::default(),
            preflight,
            install: None,
            installing: false,
        };
        (Self { model }, Task::none())
    }

    fn view(&self) -> iced::Element<'_, Msg> {
        ui::view(&self.model)
    }

    fn update(&mut self, msg: Msg) -> Task<Msg> {
        match msg {
            Msg::Next => {
                self.model.step = next_step(self.model.step);
                if self.model.step == Step::Summary {
                    self.model.data.refresh_slug();
                }
                Task::none()
            }
            Msg::Back => {
                self.model.step = prev_step(self.model.step);
                Task::none()
            }
            Msg::NameChanged(v) => { self.model.data.name = v; self.model.data.refresh_slug(); Task::none() }
            Msg::AgeChanged(v) => { self.model.data.age = v; Task::none() }
            Msg::NationalityChanged(v) => { self.model.data.nationality = v; Task::none() }
            Msg::TzChanged(v) => { self.model.data.tz = v; Task::none() }
            Msg::StageChanged(v) => { self.model.data.stage = v; Task::none() }
            Msg::CommunicationChanged(v) => { self.model.data.communication = v; Task::none() }
            Msg::ModeChanged(v) => { self.model.data.mode = v; Task::none() }
            Msg::TgTokenChanged(v) => { self.model.data.tg_token = v; Task::none() }
            Msg::TgApiIdChanged(v) => { self.model.data.tg_api_id = v; Task::none() }
            Msg::TgApiHashChanged(v) => { self.model.data.tg_api_hash = v; Task::none() }
            Msg::TgPhoneChanged(v) => { self.model.data.tg_phone = v; Task::none() }
            Msg::LlmPresetChanged(v) => { self.model.data.llm_preset = v; Task::none() }
            Msg::LlmModelChanged(v) => { self.model.data.llm_model = v; Task::none() }
            Msg::LlmKeyChanged(v) => { self.model.data.llm_api_key = v; Task::none() }
            Msg::StartInstall => {
                self.model.installing = true;
                self.model.step = Step::Installing;
                let data = self.model.data.clone();
                Task::perform(
                    async move {
                        tokio::task::spawn_blocking(move || install::run(&data))
                            .await
                            .unwrap_or_else(|e| Err(anyhow::anyhow!("join: {e}")))
                    },
                    |res| {
                        Msg::InstallFinished(InstallOutcome {
                            ok: res.is_ok(),
                            log: match &res {
                                Ok(r) => r.npm_log.clone(),
                                Err(e) => e.to_string(),
                            },
                            config_path: match &res {
                                Ok(r) => r.config_path.display().to_string(),
                                Err(_) => String::new(),
                            },
                        })
                    },
                )
            }
            Msg::InstallFinished(o) => {
                self.model.installing = false;
                self.model.install = Some(o);
                self.model.step = Step::Done;
                Task::none()
            }
            Msg::LaunchAndQuit => {
                let _ = launch_desktop_app();
                std::process::exit(0);
            }
            Msg::Quit => {
                std::process::exit(0);
            }
            Msg::OpenLink(url) => {
                let _ = open::that_in_background(url);
                Task::none()
            }
        }
    }
}

fn next_step(s: Step) -> Step {
    match s {
        Step::Welcome => Step::Preflight,
        Step::Preflight => Step::Telegram,
        Step::Telegram => Step::Llm,
        Step::Llm => Step::Persona,
        Step::Persona => Step::Summary,
        Step::Summary => Step::Installing,
        Step::Installing => Step::Done,
        Step::Done => Step::Done,
    }
}

fn prev_step(s: Step) -> Step {
    match s {
        Step::Welcome => Step::Welcome,
        Step::Preflight => Step::Welcome,
        Step::Telegram => Step::Preflight,
        Step::Llm => Step::Telegram,
        Step::Persona => Step::Llm,
        Step::Summary => Step::Persona,
        Step::Installing | Step::Done => Step::Summary,
    }
}

fn window_settings() -> iced::window::Settings {
    iced::window::Settings {
        size: iced::Size::new(720.0, 640.0),
        min_size: Some(iced::Size::new(640.0, 540.0)),
        ..iced::window::Settings::default()
    }
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(false)
        .try_init();
}

fn launch_desktop_app() -> std::io::Result<()> {
    let exe = std::env::current_exe()?;
    let dir = exe.parent().unwrap_or(std::path::Path::new("."));
    let candidate: PathBuf = if cfg!(target_os = "windows") {
        dir.join("girl-agent-desktop.exe")
    } else {
        dir.join("girl-agent-desktop")
    };
    if candidate.exists() {
        std::process::Command::new(candidate).spawn().map(|_| ())
    } else {
        // Best effort — at least open the data directory so the user knows
        // where things landed.
        let _ = open::that_in_background(girl_agent_shared::paths::data_dir());
        Ok(())
    }
}
