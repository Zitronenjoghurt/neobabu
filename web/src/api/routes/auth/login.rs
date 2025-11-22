use crate::api::error::ApiResult;
use crate::state::ServerState;
use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;
use axum::Router;
use oauth2::{CsrfToken, PkceCodeChallenge, Scope};
use tower_sessions::Session;

async fn get_login(
    session: Session,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = state
        .oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("guilds".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    session
        .insert("oauth_csrf", csrf_token.secret().to_string())
        .await?;

    session
        .insert("oauth_pkce", pkce_verifier.secret().to_string())
        .await?;

    session.save().await?;

    Ok(Redirect::to(auth_url.as_ref()))
}

pub fn router() -> Router<ServerState> {
    Router::<ServerState>::new().route("/", get(get_login))
}
