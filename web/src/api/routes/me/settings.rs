use crate::api::error::ApiResult;
use crate::api::extractors::auth_user::AuthUser;
use crate::api::models::response::user_settings::UserSettingsResponse;
use crate::state::ServerState;
use axum::extract::State;
use axum::routing::get;
use axum::Router;

async fn get_settings(
    user: AuthUser,
    State(state): State<ServerState>,
) -> ApiResult<UserSettingsResponse> {
    let user = user.into_model();
    Ok(state
        .core
        .services
        .user
        .get_settings(&state.core.services.birthday, &user)
        .await?
        .into())
}

pub fn router() -> Router<ServerState> {
    Router::<ServerState>::new().route("/", get(get_settings))
}
