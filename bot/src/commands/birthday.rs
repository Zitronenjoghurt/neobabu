use crate::error::BotResult;
use crate::Context;

mod set;

#[poise::command(slash_command, subcommands("set::set"), guild_only)]
pub async fn birthday(_ctx: Context<'_>) -> BotResult<()> {
    Ok(())
}
