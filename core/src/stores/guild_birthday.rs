use crate::database::entity::guild_birthday;
use crate::database::Database;
use crate::error::CoreResult;
use sea_orm::{ActiveModelTrait, ColumnTrait, Set};
use sea_orm::{EntityTrait, QueryFilter};
use std::sync::Arc;

pub struct GuildBirthdayStore {
    db: Arc<Database>,
}

impl GuildBirthdayStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn find_by_guild_id(
        &self,
        id: impl AsRef<str>,
    ) -> CoreResult<Option<guild_birthday::Model>> {
        Ok(guild_birthday::Entity::find()
            .filter(guild_birthday::Column::GuildId.eq(id.as_ref().to_string()))
            .one(self.db.conn())
            .await?)
    }

    pub async fn fetch_or_create(
        &self,
        guild_id: impl AsRef<str>,
    ) -> CoreResult<guild_birthday::Model> {
        let guild_id = guild_id.as_ref().to_string();
        if let Some(existing_guild_birthday) = self.find_by_guild_id(&guild_id).await? {
            return Ok(existing_guild_birthday);
        };

        let new_guild_birthday = guild_birthday::ActiveModel {
            guild_id: Set(guild_id.into()),
            enabled: Set(false),
            notification_channel_id: Set(None),
            ..Default::default()
        };

        Ok(new_guild_birthday.insert(self.db.conn()).await?)
    }

    pub async fn update(
        &self,
        mut model: guild_birthday::ActiveModel,
    ) -> CoreResult<guild_birthday::Model> {
        model.updated_at = Set(chrono::Utc::now().naive_utc());
        let model = model.update(self.db.conn()).await?;
        Ok(model)
    }
}
