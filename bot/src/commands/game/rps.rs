use crate::error::BotResult;
use crate::Context;

mod challenge;
mod stats;

#[poise::command(
    slash_command,
    guild_only,
    rename = "rock-paper-scissors",
    subcommands("challenge::challenge", "stats::stats")
)]
pub async fn rps(_ctx: Context<'_>) -> BotResult<()> {
    Ok(())
}
