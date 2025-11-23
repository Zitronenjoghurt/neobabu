use crate::error::{BotError, BotResult};
use crate::state::BotState;
use poise::serenity_prelude::FullEvent;

mod guild_create;
mod message;
mod ready;

pub async fn event_handler(
    ctx: &poise::serenity_prelude::Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, BotState, BotError>,
    state: &BotState,
) -> BotResult<()> {
    match event {
        FullEvent::GuildCreate { guild, .. } => guild_create::handle(ctx, state, guild).await?,
        FullEvent::Message { new_message } => message::handle(ctx, state, new_message).await?,
        FullEvent::Ready { data_about_bot } => ready::handle(ctx, state, data_about_bot).await?,
        _ => {}
    }

    Ok(())
}
