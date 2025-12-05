use crate::database::entity::{farming, user};
use crate::database::Database;
use crate::error::CoreResult;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use std::sync::Arc;

pub struct FarmingStore {
    db: Arc<Database>,
}

impl FarmingStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn find_by_user(
        &self,
        user_id: impl AsRef<str>,
    ) -> CoreResult<Option<farming::Model>> {
        Ok(farming::Entity::find_by_id(user_id.as_ref())
            .one(self.db.conn())
            .await?)
    }

    pub async fn fetch_or_create(&self, user: &user::Model) -> CoreResult<farming::Model> {
        if let Some(farming) = self.find_by_user(user.id.as_str()).await? {
            return Ok(farming);
        }

        let new = farming::ActiveModel {
            user_id: Set(user.id.clone()),
            ..Default::default()
        };

        Ok(new.insert(self.db.conn()).await?)
    }
}
