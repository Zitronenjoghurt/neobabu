use crate::config::Config;
use crate::database::Database;
use crate::error::CoreResult;
use std::sync::Arc;

pub mod config;
mod database;
mod error;

#[derive(Clone)]
pub struct NeobabuCore {
    config: Arc<Config>,
    db: Arc<Database>,
}

impl NeobabuCore {
    pub async fn initialize(config: Config) -> CoreResult<Self> {
        let config = Arc::new(config);
        let db = Database::initialize(&config).await?;
        Ok(Self { config, db })
    }
}
