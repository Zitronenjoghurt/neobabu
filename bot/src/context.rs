use crate::error::{BotError, BotResult};
use crate::state::BotState;
use crate::ui::emoji::EmojiType;
use crate::Context;
use neobabu_core::database::entity::{guild, user};
use neobabu_core::rendering::o2d::prelude::O2DRenderer;
use neobabu_core::services::Services;
use neobabu_core::stores::Stores;
use poise::serenity_prelude::ReactionType;

#[async_trait::async_trait]
pub trait ContextExt {
    fn state(&self) -> &BotState;
    fn author_id_string(&self) -> String;
    fn guild_id_string(&self) -> Option<String>;

    fn emoji(&self, emoji: EmojiType) -> ReactionType {
        self.state().get_emoji(emoji)
    }

    fn emoji_text(&self, emoji: EmojiType) -> String {
        self.state().get_emoji_text(emoji)
    }

    fn services(&self) -> &Services {
        self.state().core.services.as_ref()
    }

    fn stores(&self) -> &Stores {
        self.state().core.stores.as_ref()
    }

    async fn fetch_author_model(&self) -> BotResult<user::Model> {
        Ok(self
            .stores()
            .user
            .fetch_or_create(self.author_id_string())
            .await?)
    }

    async fn fetch_guild_model(&self) -> BotResult<guild::Model> {
        let Some(guild_id) = self.guild_id_string() else {
            return Err(BotError::GuildCommandOnly);
        };
        Ok(self.stores().guild.fetch_or_create(guild_id).await?)
    }

    fn o2d(&self) -> &O2DRenderer {
        &self.state().o2d
    }
}

#[async_trait::async_trait]
impl<'a> ContextExt for Context<'a> {
    fn state(&self) -> &BotState {
        self.data()
    }

    fn author_id_string(&self) -> String {
        self.author().id.to_string()
    }

    fn guild_id_string(&self) -> Option<String> {
        self.guild_id().map(|id| id.to_string())
    }
}
