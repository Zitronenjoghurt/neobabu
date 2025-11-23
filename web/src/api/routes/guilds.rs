use crate::api::error::ApiResult;
use crate::api::extractors::auth_user::AuthUser;
use crate::api::models::response::guilds::GuildsResponse;
use crate::state::ServerState;
use axum::extract::State;
use axum::routing::get;
use axum::Router;

async fn get_guilds(user: AuthUser, State(state): State<ServerState>) -> ApiResult<GuildsResponse> {
    let user = user.into_model();
    let guilds = state
        .core
        .services
        .user
        .guild_infos(&user, &state.oauth_cryptor)
        .await?;
    Ok(GuildsResponse { guilds })
}

pub fn router() -> Router<ServerState> {
    Router::<ServerState>::new().route("/", get(get_guilds))
}
