//! Iced UI for the wizard. Five panels (welcome → preflight → telegram → llm →
//! persona → summary / install) controlled by [`Step`].

use girl_agent_shared::fonts::{instrument_italic, onest_bold, onest_medium, JETBRAINS, ONEST};
use girl_agent_shared::theme::{ACCENT, ACCENT2, BONE, BONE2, INK, LINE, MUTED, RADIUS_MD};
use iced::widget::{
    button, column, container, pick_list, row, scrollable, slider, text, text_input, Space,
};
use iced::{Alignment, Background, Border, Element, Length, Padding};

use crate::config::WizardData;
use crate::data::{Choice, COMMUNICATION, LLM_PRESETS, MODES, STAGES};
use crate::preflight::PreflightReport;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Welcome,
    Preflight,
    Telegram,
    Llm,
    Persona,
    Summary,
    Installing,
    Done,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Next,
    Back,
    NameChanged(String),
    AgeChanged(u32),
    NationalityChanged(String),
    TzChanged(String),
    StageChanged(String),
    CommunicationChanged(String),
    ModeChanged(String),
    TgTokenChanged(String),
    TgApiIdChanged(String),
    TgApiHashChanged(String),
    TgPhoneChanged(String),
    LlmPresetChanged(String),
    LlmModelChanged(String),
    LlmKeyChanged(String),
    StartInstall,
    InstallFinished(InstallOutcome),
    LaunchAndQuit,
    Quit,
    OpenLink(&'static str),
}

#[derive(Debug, Clone)]
pub struct InstallOutcome {
    pub ok: bool,
    pub log: String,
    pub config_path: String,
}

pub struct Model {
    pub step: Step,
    pub data: WizardData,
    pub preflight: PreflightReport,
    pub install: Option<InstallOutcome>,
    pub installing: bool,
}

pub fn view(model: &Model) -> Element<'_, Msg> {
    let body = match model.step {
        Step::Welcome => welcome(),
        Step::Preflight => preflight_step(&model.preflight),
        Step::Telegram => telegram_step(&model.data),
        Step::Llm => llm_step(&model.data),
        Step::Persona => persona_step(&model.data),
        Step::Summary => summary_step(&model.data),
        Step::Installing => installing_step(model),
        Step::Done => done_step(model),
    };

    let nav_row = match model.step {
        Step::Welcome => row![Space::with_width(Length::Fill), nav_next("начать")]
            .align_y(Alignment::Center),
        Step::Preflight if !model.preflight.node_ok => row![
            nav_back(),
            Space::with_width(Length::Fill),
            link_button("установить Node 20+", "https://nodejs.org/"),
        ]
        .align_y(Alignment::Center),
        Step::Summary => row![
            nav_back(),
            Space::with_width(Length::Fill),
            primary_button("установить", Msg::StartInstall),
        ]
        .align_y(Alignment::Center),
        Step::Installing => row![Space::with_width(Length::Fill)].align_y(Alignment::Center),
        Step::Done => row![
            ghost("выйти", Msg::Quit),
            Space::with_width(Length::Fill),
            primary_button("запустить и выйти", Msg::LaunchAndQuit),
        ]
        .align_y(Alignment::Center),
        _ => row![nav_back(), Space::with_width(Length::Fill), nav_next("дальше")]
            .align_y(Alignment::Center),
    };

    let inner = column![scrollable(body).height(Length::Fill), nav_row]
        .spacing(20)
        .padding(Padding::from([28, 32]));

    container(column![header(model), inner].spacing(0))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| iced::widget::container::Style {
            background: Some(Background::Color(BONE)),
            ..Default::default()
        })
        .into()
}

fn header(model: &Model) -> Element<'_, Msg> {
    let title = row![
        text("girl-agent").font(onest_bold()).size(20).color(INK),
        text("/ installer").font(instrument_italic()).color(MUTED).size(15),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let progress = row![
        step_dot(model.step, Step::Welcome),
        step_dot(model.step, Step::Preflight),
        step_dot(model.step, Step::Telegram),
        step_dot(model.step, Step::Llm),
        step_dot(model.step, Step::Persona),
        step_dot(model.step, Step::Summary),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    container(
        row![title, Space::with_width(Length::Fill), progress]
            .align_y(Alignment::Center)
            .padding(Padding::from([14, 24])),
    )
    .style(|_| iced::widget::container::Style {
        background: Some(Background::Color(BONE)),
        border: Border { color: LINE, width: 1.0, radius: 0.0.into() },
        ..Default::default()
    })
    .width(Length::Fill)
    .into()
}

fn step_dot(current: Step, slot: Step) -> Element<'static, Msg> {
    let bg = if step_index(current) >= step_index(slot) { ACCENT } else { LINE };
    container(Space::new(8, 8))
        .style(move |_| iced::widget::container::Style {
            background: Some(Background::Color(bg)),
            border: Border { color: bg, width: 0.0, radius: 4.0.into() },
            ..Default::default()
        })
        .into()
}

