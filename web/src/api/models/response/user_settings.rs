use axum::response::IntoResponse;
use neobabu_core::types::user_settings::UserSettings;

#[derive(serde::Serialize)]
pub struct UserSettingsResponse {
    settings: UserSettings,
}

impl From<UserSettings> for UserSettingsResponse {
    fn from(value: UserSettings) -> Self {
        Self { settings: value }
    }
}

impl IntoResponse for UserSettingsResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
