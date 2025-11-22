use crate::api::error::{ApiError, ApiResult};
use crate::state::ServerState;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use neobabu_core::database::entity::user;
use tower_sessions::Session;

pub struct AuthUser(pub user::Model);

impl AuthUser {
    pub fn into_model(self) -> user::Model {
        self.0
    }
}

impl FromRequestParts<ServerState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &ServerState) -> ApiResult<Self> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::Unauthorized)?;

        let user_id = session
            .get::<String>("user_id")
            .await?
            .ok_or(ApiError::Unauthorized)?;

        let user = state
            .core
            .stores
            .user
            .find_by_id(&user_id)
            .await?
            .ok_or(ApiError::Unauthorized)?;

        Ok(Self(user))
    }
}

impl std::ops::Deref for AuthUser {
    type Target = user::Model;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
