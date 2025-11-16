use crate::database::entity::user_birthday;
use crate::database::Database;
use crate::error::CoreResult;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, ColumnTrait};
use sea_orm::{EntityTrait, Set};
use std::sync::Arc;

pub struct UserBirthdayStore {
    db: Arc<Database>,
}

impl UserBirthdayStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn find_by_user_id(
        &self,
        id: impl AsRef<str>,
    ) -> CoreResult<Option<user_birthday::Model>> {
        Ok(user_birthday::Entity::find()
            .filter(user_birthday::Column::UserId.eq(id.as_ref().to_string()))
            .one(self.db.conn())
            .await?)
    }

    pub async fn update(
        &self,
        mut model: user_birthday::ActiveModel,
    ) -> CoreResult<user_birthday::Model> {
        model.updated_at = Set(chrono::Utc::now().naive_utc());
        let model = model.update(self.db.conn()).await?;
        Ok(model)
    }

    pub async fn insert(
        &self,
        model: user_birthday::ActiveModel,
    ) -> CoreResult<user_birthday::Model> {
        Ok(model.insert(self.db.conn()).await?)
    }

    pub async fn stream_all(
        &self,
    ) -> CoreResult<impl futures::Stream<Item = Result<user_birthday::Model, sea_orm::DbErr>>> {
        Ok(user_birthday::Entity::find().stream(self.db.conn()).await?)
    }
}
