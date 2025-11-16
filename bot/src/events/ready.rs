use crate::error::BotResult;
use crate::state::BotState;
use poise::serenity_prelude::Ready;
use tracing::info;

pub async fn handle(
    _ctx: &poise::serenity_prelude::Context,
    _state: &BotState,
    _data_about_bot: &Ready,
) -> BotResult<()> {
    info!("Bot online!");
    Ok(())
}
