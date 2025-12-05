use crate::context::ContextExt;
use crate::error::BotResult;
use crate::ui::autocomplete::autocomplete_timezone;
use crate::ui::color::UiColor;
use crate::ui::embed::interactive::response::InteractiveEmbedResponse;
use crate::ui::embed::interactive::rows::accept::AcceptRowTrait;
use crate::ui::embed::interactive::InteractiveEmbed;
use crate::ui::embed::CreateEmbedExt;
use crate::Context;
use chrono_tz::Tz;
use neobabu_core::database::entity::user;
use neobabu_core::stores::{IntoActiveModel, Set};
use poise::serenity_prelude::{ComponentInteraction, CreateEmbed};
use std::str::FromStr;

/// Set your preferred timezone, which will be used in various bot systems.
#[poise::command(slash_command, ephemeral)]
pub async fn timezone(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_timezone"]
    #[description = "Timezone options will be suggested based on your input."]
    timezone: String,
) -> BotResult<()> {
    ctx.defer_ephemeral().await?;

    let user = ctx.fetch_author_model().await?;
    let tz = chrono_tz::Tz::from_str(&timezone)?;

    let row = TimezoneRow { user, tz };
    InteractiveEmbed::new(&ctx, row.embed())
        .row(row.build())
        .timeout(std::time::Duration::from_secs(120))
        .run()
        .await?;

    Ok(())
}

struct TimezoneRow {
    user: user::Model,
    tz: Tz,
}

impl TimezoneRow {
    pub fn embed(&self) -> CreateEmbed {
        let time = chrono::Utc::now().with_timezone(&self.tz);

        let past_timezone = self
            .user
            .preferred_timezone
            .as_ref()
            .map(|tz| format!("\n\n*Your current preferred timezone is: `{}`*", tz))
            .unwrap_or_default();

        let local_time_12h = time.format("%-I:%M %p").to_string();
        let local_time_24h = time.format("%H:%M").to_string();
        let date = time.format("%A, %B %-d, %Y").to_string();
        let local_time = format!(
            "The current time there is: **`{date}`** at **`{local_time_12h}`** (`{local_time_24h}`)"
        );

        let description = format!(
            "You selected **`{}`**\n{local_time}{past_timezone}",
            self.tz
        );

        CreateEmbed::default()
            .title("Do you want to set your timezone?")
            .description(description)
            .ui_color(UiColor::Yellow)
    }
}

#[async_trait::async_trait]
impl AcceptRowTrait for TimezoneRow {
    async fn accept(
        &self,
        context: &Context<'_>,
        _interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveEmbedResponse> {
        let mut active = self.user.clone().into_active_model();
        active.preferred_timezone = Set(Some(self.tz.to_string()));
        context.stores().user.update(active).await?;

        Ok(InteractiveEmbedResponse::halt_with(
            CreateEmbed::default()
                .ui_color(UiColor::Success)
                .title("Timezone updated")
                .description(format!("Your timezone has been set to **`{}`**", self.tz)),
        ))
    }

    async fn deny(
        &self,
        _context: &Context<'_>,
        _interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveEmbedResponse> {
        Ok(InteractiveEmbedResponse::halt_with(
            CreateEmbed::default()
                .ui_color(UiColor::Gray)
                .title("Timezone change cancelled")
                .description("Your timezone will not be updated"),
        ))
    }

    fn accept_text(&self) -> &'static str {
        "Yes"
    }

    fn deny_text(&self) -> &'static str {
        "No"
    }
}
