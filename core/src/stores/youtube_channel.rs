use crate::database::entity::youtube_channel;
use crate::database::Database;
use crate::error::CoreResult;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use std::sync::Arc;

pub struct YoutubeChannelStore {
    db: Arc<Database>,
}

impl YoutubeChannelStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn find_by_id(
        &self,
        id: impl AsRef<str>,
    ) -> CoreResult<Option<youtube_channel::Model>> {
        Ok(youtube_channel::Entity::find_by_id(id.as_ref())
            .one(self.db.conn())
            .await?)
    }

    pub async fn insert(
        &self,
        model: youtube_channel::ActiveModel,
    ) -> CoreResult<youtube_channel::Model> {
        Ok(model.insert(self.db.conn()).await?)
    }

    pub async fn update(
        &self,
        mut model: youtube_channel::ActiveModel,
    ) -> CoreResult<youtube_channel::Model> {
        model.updated_at = Set(chrono::Utc::now().naive_utc());
        Ok(model.update(self.db.conn()).await?)
    }
}
