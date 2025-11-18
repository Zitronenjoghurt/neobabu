use crate::database::entity::user_birthday;
use crate::error::{CoreError, CoreResult};
use crate::events::birthday_notification::BirthdayNotification;
use crate::events::CoreEvent;
use crate::utils::upcoming_date_time;
use crate::NeobabuCore;
use chrono::{DateTime, Datelike};
use futures::StreamExt;
use sea_orm::{IntoActiveModel, Set};
use tracing::{error, info};

pub async fn run(core: NeobabuCore) -> CoreResult<()> {
    let today = chrono::Utc::now();

    info!("Processing birthdays...");

    let mut user_birthdays = core.stores.user_birthday.stream_all().await?;
    let mut count: u32 = 0;
    while let Some(user_birthday) = user_birthdays.next().await {
        let user_birthday = user_birthday?;
        let user_id = user_birthday.user_id.clone();
        if let Err(err) = handle_user_birthday(&core, user_birthday, today).await {
            error!(
                "Failed to handle user birthday for user '{}': {err}",
                user_id
            )
        } else {
            count += 1;
        }
    }

    info!("Successfully processed {count} birthdays");

    Ok(())
}

async fn handle_user_birthday(
    core: &NeobabuCore,
    user_birthday: user_birthday::Model,
    today: DateTime<chrono::Utc>,
) -> CoreResult<()> {
    let next_birthday = user_birthday.next_birthday.and_utc();
    if next_birthday > today {
        return Ok(());
    }

    let is_belated = next_birthday.date_naive() != today.date_naive();
    let age = user_birthday
        .year
        .map(|year| (today.year() - year as i32) as u8);

    core.event_bus
        .send(CoreEvent::birthday_dm(&user_birthday.user_id, is_belated));

    let mut user_guilds = core
        .stores
        .user_guild
        .stream_with_user_id(&user_birthday.user_id)
        .await?;
    while let Some(user_guild) = user_guilds.next().await {
        let user_guild = user_guild?;

        let Some(guild_birthday) = core
            .stores
            .guild_birthday
            .find_by_guild_id(&user_guild.guild_id)
            .await?
        else {
            continue;
        };

        if !guild_birthday.enabled {
            continue;
        }

        let Some(channel_id) = guild_birthday.notification_channel_id else {
            continue;
        };

        let event = BirthdayNotification {
            user_id: user_birthday.user_id.to_string(),
            guild_id: user_guild.guild_id,
            channel_id,
            age,
            is_belated,
        };
        core.event_bus.send(CoreEvent::birthday_notification(event));
    }
    drop(user_guilds);

    let next_birthday = upcoming_date_time(user_birthday.day as u32, user_birthday.month as u32)
        .ok_or(CoreError::invalid_birthday("Invalid date."))?;
    let mut active_user_birthday = user_birthday.into_active_model();
    active_user_birthday.next_birthday = Set(next_birthday.naive_utc());
    core.stores
        .user_birthday
        .update(active_user_birthday)
        .await?;

    Ok(())
}
