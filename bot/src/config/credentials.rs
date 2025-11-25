#[derive(serde::Deserialize)]
pub struct BotConfigCredentials {
    #[serde(default)]
    pub nasa_api: Option<String>,
    #[serde(default)]
    pub youtube_api: Option<String>,
    pub token: String,
}