fn step_index(s: Step) -> u8 {
    match s {
        Step::Welcome => 0,
        Step::Preflight => 1,
        Step::Telegram => 2,
        Step::Llm => 3,
        Step::Persona => 4,
        Step::Summary | Step::Installing | Step::Done => 5,
    }
}

fn welcome() -> Element<'static, Msg> {
    let title = text("girl-agent").font(onest_bold()).size(48).color(INK);
    let tag = text("AI-персона для Telegram, в одном окне")
        .font(instrument_italic())
        .color(MUTED)
        .size(20);
    let body = text("Этот мастер поставит desktop-приложение с дашбордом, веб-интерфейсом по локальному адресу и системным треем. Никаких терминалов и npx после установки.")
        .font(ONEST)
        .size(15)
        .color(INK);
    column![title, tag, Space::with_height(20), card(body.into())]
        .spacing(8)
        .into()
}

fn preflight_step(report: &PreflightReport) -> Element<'static, Msg> {
    let head = text("проверка системы").font(onest_bold()).size(24);

    let node_status = if report.node_ok {
        format!(
            "Node.js {} ✔ {}",
            report.node_version.clone().unwrap_or_default(),
            report.node_path.clone().unwrap_or_default()
        )
    } else {
        format!(
            "Node.js нужен v20+ — найдено: {}",
            report.node_version.clone().unwrap_or_else(|| "не установлен".into())
        )
    };

    let npm_status = match (&report.npm_version, &report.npm_path) {
        (Some(v), Some(p)) => format!("npm {} ✔ {}", v, p),
        _ => "npm не найден — обычно ставится вместе с Node.js".to_string(),
    };

    column![
        head,
        Space::with_height(8),
        card(
            column![
                text(node_status).font(JETBRAINS).size(13),
                Space::with_height(6),
                text(npm_status).font(JETBRAINS).size(13),
                Space::with_height(8),
                text("если что-то отсутствует — поставьте через nodejs.org и перезапустите инсталлер.")
                    .font(ONEST)
                    .color(MUTED)
                    .size(13),
            ]
            .into()
        ),
    ]
    .spacing(8)
    .into()
}

fn telegram_step<'a>(d: &'a WizardData) -> Element<'a, Msg> {
    let head = text("Telegram").font(onest_bold()).size(24);
    let mode_pick = pick_list(
        labels(MODES),
        Some(label_of(MODES, &d.mode)),
        |l| Msg::ModeChanged(id_of(MODES, &l).to_string()),
    )
    .placeholder("режим");

    let inputs = if d.mode == "bot" {
        column![
            field("bot token", &d.tg_token, "123456:ABC-...", Msg::TgTokenChanged),
            text("Создайте бота в @BotFather и вставьте токен сюда.")
                .font(ONEST)
                .color(MUTED)
                .size(12),
        ]
        .spacing(6)
    } else {
        column![
            field("api id", &d.tg_api_id, "12345678", Msg::TgApiIdChanged),
            field("api hash", &d.tg_api_hash, "abcdef…", Msg::TgApiHashChanged),
            field("phone", &d.tg_phone, "+7900…", Msg::TgPhoneChanged),
            text("Получите api id/hash на my.telegram.org → API development tools.")
                .font(ONEST)
                .color(MUTED)
                .size(12),
        ]
        .spacing(6)
    };

    column![
        head,
        Space::with_height(4),
        card(
            column![
                row![
                    text("режим").font(onest_medium()).size(14),
                    Space::with_width(12),
                    mode_pick,
                ]
                .align_y(Alignment::Center)
                .spacing(8),
                Space::with_height(12),
                inputs,
            ]
            .spacing(10)
            .into()
        ),
    ]
    .spacing(8)
    .into()
}

fn llm_step<'a>(d: &'a WizardData) -> Element<'a, Msg> {
    let head = text("LLM-провайдер").font(onest_bold()).size(24);
    let local = d.llm_preset == "ollama" || d.llm_preset == "lmstudio";
    let pick = pick_list(
        labels(LLM_PRESETS),
        Some(label_of(LLM_PRESETS, &d.llm_preset)),
        |l| Msg::LlmPresetChanged(id_of(LLM_PRESETS, &l).to_string()),
    );

    let key_field = if local {
        text("Локальный провайдер — ключ не нужен.").font(ONEST).color(MUTED).size(13).into()
    } else {
        field("api key", &d.llm_api_key, "sk-…", Msg::LlmKeyChanged)
    };

    column![
        head,
        Space::with_height(4),
        card(
            column![
                row![
                    text("provider").font(onest_medium()).size(14),
                    Space::with_width(12),
                    pick,
                ]
                .align_y(Alignment::Center)
                .spacing(8),
                Space::with_height(8),
                field("model (опц.)", &d.llm_model, "по умолчанию из preset'а", Msg::LlmModelChanged),
                Space::with_height(8),
                key_field,
            ]
            .spacing(8)
            .into()
        ),
    ]
    .spacing(8)
    .into()
}

