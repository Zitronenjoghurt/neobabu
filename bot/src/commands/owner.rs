use crate::error::{BotError, BotResult};
use crate::Context;

#[poise::command(prefix_command, owners_only, hide_in_help, aliases("sg"))]
pub async fn sync_guild(ctx: Context<'_>) -> BotResult<()> {
    let guild_id = ctx.guild_id().ok_or(BotError::GuildCommandOnly)?;

    poise::builtins::register_in_guild(
        ctx.serenity_context(),
        &ctx.framework().options().commands,
        guild_id,
    )
    .await?;

    ctx.reply("Application commands synced for current guild")
        .await?;

    Ok(())
}
