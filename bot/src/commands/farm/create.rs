use crate::context::ContextExt;
use crate::error::BotResult;
use crate::ui::color::UiColor;
use crate::ui::embed::CreateEmbedExt;
use crate::ui::farming::world::show_world;
use crate::Context;
use neobabu_core::error::CoreError;
use neobabu_core::games::farming::hemisphere::Hemisphere;
use poise::serenity_prelude::CreateEmbed;

#[derive(Debug, Clone, Copy, poise::ChoiceParameter)]
pub enum HemisphereOption {
    Northern,
    Southern,
}

impl From<HemisphereOption> for Hemisphere {
    fn from(option: HemisphereOption) -> Self {
        match option {
            HemisphereOption::Northern => Hemisphere::Northern,
            HemisphereOption::Southern => Hemisphere::Southern,
        }
    }
}

/// Create a new farm.
#[poise::command(slash_command)]
pub async fn create(
    ctx: Context<'_>,
    #[min_length = 1]
    #[max_length = 32]
    name: String,
    hemisphere: HemisphereOption,
) -> BotResult<()> {
    let user = ctx.fetch_author_model().await?;
    let world = match ctx
        .services()
        .farming
        .create_world(&user, name, hemisphere.into())
        .await
    {
        Ok(world) => world,
        Err(CoreError::NoPreferredTimezone) => {
            let embed = CreateEmbed::default()
                .ui_color(UiColor::Warning)
                .title("Farming runs in real time!")
                .description("For this to work properly, you need to set up your preferred timezone.\n\nJust use **`/settings timezone`** to do so!\nTimezones will be suggested based on your input, it will also show you the current time in the selected timezone, its super easy :) (If you have 0 clue about timezones, try typing in the capital/close&big city of your country)\n\n*After setting your timezone, just re-run this command!*");
            ctx.send(embed.create_reply().ephemeral(true)).await?;
            return Ok(());
        }
        Err(err) => return Err(err.into()),
    };

    show_world(&ctx, world).await
}
