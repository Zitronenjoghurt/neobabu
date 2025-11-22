use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
pub struct MeResponse {
    pub id: String,
    pub username: Option<String>,
    pub avatar_hash: Option<String>,
}

impl IntoResponse for MeResponse {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}
