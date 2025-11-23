use crate::error::BotResult;
use crate::state::BotState;

pub async fn handle(
    _ctx: &poise::serenity_prelude::Context,
    state: &BotState,
    guild: &poise::serenity_prelude::Guild,
) -> BotResult<()> {
    state
        .core
        .stores
        .guild
        .fetch_or_create(guild.id.to_string())
        .await?;
    Ok(())
}
