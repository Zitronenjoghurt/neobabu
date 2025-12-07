use crate::error::BotResult;
use crate::Context;

mod blackjack;
mod rps;

#[poise::command(
    slash_command,
    subcommands("blackjack::blackjack", "rps::rps"),
    guild_only
)]
pub async fn game(_ctx: Context<'_>) -> BotResult<()> {
    Ok(())
}
