use crate::config::BotConfig;
use neobabu_core::config::Config;
use neobabu_core::NeobabuCore;
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
        let core_config = load_core_config()?;
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
}

fn load_core_config() -> anyhow::Result<Config> {
    let db_url = std::env::var("DATABASE_URL")?;
    Ok(Config { db_url })
}
