use crate::state::ServerState;
use axum::Router;

mod webhook;

pub fn router() -> Router<ServerState> {
    Router::new().nest("/webhook", webhook::router())
}
