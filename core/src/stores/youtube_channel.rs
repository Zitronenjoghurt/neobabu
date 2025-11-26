use crate::database::entity::youtube_channel;
use crate::database::Database;
use crate::error::CoreResult;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
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

    pub async fn find_by_handle(
        &self,
        handle: impl AsRef<str>,
    ) -> CoreResult<Option<youtube_channel::Model>> {
        let normalized_handle = handle.as_ref().strip_prefix('@').unwrap_or(handle.as_ref());
        Ok(youtube_channel::Entity::find()
            .filter(youtube_channel::Column::Handle.eq(normalized_handle))
            .one(self.db.conn())
            .await?)
    }

    pub async fn insert(
        &self,
        mut model: youtube_channel::ActiveModel,
    ) -> CoreResult<youtube_channel::Model> {
        if let Some(handle) = model.handle.as_ref() {
            model.handle = Set(Some(handle.strip_prefix('@').unwrap_or(handle).to_string()));
        }
        Ok(model.insert(self.db.conn()).await?)
    }

    pub async fn update(
        &self,
        mut model: youtube_channel::ActiveModel,
    ) -> CoreResult<youtube_channel::Model> {
        if let Some(handle) = model.handle.as_ref() {
            model.handle = Set(Some(handle.strip_prefix('@').unwrap_or(handle).to_string()));
        }
        model.updated_at = Set(chrono::Utc::now().naive_utc());
        Ok(model.update(self.db.conn()).await?)
    }
}
