use crate::database::entity::guild;
use crate::database::Database;
use crate::error::CoreResult;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use std::sync::Arc;

pub struct GuildStore {
    db: Arc<Database>,
}

impl GuildStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn find_by_id(&self, id: impl AsRef<str>) -> CoreResult<Option<guild::Model>> {
        Ok(guild::Entity::find_by_id(id.as_ref().to_string())
            .one(self.db.conn())
            .await?)
    }

    pub async fn fetch_or_create(&self, id: impl AsRef<str>) -> CoreResult<guild::Model> {
        let id = id.as_ref().to_string();
        if let Some(existing) = self.find_by_id(&id).await? {
            return Ok(existing);
        };

        let new = guild::ActiveModel {
            id: Set(id),
            ..Default::default()
        };
        Ok(new.insert(self.db.conn()).await?)
    }

    pub async fn with_ids(
        &self,
        ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> CoreResult<Vec<guild::Model>> {
        let ids: Vec<String> = ids.into_iter().map(|id| id.as_ref().to_string()).collect();
        Ok(guild::Entity::find()
            .filter(guild::Column::Id.is_in(ids))
            .all(self.db.conn())
            .await?)
    }
}
