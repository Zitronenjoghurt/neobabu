use crate::database::entity::youtube_video;
use crate::database::Database;
use crate::error::CoreResult;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ExprTrait, QueryFilter, QueryOrder};
use std::sync::Arc;

pub struct YoutubeVideoStore {
    db: Arc<Database>,
}

impl YoutubeVideoStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn find_by_id(
        &self,
        id: impl AsRef<str>,
    ) -> CoreResult<Option<youtube_video::Model>> {
        Ok(youtube_video::Entity::find_by_id(id.as_ref())
            .one(self.db.conn())
            .await?)
    }

    pub async fn insert(
        &self,
        model: youtube_video::ActiveModel,
    ) -> CoreResult<youtube_video::Model> {
        Ok(model.insert(self.db.conn()).await?)
    }

    pub async fn find_unannounced_for_channel_id(
        &self,
        channel_id: impl AsRef<str>,
    ) -> CoreResult<Vec<youtube_video::Model>> {
        Ok(youtube_video::Entity::find()
            .filter(
                youtube_video::Column::ChannelId
                    .eq(channel_id.as_ref())
                    .and(youtube_video::Column::NotificationSent.eq(false)),
            )
            .order_by_asc(youtube_video::Column::CreatedAt)
            .all(self.db.conn())
            .await?)
    }
}
