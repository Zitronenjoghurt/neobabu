use crate::error::BotResult;
use crate::Context;

mod settings;
mod subscribe;

#[poise::command(
    slash_command,
    subcommands("settings::settings", "subscribe::subscribe"),
    guild_only,
    ephemeral
)]
pub async fn youtube(_ctx: Context<'_>) -> BotResult<()> {
    Ok(())
}
