use crate::error::BotResult;
use crate::Context;

mod start;
mod stats;

#[poise::command(slash_command, guild_only, subcommands("start::start", "stats::stats"))]
pub async fn blackjack(_ctx: Context<'_>) -> BotResult<()> {
    Ok(())
}
