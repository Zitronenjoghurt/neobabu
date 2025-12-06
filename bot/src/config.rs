use crate::error::BotResult;
use std::collections::HashSet;

mod credentials;
mod emoji;

#[derive(serde::Deserialize)]
pub struct BotConfig {
    #[serde(default)]
    pub emojis: emoji::BotConfigEmoji,
    #[serde(default)]
    pub owner_ids: HashSet<u64>,
}

impl BotConfig {
    pub fn load_from_env() -> BotResult<Self> {
        let config_path_string = std::env::var("BOT_CONFIG_PATH")?;
        let config_path = std::path::PathBuf::from(config_path_string);
        let config_str = std::fs::read_to_string(config_path)?;
        let config: Self = toml::from_str(&config_str)?;
        Ok(config)
    }
}