fn persona_step<'a>(d: &'a WizardData) -> Element<'a, Msg> {
    let head = text("персона").font(onest_bold()).size(24);
    let stage_pick = pick_list(
        labels(STAGES),
        Some(label_of(STAGES, &d.stage)),
        |l| Msg::StageChanged(id_of(STAGES, &l).to_string()),
    );
    let comm_pick = pick_list(
        labels(COMMUNICATION),
        Some(label_of(COMMUNICATION, &d.communication)),
        |l| Msg::CommunicationChanged(id_of(COMMUNICATION, &l).to_string()),
    );

    column![
        head,
        Space::with_height(4),
        card(
            column![
                field("имя", &d.name, "Маша", Msg::NameChanged),
                Space::with_height(8),
                row![
                    text("возраст").font(onest_medium()).size(14).width(Length::Fixed(80.0)),
                    slider(18..=40_u32, d.age, Msg::AgeChanged).width(Length::Fill),
                    text(format!("{}", d.age)).font(JETBRAINS).size(14).width(Length::Fixed(40.0)),
                ]
                .align_y(Alignment::Center)
                .spacing(10),
                Space::with_height(8),
                field("nationality", &d.nationality, "ru / by / ua / …", Msg::NationalityChanged),
                Space::with_height(8),
                field("timezone", &d.tz, "Europe/Moscow", Msg::TzChanged),
                Space::with_height(8),
                row![
                    text("стадия").font(onest_medium()).size(14).width(Length::Fixed(80.0)),
                    stage_pick,
                ]
                .align_y(Alignment::Center)
                .spacing(10),
                Space::with_height(8),
                row![
                    text("стиль").font(onest_medium()).size(14).width(Length::Fixed(80.0)),
                    comm_pick,
                ]
                .align_y(Alignment::Center)
                .spacing(10),
            ]
            .spacing(4)
            .into()
        ),
    ]
    .spacing(8)
    .into()
}

fn summary_step<'a>(d: &'a WizardData) -> Element<'a, Msg> {
    let head = text("готово к установке").font(onest_bold()).size(24);
    let age_str = d.age.to_string();
    let body = column![
        kv_owned("имя", d.name.clone()),
        kv_owned("slug", d.slug.clone()),
        kv_owned("возраст", age_str),
        kv_owned("стадия", d.stage.clone()),
        kv_owned("стиль", d.communication.clone()),
        kv_owned("режим", d.mode.clone()),
        kv_owned("LLM", d.llm_preset.clone()),
    ]
    .spacing(4);
    let note = text(
        "Сейчас установится npm-пакет @thesashadev/girl-agent и будет создан профиль. \
         После этого откроется десктоп-приложение с дашбордом."
    )
    .font(ONEST)
    .color(MUTED)
    .size(13);

    column![head, Space::with_height(4), card(body.into()), note]
        .spacing(8)
        .into()
}

fn installing_step(model: &Model) -> Element<'_, Msg> {
    let head = text("устанавливаем…").font(onest_bold()).size(24);
    let body: Element<'_, Msg> = match &model.install {
        Some(o) => {
            let status = if o.ok {
                text("готово").font(onest_medium()).size(15).color(ACCENT2)
            } else {
                text("ошибка").font(onest_medium()).size(15).color(ACCENT)
            };
            column![status, Space::with_height(6), text(&o.log).font(JETBRAINS).size(11).color(MUTED)]
                .into()
        }
        None => text("npm install -g @thesashadev/girl-agent…")
            .font(JETBRAINS)
            .size(13)
            .into(),
    };
    column![head, Space::with_height(4), card(body)].spacing(8).into()
}

fn done_step(model: &Model) -> Element<'_, Msg> {
    let head = text("всё готово").font(onest_bold()).size(28);
    let body = match &model.install {
        Some(o) if o.ok => column![
            text("профиль создан и пакет установлен")
                .font(ONEST)
                .size(14)
                .color(INK),
            Space::with_height(6),
            text(format!("config: {}", o.config_path)).font(JETBRAINS).size(11).color(MUTED),
            Space::with_height(10),
            text("кликнуть «запустить и выйти», чтобы открыть дашборд")
                .font(instrument_italic())
                .size(15)
                .color(MUTED),
        ],
        Some(o) => column![
            text("установка не завершилась").font(ONEST).size(14).color(ACCENT),
            Space::with_height(6),
            text(&o.log).font(JETBRAINS).size(11).color(MUTED),
        ],
        None => column![text("?")],
    };
    column![head, Space::with_height(8), card(body.into())].spacing(8).into()
}

