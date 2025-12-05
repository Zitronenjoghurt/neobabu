use crate::database::entity::{guild, user, youtube_channel};
use crate::error::{CoreError, CoreResult};
use crate::integrations::apis::youtube::YoutubeChannel;
use crate::integrations::apis::Apis;
use crate::stores::Stores;
use chrono::Duration;
use sea_orm::{IntoActiveModel, Set};
use std::ops::Add;
use std::sync::Arc;

const GUILD_CHANNEL_LIMIT: u64 = 100;
const USER_CHANNEL_LIMIT: u64 = 100;

pub struct YoutubeService {
    apis: Arc<Apis>,
    stores: Arc<Stores>,
}

impl YoutubeService {
    pub fn initialize(apis: &Arc<Apis>, stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            apis: apis.clone(),
            stores: stores.clone(),
        })
    }

    fn active_model_from_channel_info(
        &self,
        channel_info: YoutubeChannel,
    ) -> youtube_channel::ActiveModel {
        youtube_channel::ActiveModel {
            id: Set(channel_info.id),
            name: Set(channel_info.title),
            icon_url: Set(channel_info.icon_url),
            ..Default::default()
        }
    }

    pub async fn update_channel_if_needed(
        &self,
        channel: youtube_channel::Model,
    ) -> CoreResult<Option<youtube_channel::Model>> {
        if channel.updated_at.add(Duration::days(7)) >= chrono::Utc::now().naive_utc() {
            return Ok(Some(channel));
        };

        let Some(channel_info) = self.apis.youtube.fetch_channel_by_id(&channel.id).await? else {
            self.stores.youtube_channel.delete(&channel.id).await?;
            return Ok(None);
        };

        let mut active = channel.into_active_model();
        active.name = Set(channel_info.title);
        active.icon_url = Set(channel_info.icon_url);
        Ok(Some(self.stores.youtube_channel.update(active).await?))
    }

    pub async fn find_channel_by_id(
        &self,
        channel_id: impl AsRef<str>,
    ) -> CoreResult<Option<youtube_channel::Model>> {
        if let Some(existing) = self.stores.youtube_channel.find_by_id(&channel_id).await? {
            return Ok(Some(existing));
        };

        let Some(channel_info) = self.apis.youtube.fetch_channel_by_id(channel_id).await? else {
            return Ok(None);
        };

        let active = self.active_model_from_channel_info(channel_info);
        let channel = self.stores.youtube_channel.insert(active).await?;
        Ok(Some(channel))
    }

    pub async fn find_channel_by_handle(
        &self,
        channel_handle: impl AsRef<str>,
    ) -> CoreResult<Option<youtube_channel::Model>> {
        if let Some(existing) = self
            .stores
            .youtube_channel
            .find_by_handle(&channel_handle)
            .await?
        {
            return Ok(Some(existing));
        };

        let Some(channel_info) = self
            .apis
            .youtube
            .fetch_channel_by_handle(&channel_handle)
            .await?
        else {
            return Ok(None);
        };

        if let Some(existing) = self
            .stores
            .youtube_channel
            .find_by_id(&channel_info.id)
            .await?
        {
            let mut active = existing.into_active_model();
            active.handle = Set(Some(channel_handle.as_ref().to_string()));
            let model = self.stores.youtube_channel.update(active).await?;
            return Ok(Some(model));
        };

        let mut active = self.active_model_from_channel_info(channel_info);
        active.handle = Set(Some(channel_handle.as_ref().to_string()));
        let channel = self.stores.youtube_channel.insert(active).await?;
        Ok(Some(channel))
    }

    pub async fn verify_can_subscribe(
        &self,
        guild: &guild::Model,
        user: &user::Model,
    ) -> CoreResult<()> {
        let guild_count = self
            .stores
            .guild_youtube_channel
            .count_by_guild_id(&guild.id)
            .await?;
        if guild_count >= GUILD_CHANNEL_LIMIT {
            return Err(CoreError::GuildYoutubeChannelLimitReached);
        };

        let user_count = self
            .stores
            .guild_youtube_channel
            .count_by_user_id(&user.id)
            .await?;
        if user_count >= USER_CHANNEL_LIMIT {
            return Err(CoreError::UserYoutubeChannelLimitReached);
        };

        Ok(())
    }

    pub async fn verify_can_subscribe_to_channel(
        &self,
        guild: &guild::Model,
        channel: &youtube_channel::Model,
    ) -> CoreResult<()> {
        if self
            .stores
            .guild_youtube_channel
            .find(&guild.id, &channel.id)
            .await?
            .is_some()
        {
            return Err(CoreError::GuildYoutubeChannelAlreadySubscribed);
        }

        Ok(())
    }

    pub async fn guild_subscribe(
        &self,
        guild: &guild::Model,
        user: &user::Model,
        channel: &youtube_channel::Model,
    ) -> CoreResult<()> {
        self.verify_can_subscribe_to_channel(guild, channel).await?;
        self.verify_can_subscribe(guild, user).await?;

        self.stores
            .guild_youtube_channel
            .add(guild, channel, user)
            .await?;

        Ok(())
    }
}
