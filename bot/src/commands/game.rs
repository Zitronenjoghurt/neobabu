use crate::error::BotResult;
use crate::Context;

mod rps;

#[poise::command(slash_command, subcommands("rps::rock_paper_scissors"), guild_only)]
pub async fn game(_ctx: Context<'_>) -> BotResult<()> {
    Ok(())
}
