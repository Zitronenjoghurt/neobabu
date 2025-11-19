use crate::error::BotResult;
use crate::Context;

mod rps;

#[poise::command(slash_command, subcommands("rps::rps"), guild_only)]
pub async fn game(_ctx: Context<'_>) -> BotResult<()> {
    Ok(())
}
