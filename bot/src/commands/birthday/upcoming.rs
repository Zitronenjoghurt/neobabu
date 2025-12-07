use crate::context::ContextExt;
use crate::error::{BotError, BotResult};
use crate::ui::color::UiColor;
use crate::ui::message::interactive::state::pagination::PaginationStateTrait;
use crate::ui::message::interactive::InteractiveMessage;
use crate::ui::message::CreateEmbedExt;
use crate::ui::time::format_time_relative_at;
use crate::Context;
use neobabu_core::error::CoreError;
use neobabu_core::types::feature::Feature;
use poise::futures_util::StreamExt;
use poise::serenity_prelude::CreateEmbed;
use std::time::Duration;

const PAGE_SIZE: usize = 30;

/// View upcoming birthdays of users which are active on this server.
#[poise::command(slash_command, guild_only, user_cooldown = "30")]
pub async fn upcoming(ctx: Context<'_>) -> BotResult<()> {
    ctx.defer().await?;

    let guild = ctx.fetch_guild_model().await?;
    let guild_birthday = ctx.stores().guild_birthday.fetch_or_create(&guild).await?;
    if !guild_birthday.enabled {
        return Err(BotError::Core(CoreError::FeatureNotEnabled(
            Feature::Birthday,
        )));
    }

    let total_count = ctx.stores().user_birthday.count_by_guild(&guild.id).await? as usize;
    let state = Upcoming {
        guild_id: guild.id,
        page: 0,
        total_count,
    };

    InteractiveMessage::new(&ctx, state.build())
        .timeout(Duration::from_secs(300))
        .run()
        .await?;

    Ok(())
}

struct Upcoming {
    guild_id: String,
    page: usize,
    total_count: usize,
}

#[async_trait::async_trait]
impl PaginationStateTrait for Upcoming {
    fn get_page(&self) -> usize {
        self.page
    }

    fn set_page(&mut self, page: usize) {
        self.page = page;
    }

    fn max_pages(&self) -> usize {
        self.total_count.div_ceil(PAGE_SIZE)
    }

    async fn render_page(&self, page: usize, ctx: &Context) -> BotResult<CreateEmbed> {
        let limit = PAGE_SIZE as u64;
        let offset = (page * PAGE_SIZE) as u64;
        let mut all_birthdays = ctx
            .stores()
            .user_birthday
            .stream_by_guild(&self.guild_id, Some(limit), Some(offset))
            .await?;

        let mut description = String::new();
        while let Some(user_birthday) = all_birthdays.next().await {
            let user_birthday =
                user_birthday.map_err(|err| BotError::Core(CoreError::Database(err)))?;
            let time = format_time_relative_at(user_birthday.next_birthday.and_utc());
            description.push_str(&format!("{time} <@{}>\n", user_birthday.user_id));
        }

        if description.is_empty() {
            description = "`No upcoming birthdays.`".to_string();
        }

        Ok(CreateEmbed::default()
            .ui_color(UiColor::Orange)
            .title("Upcoming Birthdays")
            .description(description))
    }
}
