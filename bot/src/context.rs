use crate::error::{BotError, BotResult};
use crate::Context;
use neobabu_core::database::entity::{guild, user};
use neobabu_core::services::Services;
use neobabu_core::stores::Stores;

#[async_trait::async_trait]
pub trait ContextExt {
    fn services(&self) -> &Services;
    fn stores(&self) -> &Stores;
    fn author_id_string(&self) -> String;
    fn guild_id_string(&self) -> Option<String>;

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
}

#[async_trait::async_trait]
impl<'a> ContextExt for Context<'a> {
    fn services(&self) -> &Services {
        self.data().core.services.as_ref()
    }

    fn stores(&self) -> &Stores {
        self.data().core.stores.as_ref()
    }

    fn author_id_string(&self) -> String {
        self.author().id.to_string()
    }

    fn guild_id_string(&self) -> Option<String> {
        self.guild_id().map(|id| id.to_string())
    }
}
