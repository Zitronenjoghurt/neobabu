use crate::error::BotResult;
use crate::ui::message::interactive::state::{InteractiveState, InteractiveStateResponse};
use crate::Context;
use poise::serenity_prelude::{
    ButtonStyle, ComponentInteraction, CreateActionRow, CreateButton, CreateEmbed,
};

pub struct AcceptState<T: AcceptStateTrait>(pub T);

#[async_trait::async_trait]
pub trait AcceptStateTrait: Sized + Send + Sync {
    async fn on_accept(
        &mut self,
        ctx: &Context<'_>,
        interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveStateResponse>;

    async fn on_deny(
        &mut self,
        ctx: &Context<'_>,
        interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveStateResponse>;

    async fn embed(&self, ctx: &Context) -> BotResult<CreateEmbed>;

    async fn content(&self, _ctx: &Context) -> BotResult<Option<String>> {
        Ok(None)
    }

    fn accept_text(&self) -> &'static str {
        "Accept"
    }

    fn deny_text(&self) -> &'static str {
        "Deny"
    }

    fn build(self) -> AcceptState<Self> {
        AcceptState(self)
    }
}

#[async_trait::async_trait]
impl<T: AcceptStateTrait> InteractiveState for AcceptState<T> {
    async fn handle_interaction(
        &mut self,
        ctx: &Context,
        interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveStateResponse> {
        match interaction.data.custom_id.as_str() {
            "accept_row_accept" => self.0.on_accept(ctx, interaction).await,
            "accept_row_deny" => self.0.on_deny(ctx, interaction).await,
            _ => Ok(InteractiveStateResponse::default()),
        }
    }

    async fn render_content(&self, ctx: &Context) -> BotResult<Option<String>> {
        self.0.content(ctx).await
    }

    async fn render_embed(&self, ctx: &Context) -> BotResult<CreateEmbed> {
        self.0.embed(ctx).await
    }

    async fn render_rows(&self, _ctx: &Context) -> BotResult<Vec<CreateActionRow>> {
        Ok(vec![CreateActionRow::Buttons(vec![
            CreateButton::new("accept_row_accept")
                .style(ButtonStyle::Success)
                .label(self.0.accept_text()),
            CreateButton::new("accept_row_deny")
                .style(ButtonStyle::Danger)
                .label(self.0.deny_text()),
        ])])
    }
}
