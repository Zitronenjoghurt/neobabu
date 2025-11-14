use crate::error::{BotError, BotResult};
use crate::state::BotState;
use poise::serenity_prelude::FullEvent;

mod ready;

pub async fn event_handler(
    ctx: &poise::serenity_prelude::Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, BotState, BotError>,
    state: &BotState,
) -> BotResult<()> {
    match event {
        FullEvent::Ready { data_about_bot } => ready::handle(data_about_bot, ctx, state).await,
        _ => {}
    }

    Ok(())
}
