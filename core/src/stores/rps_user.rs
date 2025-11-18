use crate::database::entity::{rps_user, user};
use crate::database::Database;
use crate::error::CoreResult;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use std::sync::Arc;

pub struct RPSUserStore {
    db: Arc<Database>,
}

impl RPSUserStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn find_by_id(
        &self,
        user_id: impl AsRef<str>,
    ) -> CoreResult<Option<rps_user::Model>> {
        Ok(rps_user::Entity::find_by_id(user_id.as_ref().to_string())
            .one(self.db.conn())
            .await?)
    }

    pub async fn fetch_or_create(&self, user: &user::Model) -> CoreResult<rps_user::Model> {
        if let Some(existing) = self.find_by_id(&user.id).await? {
            return Ok(existing);
        };

        let new = rps_user::ActiveModel {
            user_id: Set(user.id.to_string()),
            ..Default::default()
        };

        Ok(new.insert(self.db.conn()).await?)
    }

    pub async fn update(&self, mut model: rps_user::ActiveModel) -> CoreResult<rps_user::Model> {
        model.updated_at = Set(chrono::Utc::now().naive_utc());
        Ok(model.update(self.db.conn()).await?)
    }
}
