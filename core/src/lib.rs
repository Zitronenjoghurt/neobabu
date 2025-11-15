use crate::config::Config;
use crate::database::Database;
use crate::error::CoreResult;
use std::sync::Arc;

pub mod config;
pub mod database;
pub mod error;
pub mod services;
pub mod stores;
mod types;

#[derive(Clone)]
pub struct NeobabuCore {
    config: Arc<Config>,
    db: Arc<Database>,
    pub services: Arc<services::Services>,
    pub stores: Arc<stores::Stores>,
}

impl NeobabuCore {
    pub async fn initialize(config: Config) -> CoreResult<Self> {
        let config = Arc::new(config);
        let db = Database::initialize(&config).await?;
        let stores = stores::Stores::initialize(&db);
        let services = services::Services::initialize(&stores);
        Ok(Self {
            config,
            db,
            services,
            stores,
        })
    }
}
