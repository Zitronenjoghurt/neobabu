use crate::state::ServerState;
use axum::Router;
use neobabu_core::database::entity::inventory_item;
use neobabu_core::inventory::kind::ItemKind;
use neobabu_core::inventory::state::ItemState;
use std::net::{IpAddr, SocketAddr};
use tower_http::services::{ServeDir, ServeFile};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

mod api;
mod config;
mod error;
mod state;

#[tokio::main]
async fn main() {
    init_tracing();
    info!("Starting server...");

    let state = ServerState::initialize().await.unwrap();
    let api = api::build(&state);

    let app = if let Some(dev_server) = &state.config.vite_dev_server {
        info!("Running in DEV mode, proxying to Vite at {}", dev_server);
        Router::new()
            .nest("/api", api)
            .fallback(proxy_to_vite)
            .with_state(state)
    } else {
        info!("Running in PROD mode, serving static files");
        Router::new()
            .nest("/api", api)
            .fallback_service(
                ServeDir::new("./static").not_found_service(ServeFile::new("./static/index.html")),
            )
            .with_state(state)
    };

    let address = SocketAddr::new(IpAddr::from([0, 0, 0, 0]), 48573);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    info!("Listening on {address}");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
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

async fn proxy_to_vite(
    axum::extract::State(_state): axum::extract::State<ServerState>,
    req: axum::extract::Request,
) -> axum::response::Response {
    let vite_url = std::env::var("VITE_DEV_SERVER").unwrap();

    let client = reqwest::Client::new();
    let path = req.uri().path();
    let query = req
        .uri()
        .query()
        .map(|q| format!("?{}", q))
        .unwrap_or_default();

    match client
        .get(format!("{}{}{}", vite_url, path, query))
        .send()
        .await
    {
        Ok(response) => {
            let mut builder = axum::response::Response::builder().status(response.status());

            for (key, value) in response.headers() {
                builder = builder.header(key, value);
            }

            match response.bytes().await {
                Ok(bytes) => builder.body(axum::body::Body::from(bytes)).unwrap(),
                Err(err) => {
                    error!("{err}");
                    axum::response::Response::builder()
                        .status(500)
                        .body(axum::body::Body::empty())
                        .unwrap()
                }
            }
        }
        Err(err) => {
            error!("{err}");
            axum::response::Response::builder()
                .status(502)
                .body(axum::body::Body::empty())
                .unwrap()
        }
    }
}
