use crate::error::BotResult;
use crate::ui::embed::interactive::response::InteractiveEmbedResponse;
use crate::ui::embed::interactive::rows::InteractiveRow;
use crate::Context;
use poise::serenity_prelude::{ButtonStyle, ComponentInteraction, CreateActionRow, CreateButton};

#[async_trait::async_trait]
pub trait InteractiveAcceptRow {
    async fn accept(
        &self,
        context: &Context<'_>,
        interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveEmbedResponse>;
    async fn deny(
        &self,
        context: &Context<'_>,
        interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveEmbedResponse>;

    fn accept_text(&self) -> &'static str {
        "Accept"
    }

    fn deny_text(&self) -> &'static str {
        "Deny"
    }
}

#[async_trait::async_trait]
impl<T: InteractiveAcceptRow + Send + Sync> InteractiveRow for T {
    fn render(&self) -> CreateActionRow {
        CreateActionRow::Buttons(vec![
            CreateButton::new("interactive_accept_row_accept")
                .style(ButtonStyle::Success)
                .label(self.accept_text()),
            CreateButton::new("interactive_accept_row_deny")
                .style(ButtonStyle::Danger)
                .label(self.deny_text()),
        ])
    }

    fn matches(&self, custom_id: &str) -> bool {
        custom_id == "interactive_accept_row_accept" || custom_id == "interactive_accept_row_deny"
    }

    async fn handle(
        &self,
        context: &Context,
        interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveEmbedResponse> {
        match interaction.data.custom_id.as_str() {
            "interactive_accept_row_accept" => self.accept(context, interaction).await,
            "interactive_accept_row_deny" => self.deny(context, interaction).await,
            _ => Ok(InteractiveEmbedResponse::new()),
        }
    }
}
