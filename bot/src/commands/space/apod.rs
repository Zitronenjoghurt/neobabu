use crate::error::BotResult;
use crate::Context;

mod admin;

#[poise::command(slash_command, subcommands("admin::admin"))]
pub async fn apod(_ctx: Context<'_>) -> BotResult<()> {
    Ok(())
}
