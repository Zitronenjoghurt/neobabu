use serde::Serialize;

#[derive(Serialize)]
pub struct UserGuildInfo {
    pub id: String,
    pub name: String,
    pub icon_hash: Option<String>,
    pub has_bot: bool,
    pub is_active: bool,
    pub can_add_bot: bool,
}
