use crate::database::entity::{guild, guild_youtube_channel, user, youtube_channel};
use crate::database::Database;
use crate::error::CoreResult;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};
use std::sync::Arc;

pub struct GuildYoutubeChannelStore {
    db: Arc<Database>,
}

impl GuildYoutubeChannelStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn find(
        &self,
        guild_id: impl AsRef<str>,
        channel_id: impl AsRef<str>,
    ) -> CoreResult<Option<guild_youtube_channel::Model>> {
        Ok(guild_youtube_channel::Entity::find_by_id((
            guild_id.as_ref().to_string(),
            channel_id.as_ref().to_string(),
        ))
        .one(self.db.conn())
        .await?)
    }

    pub async fn add(
        &self,
        guild: &guild::Model,
        channel: &youtube_channel::Model,
        user: &user::Model,
    ) -> CoreResult<()> {
        if self.find(&guild.id, &channel.id).await?.is_some() {
            return Ok(());
        }

        let new = guild_youtube_channel::ActiveModel {
            guild_id: Set(guild.id.to_string()),
            channel_id: Set(channel.id.to_string()),
            created_by_user_id: Set(user.id.to_string()),
            ..Default::default()
        };

        new.insert(self.db.conn()).await?;

        Ok(())
    }

    pub async fn remove(
        &self,
        guild_id: impl AsRef<str>,
        channel_id: impl AsRef<str>,
    ) -> CoreResult<()> {
        guild_youtube_channel::Entity::delete_by_id((
            guild_id.as_ref().to_string(),
            channel_id.as_ref().to_string(),
        ))
        .exec(self.db.conn())
        .await?;
        Ok(())
    }

    pub async fn count_by_guild_id(&self, guild_id: impl AsRef<str>) -> CoreResult<u64> {
        Ok(guild_youtube_channel::Entity::find()
            .filter(guild_youtube_channel::Column::GuildId.eq(guild_id.as_ref()))
            .count(self.db.conn())
            .await?)
    }

    pub async fn count_by_user_id(&self, user_id: impl AsRef<str>) -> CoreResult<u64> {
        Ok(guild_youtube_channel::Entity::find()
            .filter(guild_youtube_channel::Column::CreatedByUserId.eq(user_id.as_ref()))
            .count(self.db.conn())
            .await?)
    }
}
