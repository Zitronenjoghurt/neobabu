use crate::error::ServerResult;
use neobabu_core::config::Config;
use neobabu_core::NeobabuCore;
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct ServerState {
    pub core: Arc<NeobabuCore>,
}

impl ServerState {
    pub async fn initialize() -> ServerResult<Self> {
        info!("Initializing server state...");

        info!("Loading core config...");
        let core_config = load_core_config()?;
        info!("Core config loaded");

        info!("Initializing core...");
        let core = NeobabuCore::initialize(core_config).await?;
        info!("Core initialized");

        Ok(Self {
            core: Arc::new(core),
        })
    }
}

fn load_core_config() -> ServerResult<Config> {
    let db_url = std::env::var("DATABASE_URL")?;
    Ok(Config {
        db_url,
        ..Default::default()
    })
}
