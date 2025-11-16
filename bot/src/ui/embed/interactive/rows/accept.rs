use crate::ui::embed::interactive::{InteractionResult, InteractiveRow};
use crate::Context;
use poise::serenity_prelude::{ButtonStyle, ComponentInteraction, CreateActionRow, CreateButton};

pub trait InteractiveAcceptRow {
    fn accept(
        &self,
        context: &Context<'_>,
        interaction: &ComponentInteraction,
    ) -> InteractionResult;
    fn deny(&self, context: &Context<'_>, interaction: &ComponentInteraction) -> InteractionResult;

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
    ) -> InteractionResult {
        match interaction.data.custom_id.as_str() {
            "interactive_accept_row_accept" => self.accept(context, interaction),
            "interactive_accept_row_deny" => self.deny(context, interaction),
            _ => InteractionResult::Acknowledge,
        }
    }
}
