use crate::context::ContextExt;
use crate::error::BotResult;
use crate::Context;

#[poise::command(slash_command, guild_only)]
pub async fn set(ctx: Context<'_>) -> BotResult<()> {
    let user = ctx.fetch_author().await?;
    let guild = ctx.fetch_guild().await?;

    todo!()
    //ctx.services().birthday.set_birthday()
}
