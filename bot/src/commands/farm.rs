use crate::error::BotResult;
use crate::Context;

mod create;
mod view;

#[poise::command(slash_command, subcommands("create::create", "view::view"))]
pub async fn farm(_ctx: Context<'_>) -> BotResult<()> {
    Ok(())
}
