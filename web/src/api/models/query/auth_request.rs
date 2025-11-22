use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}
