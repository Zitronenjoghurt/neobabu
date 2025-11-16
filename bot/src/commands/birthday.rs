use crate::error::BotResult;
use crate::Context;

mod admin;
mod set;

#[poise::command(slash_command, subcommands("admin::admin", "set::set"), guild_only)]
pub async fn birthday(_ctx: Context<'_>) -> BotResult<()> {
    Ok(())
}
