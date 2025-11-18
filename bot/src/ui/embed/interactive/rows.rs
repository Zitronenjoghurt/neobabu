use crate::error::BotResult;
use crate::ui::embed::interactive::response::InteractiveEmbedResponse;
use crate::Context;
use poise::serenity_prelude::{ComponentInteraction, CreateActionRow};

pub mod accept;
pub mod pagination;

#[async_trait::async_trait]
pub trait InteractiveRow: Send + Sync {
    fn render(&self, context: &Context) -> Option<CreateActionRow>;

    fn matches(&self, custom_id: &str) -> bool;

    async fn handle(
        &mut self,
        context: &Context,
        interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveEmbedResponse>;
}
