use crate::error::BotResult;
use crate::Context;

mod start;

#[poise::command(slash_command, guild_only, subcommands("start::start"))]
pub async fn blackjack(_ctx: Context<'_>) -> BotResult<()> {
    Ok(())
}
