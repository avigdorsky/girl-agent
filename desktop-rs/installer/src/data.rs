//! Hard-coded option lists mirrored from `src/presets/*.ts`.
//!
//! The installer doesn't need the full preset metadata — only enough to show
//! a label in a dropdown and emit the right `presetId` into the config file.
//! When the bot starts, it looks the preset up by id and pulls in the rest.

#[derive(Debug, Clone, Copy)]
pub struct Choice {
    pub id: &'static str,
    pub label: &'static str,
    pub hint: &'static str,
}

pub const LLM_PRESETS: &[Choice] = &[
    Choice { id: "openai", label: "OpenAI", hint: "ChatGPT API · нужен ключ" },
    Choice { id: "anthropic", label: "Anthropic", hint: "Claude · нужен ключ" },
    Choice { id: "openrouter", label: "OpenRouter", hint: "много моделей одним ключом" },
    Choice { id: "deepseek", label: "DeepSeek", hint: "дёшево, нужен ключ" },
    Choice { id: "groq", label: "Groq", hint: "быстро, нужен ключ" },
    Choice { id: "google", label: "Google AI", hint: "Gemini · нужен ключ" },
    Choice { id: "xai", label: "xAI (Grok)", hint: "нужен ключ" },
    Choice { id: "ollama", label: "Ollama (локально)", hint: "ключ не нужен" },
    Choice { id: "lmstudio", label: "LM Studio (локально)", hint: "ключ не нужен" },
];

pub const STAGES: &[Choice] = &[
    Choice { id: "met-irl-got-tg", label: "встретились ирл, дала тг", hint: "самый ранний этап" },
    Choice { id: "tg-given-cold", label: "тг есть, ещё холодно", hint: "пишет нечасто, осторожна" },
    Choice { id: "tg-given-warming", label: "тг, теплеет", hint: "интерес растёт" },
    Choice { id: "convinced", label: "уговорил/а", hint: "согласилась встретиться" },
    Choice { id: "first-date-done", label: "первое свидание было", hint: "увиделись" },
    Choice { id: "dating-early", label: "встречаемся, ранний этап", hint: "несколько недель" },
    Choice { id: "dating-stable", label: "стабильные отношения", hint: "" },
    Choice { id: "long-term", label: "долгие отношения", hint: "" },
];

pub const COMMUNICATION: &[Choice] = &[
    Choice { id: "normal", label: "обычное", hint: "среднее по больнице" },
    Choice { id: "cute", label: "милая", hint: "мягкая, нежная" },
    Choice { id: "alt", label: "альт", hint: "сухо, без лишнего" },
    Choice { id: "clingy", label: "прилипчивая", hint: "пишет первой" },
    Choice { id: "chatty", label: "болтливая", hint: "длинные сообщения" },
];

pub const MODES: &[Choice] = &[
    Choice { id: "bot", label: "бот", hint: "Telegram Bot API, проще" },
    Choice { id: "userbot", label: "юзербот", hint: "MTProto, нужны api id/hash" },
];

pub fn slugify(name: &str) -> String {
    let lower: String = name.trim().to_lowercase();
    let mut out = String::new();
    let mut prev_dash = false;
    for ch in lower.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            prev_dash = false;
        } else if !out.is_empty() && !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.is_empty() {
        out = "persona".to_string();
    }
    out
}
