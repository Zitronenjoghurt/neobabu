use crate::database::entity::{guild, user, user_guild};
use crate::database::Database;
use crate::error::CoreResult;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use std::sync::Arc;

pub struct UserGuildStore {
    db: Arc<Database>,
}

impl UserGuildStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn find_by_id(
        &self,
        user_id: impl AsRef<str>,
        guild_id: impl AsRef<str>,
    ) -> CoreResult<Option<user_guild::Model>> {
        Ok(user_guild::Entity::find_by_id((
            user_id.as_ref().to_string(),
            guild_id.as_ref().to_string(),
        ))
        .one(self.db.conn())
        .await?)
    }

    pub async fn fetch_or_create(
        &self,
        user: &user::Model,
        guild: &guild::Model,
    ) -> CoreResult<user_guild::Model> {
        if let Some(existing) = self.find_by_id(&user.id, &guild.id).await? {
            return Ok(existing);
        };

        let new = user_guild::ActiveModel {
            user_id: Set(user.id.to_string()),
            guild_id: Set(guild.id.to_string()),
            ..Default::default()
        };
        Ok(new.insert(self.db.conn()).await?)
    }

    pub async fn stream_with_user_id(
        &self,
        user_id: impl AsRef<str>,
    ) -> CoreResult<impl futures::Stream<Item = Result<user_guild::Model, sea_orm::DbErr>>> {
        Ok(user_guild::Entity::find()
            .filter(user_guild::Column::UserId.eq(user_id.as_ref().to_string()))
            .stream(self.db.conn())
            .await?)
    }

    pub async fn with_guild_ids(
        &self,
        ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> CoreResult<Vec<user_guild::Model>> {
        let ids: Vec<String> = ids.into_iter().map(|id| id.as_ref().to_string()).collect();
        Ok(user_guild::Entity::find()
            .filter(user_guild::Column::GuildId.is_in(ids))
            .all(self.db.conn())
            .await?)
    }
}
