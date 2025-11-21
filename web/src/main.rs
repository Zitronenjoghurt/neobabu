use crate::state::ServerState;
use axum::Router;
use std::net::{IpAddr, SocketAddr};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod api;
mod error;
mod state;

#[tokio::main]
async fn main() {
    init_tracing();
    info!("Starting server...");

    let state = ServerState::initialize().await.unwrap();
    let api = api::build();

    let app = Router::new()
        .nest("/api", api)
        .fallback_service(ServeDir::new("./static"))
        .with_state(state);

    let address = SocketAddr::new(IpAddr::from([0, 0, 0, 0]), 48573);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    info!("Listening on {address}");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
    info!("Tracing initialized");
}
