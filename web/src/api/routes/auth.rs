use crate::state::ServerState;
use axum::Router;

mod callback;
mod login;
mod logout;

pub fn router() -> Router<ServerState> {
    Router::new()
        .nest("/callback", callback::router())
        .nest("/login", login::router())
        .nest("/logout", logout::router())
}
