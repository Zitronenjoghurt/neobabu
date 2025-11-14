use crate::state::BotState;
use poise::serenity_prelude::Ready;
use tracing::info;

pub async fn handle(
    _data_about_bot: &Ready,
    _ctx: &poise::serenity_prelude::Context,
    _state: &BotState,
) {
    info!("Bot online!")
}
