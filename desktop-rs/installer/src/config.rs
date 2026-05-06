//! Wizard form state.

use crate::data::slugify;

#[derive(Debug, Clone)]
pub struct WizardData {
    // Basics
    pub name: String,
    pub age: u32,
    pub nationality: String,
    pub tz: String,
    pub slug: String,
    pub stage: String,
    pub communication: String,

    // Telegram
    pub mode: String, // "bot" or "userbot"
    pub tg_token: String,
    pub tg_api_id: String,
    pub tg_api_hash: String,
    pub tg_phone: String,

    // LLM
    pub llm_preset: String,
    pub llm_model: String,
    pub llm_api_key: String,
}

impl Default for WizardData {
    fn default() -> Self {
        Self {
            name: "".into(),
            age: 21,
            nationality: "ru".into(),
            tz: "Europe/Moscow".into(),
            slug: "".into(),
            stage: "tg-given-cold".into(),
            communication: "normal".into(),
            mode: "bot".into(),
            tg_token: "".into(),
            tg_api_id: "".into(),
            tg_api_hash: "".into(),
            tg_phone: "".into(),
            llm_preset: "openai".into(),
            llm_model: "".into(),
            llm_api_key: "".into(),
        }
    }
}

impl WizardData {
    pub fn refresh_slug(&mut self) {
        self.slug = slugify(&self.name);
    }

    pub fn is_basics_valid(&self) -> bool {
        !self.name.trim().is_empty() && self.age > 0 && self.age < 80
    }

    pub fn is_tg_valid(&self) -> bool {
        match self.mode.as_str() {
            "bot" => !self.tg_token.trim().is_empty(),
            "userbot" => {
                !self.tg_api_id.trim().is_empty()
                    && !self.tg_api_hash.trim().is_empty()
                    && !self.tg_phone.trim().is_empty()
            }
            _ => false,
        }
    }

    pub fn is_llm_valid(&self) -> bool {
        let preset_ok = !self.llm_preset.trim().is_empty();
        let local = self.llm_preset == "ollama" || self.llm_preset == "lmstudio";
        preset_ok && (local || !self.llm_api_key.trim().is_empty())
    }
}
