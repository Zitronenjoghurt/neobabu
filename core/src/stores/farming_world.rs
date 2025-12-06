use crate::database::entity::farming_world;
use crate::database::Database;
use crate::error::CoreResult;
use futures::StreamExt;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use std::sync::Arc;

pub struct FarmingWorldStore {
    db: Arc<Database>,
}

impl FarmingWorldStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn find(
        &self,
        user_id: impl AsRef<str>,
        index: usize,
    ) -> CoreResult<Option<farming_world::Model>> {
        Ok(
            farming_world::Entity::find_by_id((user_id.as_ref().to_string(), index as i32))
                .one(self.db.conn())
                .await?,
        )
    }

    pub async fn count_by_user(&self, user_id: impl AsRef<str>) -> CoreResult<u64> {
        Ok(farming_world::Entity::find()
            .filter(farming_world::Column::UserId.eq(user_id.as_ref()))
            .count(self.db.conn())
            .await?)
    }

    pub async fn stream_by_user(
        &self,
        user_id: impl AsRef<str>,
    ) -> CoreResult<impl futures::Stream<Item = CoreResult<farming_world::Model>>> {
        Ok(farming_world::Entity::find()
            .filter(farming_world::Column::UserId.eq(user_id.as_ref()))
            .stream(self.db.conn())
            .await?
            .map(|model| Ok(model?)))
    }

    pub async fn insert(
        &self,
        model: farming_world::ActiveModel,
    ) -> CoreResult<farming_world::Model> {
        Ok(model.insert(self.db.conn()).await?)
    }
}
