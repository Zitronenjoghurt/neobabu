use crate::database::entity::apod;
use crate::database::Database;
use crate::error::CoreResult;
use sea_orm::{ActiveModelTrait, EntityTrait};
use std::sync::Arc;

pub struct ApodStore {
    db: Arc<Database>,
}

impl ApodStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn find(&self, day: i16, month: i16, year: i16) -> CoreResult<Option<apod::Model>> {
        Ok(apod::Entity::find_by_id((day, month, year))
            .one(self.db.conn())
            .await?)
    }

    pub async fn insert(&self, model: apod::ActiveModel) -> CoreResult<apod::Model> {
        Ok(model.insert(self.db.conn()).await?)
    }

    pub async fn update(&self, model: apod::ActiveModel) -> CoreResult<apod::Model> {
        Ok(model.update(self.db.conn()).await?)
    }
}
