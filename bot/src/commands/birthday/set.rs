use crate::context::ContextExt;
use crate::error::BotResult;
use crate::ui::color::UiColor;
use crate::ui::embed::interactive::response::InteractiveEmbedResponse;
use crate::ui::embed::interactive::rows::accept::{AcceptRow, AcceptRowTrait};
use crate::ui::embed::interactive::InteractiveEmbed;
use crate::ui::embed::CreateEmbedExt;
use crate::Context;
use poise::serenity_prelude::{ComponentInteraction, CreateEmbed};

/// Set your birthday, which will be announced if enabled by an administrator.
#[poise::command(slash_command, ephemeral, user_cooldown = "30")]
pub async fn set(
    ctx: Context<'_>,
    #[description = "Your birth month"]
    #[min = 1]
    #[max = 12]
    month: i16,
    #[description = "Your birth day"]
    #[min = 1]
    #[max = 31]
    day: i16,
    #[description = "Your birth year"]
    #[min = 1900]
    year: Option<i16>,
) -> BotResult<()> {
    let mut embed = CreateEmbed::default()
        .warning_user(&ctx.author())
        .title("Do you want to set your birthday?")
        .description("Your birthday will be set globally and may be **announced** on servers you have interacted with (where this bot is on).\n\nYou will **not** be able to change it again for a while. If you did not specify your birth year the bot will not announce your age.\n\n**Are you sure you want to proceed?**")
        .field("Day", day.to_string(), true)
        .field("Month", month.to_string(), true);

    if let Some(year) = year {
        embed = embed.field("Year", year.to_string(), true);
        embed = embed.footer_text(
            "Since you specified your birth year, people will be able to know your age.",
        )
    }

    InteractiveEmbed::new(&ctx, embed)
        .row(AcceptRow(BirthdaySetRow { day, month, year }))
        .timeout(std::time::Duration::from_secs(120))
        .run()
        .await?;

    Ok(())
}

struct BirthdaySetRow {
    day: i16,
    month: i16,
    year: Option<i16>,
}

#[async_trait::async_trait]
impl AcceptRowTrait for BirthdaySetRow {
    async fn accept(
        &self,
        context: &Context<'_>,
        _interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveEmbedResponse> {
        let user = context.fetch_author_model().await?;
        context
            .services()
            .birthday
            .set_birthday(&user, self.day, self.month, self.year)
            .await?;

        Ok(InteractiveEmbedResponse::halt_with(
            CreateEmbed::default()
                .success_user(&context.author())
                .title("Birthday set")
                .description("Your birthday was set successfully."),
        ))
    }

    async fn deny(
        &self,
        context: &Context<'_>,
        _interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveEmbedResponse> {
        Ok(InteractiveEmbedResponse::halt_with(
            CreateEmbed::default()
                .ui_color(UiColor::Gray)
                .user(&context.author())
                .title("Birthday not set")
                .description("Your birthday was not set."),
        ))
    }

    fn accept_text(&self) -> &'static str {
        "Yes"
    }

    fn deny_text(&self) -> &'static str {
        "No"
    }
}
