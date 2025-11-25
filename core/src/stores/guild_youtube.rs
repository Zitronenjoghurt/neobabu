use crate::database::entity::{guild, guild_youtube};
use crate::database::Database;
use crate::error::CoreResult;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use std::sync::Arc;

pub struct GuildYoutubeStore {
    db: Arc<Database>,
}

impl GuildYoutubeStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn find_by_guild_id(
        &self,
        guild_id: impl AsRef<str>,
    ) -> CoreResult<Option<guild_youtube::Model>> {
        Ok(guild_youtube::Entity::find_by_id(guild_id.as_ref())
            .one(self.db.conn())
            .await?)
    }

    pub async fn fetch_or_create(&self, guild: &guild::Model) -> CoreResult<guild_youtube::Model> {
        if let Some(existing) = self.find_by_guild_id(&guild.id).await? {
            return Ok(existing);
        };

        let new = guild_youtube::ActiveModel {
            guild_id: Set(guild.id.to_string()),
            ..Default::default()
        };

        Ok(new.insert(self.db.conn()).await?)
    }

    pub async fn update(
        &self,
        mut model: guild_youtube::ActiveModel,
    ) -> CoreResult<guild_youtube::Model> {
        model.updated_at = Set(chrono::Utc::now().naive_utc());
        Ok(model.update(self.db.conn()).await?)
    }
}