// ── reusable ────────────────────────────────────────────────────────────────

fn card<'a>(content: Element<'a, Msg>) -> Element<'a, Msg> {
    container(content)
        .padding(Padding::from([18, 22]))
        .width(Length::Fill)
        .style(|_| iced::widget::container::Style {
            background: Some(Background::Color(BONE2)),
            border: Border { color: LINE, width: 1.0, radius: 14.0.into() },
            ..Default::default()
        })
        .into()
}

fn field<'a>(
    label: &'a str,
    value: &'a str,
    placeholder: &'a str,
    on: impl 'a + Fn(String) -> Msg,
) -> Element<'a, Msg> {
    column![
        text(label).font(onest_medium()).size(13).color(MUTED),
        text_input(placeholder, value)
            .on_input(on)
            .padding(Padding::from([10, 12]))
            .size(14)
            .font(JETBRAINS)
            .style(|_, status| iced::widget::text_input::Style {
                background: Background::Color(BONE),
                border: Border {
                    color: match status {
                        iced::widget::text_input::Status::Focused { .. } => ACCENT,
                        _ => LINE,
                    },
                    width: 1.0,
                    radius: RADIUS_MD.into(),
                },
                icon: INK,
                placeholder: MUTED,
                value: INK,
                selection: ACCENT2,
            }),
    ]
    .spacing(2)
    .into()
}

fn kv<'a>(k: &'a str, v: &'a str) -> Element<'a, Msg> {
    row![
        text(k).font(ONEST).color(MUTED).size(13).width(Length::Fixed(110.0)),
        text(v).font(JETBRAINS).color(INK).size(13),
    ]
    .into()
}

fn kv_owned(k: &'static str, v: String) -> Element<'static, Msg> {
    row![
        text(k).font(ONEST).color(MUTED).size(13).width(Length::Fixed(110.0)),
        text(v).font(JETBRAINS).color(INK).size(13),
    ]
    .into()
}

fn nav_next(label: &str) -> Element<'static, Msg> {
    primary_button(label, Msg::Next)
}

fn nav_back() -> Element<'static, Msg> {
    ghost("назад", Msg::Back)
}

fn primary_button(label: &str, msg: Msg) -> Element<'static, Msg> {
    let label_owned = label.to_string();
    button(text(label_owned).font(onest_medium()).size(14))
        .padding(Padding::from([10, 18]))
        .on_press(msg)
        .style(|_, status| {
            let bg = match status {
                button::Status::Hovered | button::Status::Pressed => INK,
                _ => ACCENT,
            };
            let fg = match status {
                button::Status::Hovered | button::Status::Pressed => BONE,
                _ => INK,
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: fg,
                border: Border { color: bg, width: 0.0, radius: RADIUS_MD.into() },
                ..Default::default()
            }
        })
        .into()
}

fn ghost(label: &str, msg: Msg) -> Element<'static, Msg> {
    let label_owned = label.to_string();
    button(text(label_owned).font(ONEST).size(14))
        .padding(Padding::from([10, 18]))
        .on_press(msg)
        .style(|_, status| {
            let (bg, fg) = match status {
                button::Status::Hovered | button::Status::Pressed => (BONE2, INK),
                _ => (BONE, INK),
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: fg,
                border: Border { color: LINE, width: 1.0, radius: RADIUS_MD.into() },
                ..Default::default()
            }
        })
        .into()
}

fn link_button(label: &str, url: &'static str) -> Element<'static, Msg> {
    let label_owned = label.to_string();
    button(text(label_owned).font(ONEST).size(14))
        .padding(Padding::from([10, 18]))
        .on_press(Msg::OpenLink(url))
        .style(|_, _| button::Style {
            background: Some(Background::Color(ACCENT)),
            text_color: INK,
            border: Border { color: ACCENT, width: 0.0, radius: RADIUS_MD.into() },
            ..Default::default()
        })
        .into()
}

// ── helpers for pick_list with Choice list ──────────────────────────────────

fn labels(list: &[Choice]) -> Vec<String> {
    list.iter().map(|c| c.label.to_string()).collect()
}

fn label_of(list: &[Choice], id: &str) -> String {
    list.iter()
        .find(|c| c.id == id)
        .map(|c| c.label.to_string())
        .unwrap_or_else(|| id.to_string())
}

fn id_of<'a>(list: &'a [Choice], label: &str) -> &'a str {
    list.iter()
        .find(|c| c.label == label)
        .map(|c| c.id)
        .unwrap_or("")
}
