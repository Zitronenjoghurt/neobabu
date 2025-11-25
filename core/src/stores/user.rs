use crate::database::entity::user;
use crate::database::Database;
use crate::error::CoreResult;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use std::sync::Arc;

#[derive(Debug)]
pub struct UserStore {
    db: Arc<Database>,
}

impl UserStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn find_by_id(&self, id: impl AsRef<str>) -> CoreResult<Option<user::Model>> {
        Ok(user::Entity::find_by_id(id.as_ref().to_string())
            .one(self.db.conn())
            .await?)
    }

    pub async fn fetch_or_create(&self, id: impl AsRef<str>) -> CoreResult<user::Model> {
        let id = id.as_ref().to_string();
        if let Some(existing_user) = self.find_by_id(&id).await? {
            return Ok(existing_user);
        };

        let new_user = user::ActiveModel {
            id: Set(id.into()),
            ..Default::default()
        };
        Ok(new_user.insert(self.db.conn()).await?)
    }

    pub async fn update(&self, mut model: user::ActiveModel) -> CoreResult<user::Model> {
        model.updated_at = Set(chrono::Utc::now().naive_utc());
        Ok(model.update(self.db.conn()).await?)
    }
}
