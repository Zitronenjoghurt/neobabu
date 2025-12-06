use crate::context::ContextExt;
use crate::error::{BotError, BotResult};
use crate::ui::farming::autocomplete::{autocomplete_extract_world_index, autocomplete_world_name};
use crate::ui::farming::world::show_world;
use crate::Context;

/// View your farm.
#[poise::command(slash_command, user_cooldown = "15")]
pub async fn view(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_world_name"] farm_name: String,
) -> BotResult<()> {
    ctx.defer().await?;

    let index = autocomplete_extract_world_index(&farm_name);
    let Some(world) = ctx
        .stores()
        .farming_world
        .find(ctx.author_id_string(), index)
        .await?
    else {
        return Err(BotError::FarmNotFound);
    };

    show_world(&ctx, world).await
}
