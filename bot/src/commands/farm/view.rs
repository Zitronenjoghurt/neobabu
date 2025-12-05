use crate::context::ContextExt;
use crate::error::BotResult;
use crate::ui::farming::world::show_world;
use crate::Context;
use poise::CreateReply;

/// View your farm.
#[poise::command(slash_command, user_cooldown = "15")]
pub async fn view(ctx: Context<'_>) -> BotResult<()> {
    // ToDo: Select which farm to view, options with the farm names, improve error message
    let Some(world) = ctx
        .stores()
        .farming_world
        .find(ctx.author_id_string(), 0)
        .await?
    else {
        ctx.send(
            CreateReply::default()
                .content("You do not have any farm yet!")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    };

    show_world(&ctx, world).await
}
