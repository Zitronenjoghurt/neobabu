mod credentials;
mod emoji;

#[derive(serde::Deserialize)]
pub struct BotConfig {
    pub credentials: credentials::BotConfigCredentials,
    #[serde(default)]
    pub emojis: emoji::BotConfigEmoji,
}

impl BotConfig {
    pub fn load_from_env() -> anyhow::Result<Self> {
        let config_path_string = std::env::var("BOT_CONFIG_PATH")?;
        let config_path = std::path::PathBuf::from(config_path_string);
        let config_str = std::fs::read_to_string(config_path)?;
        let config: Self = toml::from_str(&config_str)?;
        Ok(config)
    }
}
