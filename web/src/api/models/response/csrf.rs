use axum::response::IntoResponse;

#[derive(serde::Serialize)]
pub struct CsrfResponse {
    pub csrf_token: String,
}

impl CsrfResponse {
    pub fn new(csrf_token: String) -> Self {
        Self { csrf_token }
    }
}

impl IntoResponse for CsrfResponse {
    fn into_response(self) -> axum::response::Response {
        axum::response::Json(self.csrf_token).into_response()
    }
}
