use crate::context::ContextExt;
use crate::error::BotResult;
use crate::ui::autocomplete::autocomplete_timezone;
use crate::ui::color::UiColor;
use crate::ui::message::interactive::state::simple_accept::SimpleAcceptStateTrait;
use crate::ui::message::interactive::InteractiveMessage;
use crate::ui::message::CreateEmbedExt;
use crate::utils::formatting::humane_datetime;
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
    let state = TimezoneState { user, tz };

    InteractiveMessage::new(&ctx, state.build())
        .timeout(std::time::Duration::from_secs(120))
        .run()
        .await?;

    Ok(())
}

struct TimezoneState {
    user: user::Model,
    tz: Tz,
}

#[async_trait::async_trait]
impl SimpleAcceptStateTrait for TimezoneState {
    async fn embed_question(&self, _context: &Context) -> BotResult<CreateEmbed> {
        let time = chrono::Utc::now().with_timezone(&self.tz);

        let past_timezone = self
            .user
            .preferred_timezone
            .as_ref()
            .map(|tz| format!("\n\n*Your current preferred timezone is: `{}`*", tz))
            .unwrap_or_default();

        let time_formatted = humane_datetime(time);
        let local_time = format!("The current time there is: {time_formatted}");

        let description = format!(
            "You selected **`{}`**\n{local_time}{past_timezone}",
            self.tz
        );

        Ok(CreateEmbed::default()
            .title("Do you want to set your timezone?")
            .description(description)
            .ui_color(UiColor::Yellow))
    }

    async fn embed_accepted(&self, _context: &Context) -> BotResult<CreateEmbed> {
        Ok(CreateEmbed::default()
            .ui_color(UiColor::Success)
            .title("Timezone updated")
            .description(format!("Your timezone has been set to **`{}`**", self.tz)))
    }

    async fn embed_denied(&self, _context: &Context) -> BotResult<CreateEmbed> {
        Ok(CreateEmbed::default()
            .ui_color(UiColor::Gray)
            .title("Timezone change cancelled")
            .description("Your timezone will not be updated"))
    }

    async fn on_accept(
        &mut self,
        context: &Context<'_>,
        _interaction: &ComponentInteraction,
    ) -> BotResult<()> {
        let mut active = self.user.clone().into_active_model();
        active.preferred_timezone = Set(Some(self.tz.to_string()));
        context.stores().user.update(active).await?;
        Ok(())
    }

    fn accept_text(&self) -> &'static str {
        "Yes"
    }

    fn deny_text(&self) -> &'static str {
        "No"
    }
}
