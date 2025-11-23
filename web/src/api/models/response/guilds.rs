use axum::response::{IntoResponse, Response};
use neobabu_core::types::user_guild_info::UserGuildInfo;

#[derive(serde::Serialize)]
pub struct GuildsResponse {
    pub guilds: Vec<UserGuildInfo>,
}

impl IntoResponse for GuildsResponse {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}
