use crate::config::BotConfig;
use crate::ui::emoji::EmojiType;
use neobabu_core::config::Config;
use neobabu_core::NeobabuCore;
use poise::serenity_prelude::{EmojiId, ReactionType};
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct BotState {
    pub config: Arc<BotConfig>,
    pub core: Arc<NeobabuCore>,
}

impl BotState {
    pub async fn initialize() -> anyhow::Result<Self> {
        info!("Initializing bot state...");

        info!("Loading bot config...");
        let config = BotConfig::load_from_env()?;
        info!("Bot config loaded");

        info!("Loading core config...");
        let core_config = load_core_config(&config)?;
        info!("Core config loaded");

        info!("Initializing core...");
        let core = NeobabuCore::initialize(core_config).await?;
        info!("Core initialized");

        let state = Self {
            config: Arc::new(config),
            core: Arc::new(core),
        };

        info!("Bot state initialized");

        Ok(state)
    }

    pub fn get_emoji(&self, emoji: EmojiType) -> ReactionType {
        let name = emoji.name();
        let id = self.config.emojis.id(name).unwrap_or(&0u64);
        ReactionType::Custom {
            animated: false,
            id: EmojiId::new(*id),
            name: Some(name.to_string()),
        }
    }

    pub fn get_emoji_text(&self, emoji: EmojiType) -> String {
        let id = self.config.emojis.id(emoji.name()).unwrap_or(&0u64);
        format!("<:{}:{id}>", emoji.name())
    }
}

fn load_core_config(bot_config: &BotConfig) -> anyhow::Result<Config> {
    let db_url = std::env::var("DATABASE_URL")?;
    Ok(Config {
        db_url,
        nasa_api_key: bot_config.credentials.nasa_api.clone(),
    })
}
