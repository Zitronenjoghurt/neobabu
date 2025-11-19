use crate::error::BotResult;
use crate::Context;

mod challenge;

#[poise::command(
    slash_command,
    guild_only,
    rename = "rock-paper-scissors",
    subcommands("challenge::challenge")
)]
pub async fn rps(_ctx: Context<'_>) -> BotResult<()> {
    Ok(())
}
