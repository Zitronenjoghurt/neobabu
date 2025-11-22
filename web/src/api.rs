use crate::state::ServerState;
use axum::Router;

mod error;
mod extractors;
mod layers;
mod models;
mod routes;

pub fn build(state: &ServerState) -> Router<ServerState> {
    Router::new().merge(routes::build_routes(state))
}
