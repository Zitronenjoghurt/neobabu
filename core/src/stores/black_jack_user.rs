use crate::database::entity::{black_jack_user, user};
use crate::database::Database;
use crate::error::CoreResult;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use std::sync::Arc;

pub struct BlackJackUserStore {
    db: Arc<Database>,
}

impl BlackJackUserStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn find(
        &self,
        user_id: impl AsRef<str>,
    ) -> CoreResult<Option<black_jack_user::Model>> {
        Ok(black_jack_user::Entity::find_by_id(user_id.as_ref())
            .one(self.db.conn())
            .await?)
    }

    pub async fn fetch_or_create(&self, user: &user::Model) -> CoreResult<black_jack_user::Model> {
        if let Some(existing) = self.find(&user.id).await? {
            return Ok(existing);
        };

        let new = black_jack_user::ActiveModel {
            user_id: Set(user.id.to_string()),
            ..Default::default()
        };

        Ok(new.insert(self.db.conn()).await?)
    }

    pub async fn update(
        &self,
        mut model: black_jack_user::ActiveModel,
    ) -> CoreResult<black_jack_user::Model> {
        model.updated_at = Set(chrono::Utc::now().naive_utc());
        Ok(model.update(self.db.conn()).await?)
    }
}
