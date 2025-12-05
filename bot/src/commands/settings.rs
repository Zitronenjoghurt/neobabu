use crate::error::BotResult;
use crate::Context;

mod timezone;

#[poise::command(slash_command, subcommands("timezone::timezone"), ephemeral)]
pub async fn settings(_ctx: Context<'_>) -> BotResult<()> {
    Ok(())
}
