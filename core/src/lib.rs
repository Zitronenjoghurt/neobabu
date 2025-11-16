use crate::config::Config;
use crate::database::Database;
use crate::error::CoreResult;
use std::sync::Arc;

pub mod config;
pub mod database;
pub mod error;
pub mod events;
pub mod jobs;
pub mod services;
pub mod stores;
mod types;

#[derive(Clone)]
pub struct NeobabuCore {
    pub config: Arc<Config>,
    pub db: Arc<Database>,
    pub event_bus: Arc<events::CoreEventBus>,
    pub services: Arc<services::Services>,
    pub stores: Arc<stores::Stores>,
}

impl NeobabuCore {
    pub async fn initialize(config: Config) -> CoreResult<Self> {
        let config = Arc::new(config);
        let db = Database::initialize(&config).await?;
        let event_bus = events::CoreEventBus::initialize();
        let stores = stores::Stores::initialize(&db);
        let services = services::Services::initialize(&stores);
        Ok(Self {
            config,
            db,
            event_bus,
            services,
            stores,
        })
    }

    pub async fn start_jobs(&self) -> CoreResult<()> {
        let scheduler = jobs::Scheduler::new(&self).await?;

        tokio::spawn(async move {
            if let Err(err) = scheduler.start().await {
                tracing::error!("Failed to start job scheduler: {err}");
            }
        });

        Ok(())
    }
}
