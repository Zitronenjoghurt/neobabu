use crate::database::entity::{guild, guild_apod};
use crate::database::Database;
use crate::error::CoreResult;
use futures::StreamExt;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use std::sync::Arc;

pub struct GuildApodStore {
    db: Arc<Database>,
}

impl GuildApodStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn find_by_guild_id(
        &self,
        guild_id: impl AsRef<str>,
    ) -> CoreResult<Option<guild_apod::Model>> {
        Ok(guild_apod::Entity::find_by_id(guild_id.as_ref())
            .one(self.db.conn())
            .await?)
    }

    pub async fn fetch_or_create(&self, guild: &guild::Model) -> CoreResult<guild_apod::Model> {
        if let Some(existing) = self.find_by_guild_id(&guild.id).await? {
            return Ok(existing);
        };

        let new = guild_apod::ActiveModel {
            guild_id: Set(guild.id.to_string()),
            ..Default::default()
        };

        Ok(new.insert(self.db.conn()).await?)
    }

    pub async fn update(
        &self,
        mut model: guild_apod::ActiveModel,
    ) -> CoreResult<guild_apod::Model> {
        model.updated_at = Set(chrono::Utc::now().naive_utc());
        Ok(model.update(self.db.conn()).await?)
    }

    pub async fn stream_all_enabled(
        &self,
    ) -> CoreResult<impl futures::Stream<Item = CoreResult<guild_apod::Model>>> {
        Ok(guild_apod::Entity::find()
            .filter(guild_apod::Column::Enabled.eq(true))
            .stream(self.db.conn())
            .await?
            .map(|model| Ok(model?)))
    }
}
