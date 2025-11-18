use crate::error::BotResult;
use crate::ui::embed::interactive::response::InteractiveEmbedResponse;
use crate::Context;
use poise::serenity_prelude::{ComponentInteraction, CreateActionRow};

pub mod accept;

#[async_trait::async_trait]
pub trait InteractiveRow: Send + Sync {
    fn render(&self) -> CreateActionRow;

    fn matches(&self, custom_id: &str) -> bool;

    async fn handle(
        &self,
        context: &Context,
        interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveEmbedResponse>;
}
