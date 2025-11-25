use crate::api::layers::csrf::build_csrf_layer;
use crate::api::layers::rate_limit::build_rate_limit_layer;
use crate::api::layers::session::build_session_layer;
use crate::state::ServerState;
use axum::Router;

mod auth;
mod csrf;
mod guilds;
mod me;

pub fn build_routes(state: &ServerState) -> Router<ServerState> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/csrf", csrf::router())
        .nest("/me", me::router())
        .nest("/guilds", guilds::router())
        .layer(build_session_layer(state))
        .layer(build_csrf_layer(state))
        .layer(build_rate_limit_layer())
}
