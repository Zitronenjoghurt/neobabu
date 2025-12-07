use crate::context::ContextExt;
use crate::error::BotResult;
use crate::ui::color::UiColor;
use crate::ui::message::interactive::state::simple_accept::SimpleAcceptStateTrait;
use crate::ui::message::interactive::InteractiveMessage;
use crate::ui::message::CreateEmbedExt;
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
    ctx.defer_ephemeral().await?;

    let state = BirthdaySet { day, month, year };
    InteractiveMessage::new(&ctx, state.build())
        .timeout(std::time::Duration::from_secs(120))
        .run()
        .await?;

    Ok(())
}

struct BirthdaySet {
    day: i16,
    month: i16,
    year: Option<i16>,
}

#[async_trait::async_trait]
impl SimpleAcceptStateTrait for BirthdaySet {
    async fn embed_question(&self, ctx: &Context) -> BotResult<CreateEmbed> {
        let mut embed = CreateEmbed::default()
            .warning_user(ctx.author())
            .title("Do you want to set your birthday?")
            .description("Your birthday will be set globally and may be **announced** on servers you have interacted with (where this bot is on).\n\nYou will **not** be able to change it again for a while. If you did not specify your birth year the bot will not announce your age.\n\n**Are you sure you want to proceed?**")
            .field("Day", self.day.to_string(), true)
            .field("Month", self.month.to_string(), true);

        if let Some(year) = self.year {
            embed = embed.field("Year", year.to_string(), true);
            embed = embed.footer_text(
                "Since you specified your birth year, people will be able to know your age.",
            )
        };

        Ok(embed)
    }

    async fn embed_accepted(&self, ctx: &Context) -> BotResult<CreateEmbed> {
        Ok(CreateEmbed::default()
            .success_user(ctx.author())
            .title("Birthday set")
            .description("Your birthday was set successfully."))
    }

    async fn embed_denied(&self, ctx: &Context) -> BotResult<CreateEmbed> {
        Ok(CreateEmbed::default()
            .ui_color(UiColor::Gray)
            .user(ctx.author())
            .title("Birthday not set")
            .description("Your birthday was not set."))
    }

    async fn on_accept(
        &mut self,
        ctx: &Context<'_>,
        _interaction: &ComponentInteraction,
    ) -> BotResult<()> {
        let user = ctx.fetch_author_model().await?;
        ctx.services()
            .birthday
            .set_birthday(&user, self.day, self.month, self.year)
            .await?;
        Ok(())
    }

    fn accept_text(&self) -> &'static str {
        "Yes"
    }

    fn deny_text(&self) -> &'static str {
        "No"
    }
}
