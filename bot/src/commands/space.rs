use crate::error::BotResult;
use crate::Context;

mod apod;

#[poise::command(slash_command, subcommands("apod::apod"))]
pub async fn space(_ctx: Context<'_>) -> BotResult<()> {
    Ok(())
}
