#[derive(serde::Deserialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    #[serde(rename = "avatar")]
    pub avatar_hash: Option<String>,
}
