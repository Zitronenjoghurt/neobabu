use crate::api::error::ApiResult;
use crate::api::extractors::auth_user::AuthUser;
use crate::api::models::response::me::MeResponse;
use crate::state::ServerState;
use axum::routing::get;
use axum::Router;

mod settings;

async fn get_me(user: AuthUser) -> ApiResult<MeResponse> {
    let user = user.into_model();
    Ok(MeResponse {
        id: user.id,
        username: user.username,
        avatar_hash: user.avatar_hash,
    })
}

pub fn router() -> Router<ServerState> {
    Router::<ServerState>::new()
        .route("/", get(get_me))
        .nest("/settings", settings::router())
}
