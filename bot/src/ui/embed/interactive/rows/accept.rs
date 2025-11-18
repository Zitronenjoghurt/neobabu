use crate::error::BotResult;
use crate::ui::embed::interactive::response::InteractiveEmbedResponse;
use crate::ui::embed::interactive::rows::InteractiveRow;
use crate::Context;
use poise::serenity_prelude::{ButtonStyle, ComponentInteraction, CreateActionRow, CreateButton};
use std::ops::Deref;

pub struct AcceptRow<T: AcceptRowTrait>(pub T);

impl<T> Deref for AcceptRow<T>
where
    T: AcceptRowTrait,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait::async_trait]
pub trait AcceptRowTrait {
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
impl<T: AcceptRowTrait + Send + Sync> InteractiveRow for AcceptRow<T> {
    fn render(&self, _context: &Context) -> Option<CreateActionRow> {
        Some(CreateActionRow::Buttons(vec![
            CreateButton::new("accept_row_accept")
                .style(ButtonStyle::Success)
                .label(self.0.accept_text()),
            CreateButton::new("accept_row_deny")
                .style(ButtonStyle::Danger)
                .label(self.0.deny_text()),
        ]))
    }

    fn matches(&self, custom_id: &str) -> bool {
        custom_id == "accept_row_accept" || custom_id == "accept_row_deny"
    }

    async fn handle(
        &mut self,
        context: &Context,
        interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveEmbedResponse> {
        match interaction.data.custom_id.as_str() {
            "accept_row_accept" => self.0.accept(context, interaction).await,
            "accept_row_deny" => self.0.deny(context, interaction).await,
            _ => Ok(InteractiveEmbedResponse::new()),
        }
    }
}
