use crate::error::BotResult;
use crate::Context;
use poise::serenity_prelude::{ComponentInteraction, CreateActionRow, CreateEmbed};

pub mod accept;
pub mod pagination;
pub mod simple_accept;

#[async_trait::async_trait]
pub trait InteractiveState: Send + Sync {
    async fn handle_interaction(
        &mut self,
        ctx: &Context,
        interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveStateResponse>;
    async fn render_embed(&self, ctx: &Context) -> BotResult<CreateEmbed>;
    async fn render_rows(&self, ctx: &Context) -> BotResult<Vec<CreateActionRow>>;

    async fn render_content(&self, _ctx: &Context) -> BotResult<Option<String>> {
        Ok(None)
    }

    async fn on_tick(&mut self, _ctx: &Context) -> BotResult<InteractiveStateResponse> {
        Ok(InteractiveStateResponse::default())
    }
}

#[derive(Default)]
pub struct InteractiveStateResponse {
    pub do_update: bool,
    pub do_stop: bool,
}

impl InteractiveStateResponse {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_halt() -> Self {
        Self {
            do_update: true,
            do_stop: true,
        }
    }

    pub fn new_update() -> Self {
        Self {
            do_update: true,
            ..Default::default()
        }
    }

    pub fn stop(mut self, stop: bool) -> Self {
        self.do_stop = stop;
        self
    }

    pub fn update(mut self, update: bool) -> Self {
        self.do_update = update;
        self
    }
}
