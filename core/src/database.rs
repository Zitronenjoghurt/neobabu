use crate::config::Config;
use crate::error::CoreResult;
use migration::{Migrator, MigratorTrait};
use sea_orm::*;
use std::sync::Arc;
use tracing::info;

pub mod entity;
mod entity_ext;

#[derive(Debug)]
pub struct Database {
    connection: DatabaseConnection,
}

impl Database {
    pub async fn initialize(config: &Arc<Config>) -> CoreResult<Arc<Self>> {
        let options = ConnectOptions::new::<&str>(config.db_url.as_ref());

        info!("Connecting to database...");
        let connection = sea_orm::Database::connect(options).await?;
        info!("Database connection established");

        let db = Self { connection };
        db.apply_migrations().await?;

        Ok(Arc::new(db))
    }

    pub fn conn(&self) -> &DatabaseConnection {
        &self.connection
    }

    async fn apply_migrations(&self) -> CoreResult<()> {
        info!("Applying database migrations...");
        Migrator::up(&self.connection, None).await?;
        info!("Database migrations applied");
        Ok(())
    }
}
