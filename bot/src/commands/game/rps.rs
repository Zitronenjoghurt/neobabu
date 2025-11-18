use crate::error::BotResult;
use crate::ui::embed::interactive::response::InteractiveEmbedResponse;
use crate::ui::embed::interactive::rows::InteractiveRow;
use crate::Context;
use neobabu_core::services::rock_paper_scissors::RPSChoice;
use poise::serenity_prelude::{ComponentInteraction, CreateActionRow, UserId};

#[poise::command(slash_command, guild_only)]
pub async fn rock_paper_scissors(_ctx: Context<'_>) -> BotResult<()> {
    Ok(())
}

struct RPSRow {
    user_1: UserId,
    user_2: UserId,
    choice_1: Option<RPSChoice>,
    choice_2: Option<RPSChoice>,
}

#[async_trait::async_trait]
impl InteractiveRow for RPSRow {
    fn render(&self, context: &Context) -> Option<CreateActionRow> {
        todo!()
    }

    fn matches(&self, custom_id: &str) -> bool {
        todo!()
    }

    async fn handle(
        &mut self,
        context: &Context,
        interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveEmbedResponse> {
        todo!()
    }
}
