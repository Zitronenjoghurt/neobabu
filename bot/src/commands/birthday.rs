use crate::error::BotResult;
use crate::Context;

mod admin;
mod set;
mod upcoming;

#[poise::command(
    slash_command,
    subcommands("admin::admin", "set::set", "upcoming::upcoming"),
    guild_only
)]
pub async fn birthday(_ctx: Context<'_>) -> BotResult<()> {
    Ok(())
}
