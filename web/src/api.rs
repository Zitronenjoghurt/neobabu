use crate::state::ServerState;
use axum::Router;

pub fn build() -> Router<ServerState> {
    Router::new()
}
