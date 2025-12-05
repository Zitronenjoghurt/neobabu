use crate::context::ContextExt;
use crate::error::BotResult;
use crate::ui::embed::CreateEmbedExt;
use crate::Context;
use neobabu_core::database::entity::farming_world;
use neobabu_core::games::farming::world::FarmWorldDebugOptions;
use poise::serenity_prelude::{CreateAttachment, CreateEmbed};

pub async fn show_world(ctx: &Context<'_>, world: farming_world::Model) -> BotResult<()> {
    let mut data = world.data()?;
    let png_bytes = data.render_png(ctx.o2d(), FarmWorldDebugOptions::default())?;

    let attachment = CreateAttachment::bytes(png_bytes, "world.png");
    let embed = CreateEmbed::new()
        .title(&world.name)
        .image("attachment://world.png");

    ctx.send(embed.create_reply().attachment(attachment))
        .await?;

    Ok(())
}
