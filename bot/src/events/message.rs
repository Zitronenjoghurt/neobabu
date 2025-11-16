use crate::error::BotResult;
use crate::state::BotState;

pub async fn handle(
    _ctx: &poise::serenity_prelude::Context,
    state: &BotState,
    message: &poise::serenity_prelude::Message,
) -> BotResult<()> {
    if message.author.bot {
        return Ok(());
    }

    let Some(guild_id) = message.guild_id.map(|id| id.to_string()) else {
        return Ok(());
    };
    let user_id = message.author.id.to_string();

    let user = state.core.stores.user.fetch_or_create(user_id).await?;
    let guild = state.core.stores.guild.fetch_or_create(guild_id).await?;
    let _user_guild = state
        .core
        .stores
        .user_guild
        .fetch_or_create(&user, &guild)
        .await?;

    // For now this is just for registering which guilds a user actually interacted with

    Ok(())
}
