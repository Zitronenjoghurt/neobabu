use crate::error::BotResult;
use crate::ui::message::interactive::state::accept::{AcceptState, AcceptStateTrait};
use crate::ui::message::interactive::state::InteractiveStateResponse;
use crate::Context;
use poise::serenity_prelude::{ComponentInteraction, CreateEmbed};

pub struct SimpleAcceptState<T: SimpleAcceptStateTrait> {
    inner: T,
    accepted: Option<bool>,
}

#[async_trait::async_trait]
pub trait SimpleAcceptStateTrait: Sized + Send + Sync {
    async fn embed_question(&self, context: &Context) -> BotResult<CreateEmbed>;
    async fn embed_accepted(&self, context: &Context) -> BotResult<CreateEmbed>;
    async fn embed_denied(&self, context: &Context) -> BotResult<CreateEmbed>;

    async fn on_accept(
        &mut self,
        _context: &Context<'_>,
        _interaction: &ComponentInteraction,
    ) -> BotResult<()> {
        Ok(())
    }

    async fn on_deny(
        &mut self,
        _context: &Context<'_>,
        _interaction: &ComponentInteraction,
    ) -> BotResult<()> {
        Ok(())
    }

    async fn content(&self, _context: &Context) -> BotResult<Option<String>> {
        Ok(None)
    }

    fn accept_text(&self) -> &'static str {
        "Accept"
    }

    fn deny_text(&self) -> &'static str {
        "Deny"
    }

    fn build(self) -> AcceptState<SimpleAcceptState<Self>> {
        AcceptState(SimpleAcceptState {
            inner: self,
            accepted: None,
        })
    }
}

#[async_trait::async_trait]
impl<T: SimpleAcceptStateTrait> AcceptStateTrait for SimpleAcceptState<T> {
    async fn on_accept(
        &mut self,
        context: &Context<'_>,
        interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveStateResponse> {
        self.inner.on_accept(context, interaction).await?;
        self.accepted = Some(true);
        Ok(InteractiveStateResponse::new_halt())
    }

    async fn on_deny(
        &mut self,
        context: &Context<'_>,
        interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveStateResponse> {
        self.inner.on_deny(context, interaction).await?;
        self.accepted = Some(false);
        Ok(InteractiveStateResponse::new_halt())
    }

    async fn embed(&self, context: &Context) -> BotResult<CreateEmbed> {
        match self.accepted {
            Some(true) => self.inner.embed_accepted(context).await,
            Some(false) => self.inner.embed_denied(context).await,
            None => self.inner.embed_question(context).await,
        }
    }

    async fn content(&self, context: &Context) -> BotResult<Option<String>> {
        self.inner.content(context).await
    }

    fn accept_text(&self) -> &'static str {
        self.inner.accept_text()
    }

    fn deny_text(&self) -> &'static str {
        self.inner.deny_text()
    }
}
