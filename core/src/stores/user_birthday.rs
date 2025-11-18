use crate::database::entity::{user, user_birthday, user_guild};
use crate::database::Database;
use crate::error::CoreResult;
use sea_orm::PaginatorTrait;
use sea_orm::{ActiveModelTrait, ColumnTrait, JoinType, QuerySelect, RelationTrait};
use sea_orm::{EntityTrait, Set};
use sea_orm::{QueryFilter, QueryOrder};
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

    pub async fn count_by_guild(&self, guild_id: impl AsRef<str>) -> CoreResult<u64> {
        Ok(user_birthday::Entity::find()
            .join(JoinType::InnerJoin, user_birthday::Relation::User.def())
            .join(JoinType::InnerJoin, user::Relation::UserGuild.def())
            .filter(user_guild::Column::GuildId.eq(guild_id.as_ref().to_string()))
            .count(self.db.conn())
            .await?)
    }

    pub async fn stream_by_guild(
        &self,
        guild_id: impl AsRef<str>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> CoreResult<impl futures::Stream<Item = Result<user_birthday::Model, sea_orm::DbErr>>> {
        let mut query = user_birthday::Entity::find()
            .join(JoinType::InnerJoin, user_birthday::Relation::User.def())
            .join(JoinType::InnerJoin, user::Relation::UserGuild.def())
            .filter(user_guild::Column::GuildId.eq(guild_id.as_ref().to_string()))
            .order_by_asc(user_birthday::Column::NextBirthday);

        if let Some(limit) = limit {
            query = query.limit(limit);
        }

        if let Some(offset) = offset {
            query = query.offset(offset);
        }

        Ok(query.stream(self.db.conn()).await?)
    }
}
