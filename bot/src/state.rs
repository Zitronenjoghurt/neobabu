use crate::config::BotConfig;
use crate::error::BotResult;
use crate::ui::emoji::EmojiType;
use neobabu_core::config::Config;
use neobabu_core::rendering::o2d::prelude::O2DRenderer;
use neobabu_core::NeobabuCore;
use poise::serenity_prelude::{EmojiId, ReactionType};
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct BotState {
    pub config: Arc<BotConfig>,
    pub core: Arc<NeobabuCore>,
    pub o2d: O2DRenderer,
}

impl BotState {
    pub async fn initialize() -> BotResult<Self> {
        info!("Initializing bot state...");

        info!("Loading bot config...");
        let config = BotConfig::load_from_env()?;
        info!("Bot config loaded");

        info!("Loading core config...");
        let core_config = load_core_config()?;
        info!("Core config loaded");

        info!("Initializing core...");
        let core = NeobabuCore::initialize(core_config).await?;
        info!("Core initialized");

        let state = Self {
            config: Arc::new(config),
            core: Arc::new(core),
            o2d: O2DRenderer::initialize()?,
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

fn load_core_config() -> BotResult<Config> {
    let db_url = std::env::var("DATABASE_URL")?;
    let nasa_api_key = std::env::var("NASA_API_KEY")?;
    let youtube_api_key = std::env::var("YOUTUBE_API_KEY")?;
    let youtube_hub_callback_url = std::env::var("YOUTUBE_HUB_CALLBACK_URL")?;
    let youtube_hub_secret = std::env::var("YOUTUBE_HUB_SECRET")?;
    Ok(Config {
        db_url,
        nasa_api_key: Some(nasa_api_key),
        youtube_api_key: Some(youtube_api_key),
        youtube_hub_callback_url: Some(youtube_hub_callback_url),
        youtube_hub_secret: Some(youtube_hub_secret),
    })
}
