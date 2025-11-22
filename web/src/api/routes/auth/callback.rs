use crate::api::error::{ApiError, ApiResult};
use crate::api::models::discord::user::DiscordUser;
use crate::api::models::query::auth_request::AuthRequest;
use crate::state::ServerState;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;
use axum::Router;
use neobabu_core::stores::{IntoActiveModel, Set};
use oauth2::{AuthorizationCode, PkceCodeVerifier, TokenResponse};
use tower_sessions::Session;

async fn get_callback(
    session: Session,
    State(state): State<ServerState>,
    Query(query): Query<AuthRequest>,
) -> ApiResult<impl IntoResponse> {
    let csrf_token = session
        .get::<String>("oauth_csrf")
        .await?
        .ok_or(ApiError::MissingCsrfToken)?;
    let pkce_verifier = session
        .get::<String>("oauth_pkce")
        .await?
        .ok_or(ApiError::MissingPkceVerifier)?;

    if query.state != csrf_token {
        return Err(ApiError::InvalidCsrfToken);
    };

    let client = reqwest::Client::new();
    let code = AuthorizationCode::new(query.code);
    let pkce_verifier = PkceCodeVerifier::new(pkce_verifier);
    let token_response = state
        .oauth_client
        .exchange_code(code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(&client)
        .await
        .map_err(|_| ApiError::OauthTokenExchangeFailed)?;

    let discord_user = client
        .get("https://discord.com/api/users/@me")
        .bearer_auth(token_response.access_token().secret())
        .send()
        .await?
        .json::<DiscordUser>()
        .await?;

    let mut user = state
        .core
        .stores
        .user
        .fetch_or_create(&discord_user.id)
        .await?
        .into_active_model();
    let oauth_token = state
        .oauth_cryptor
        .encrypt(&token_response.access_token().secret())?;
    user.encrypted_oauth_token = Set(Some(oauth_token));
    user.username = Set(Some(discord_user.username));
    user.avatar_hash = Set(discord_user.avatar_hash);
    state.core.stores.user.update(user).await?;

    session.remove::<String>("oauth_csrf").await?;
    session.remove::<String>("oauth_pkce").await?;
    session.insert("user_id", discord_user.id).await?;
    session.cycle_id().await?;
    session.save().await?;

    Ok(Redirect::to("/"))
}

pub fn router() -> Router<ServerState> {
    Router::<ServerState>::new().route("/", get(get_callback))
}
