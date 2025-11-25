use crate::api::error::ApiResult;
use crate::api::extractors::auth_user::AuthUser;
use crate::api::models::response::csrf::CsrfResponse;
use crate::state::ServerState;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use axum_csrf::CsrfToken;

async fn get_csrf(_user: AuthUser, token: CsrfToken) -> ApiResult<impl IntoResponse> {
    let response = CsrfResponse::new(token.authenticity_token()?);
    Ok((token, response).into_response())
}

pub fn router() -> Router<ServerState> {
    Router::<ServerState>::new().route("/", get(get_csrf))
}
