use crate::context::ContextExt;
use crate::error::BotResult;
use crate::ui::color::UiColor;
use crate::ui::embed::CreateEmbedExt;
use crate::ui::emoji::EmojiType;
use crate::utils::formatting::humane_time;
use crate::Context;
use neobabu_core::database::entity::farming_world;
use neobabu_core::games::farming::world::FarmWorldDebugOptions;
use poise::serenity_prelude::{CreateAttachment, CreateEmbed};

pub async fn show_world(ctx: &Context<'_>, world: farming_world::Model) -> BotResult<()> {
    let mut data = world.data()?;
    let png_bytes = data.render_png(ctx.o2d(), FarmWorldDebugOptions::default())?;

    let season = data.current_season();
    let season_emoji = ctx.emoji_text(EmojiType::from(season));
    let time = humane_time(data.current_time());
    let time_info = format!("{season_emoji} **`{season}`** | {time}");

    let description = format!("{time_info}");

    let attachment = CreateAttachment::bytes(png_bytes, "world.png");
    let embed = CreateEmbed::new()
        .ui_color(UiColor::from(season))
        .title(&world.name)
        .description(description)
        .image("attachment://world.png");

    ctx.send(embed.create_reply().attachment(attachment))
        .await?;

    Ok(())
}
