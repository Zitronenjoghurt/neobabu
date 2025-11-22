use crate::api::error::ApiResult;
use crate::api::extractors::auth_user::AuthUser;
use crate::state::ServerState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use neobabu_core::stores::{IntoActiveModel, Set};
use tower_sessions::Session;

async fn get_logout(
    State(state): State<ServerState>,
    user: AuthUser,
    session: Session,
) -> ApiResult<impl IntoResponse> {
    let mut active_user = user.into_model().into_active_model();
    active_user.encrypted_oauth_token = Set(None);
    state.core.stores.user.update(active_user).await?;
    session.delete().await?;
    Ok(())
}

pub fn router() -> Router<ServerState> {
    Router::<ServerState>::new().route("/", get(get_logout))
}
