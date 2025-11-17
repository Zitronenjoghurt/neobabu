use crate::database::entity::user_birthday;
use crate::error::{CoreError, CoreResult};
use crate::events::birthday_notification::BirthdayNotification;
use crate::events::CoreEvent;
use crate::NeobabuCore;
use chrono::{DateTime, Datelike, NaiveDate};
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
    let (month, day) = adjust_for_leap_day(
        today.year(),
        user_birthday.month as u32,
        user_birthday.day as u32,
    );

    let Some(this_years_birthday) = NaiveDate::from_ymd_opt(today.year(), month, day) else {
        return Err(CoreError::invalid_birthday("Invalid date"));
    };

    let already_announced_this_year = match user_birthday.last_announced_at {
        None => false,
        Some(last_announced) => last_announced.date() >= this_years_birthday,
    };
    if already_announced_this_year {
        return Ok(());
    }

    let birthday_passed_this_year = today.date_naive() >= this_years_birthday;

    if birthday_passed_this_year {
        handle_birthday_notification(core, user_birthday, today, this_years_birthday).await?;
        return Ok(());
    }

    // This whole edge-case is specifically for the case if a user's birthday was on the 31st of December
    // and the bot failed to run the job before the 1st of January in the next year.
    let (month, day) = adjust_for_leap_day(
        today.year() - 1,
        user_birthday.month as u32,
        user_birthday.day as u32,
    );

    let Some(last_years_birthday) = NaiveDate::from_ymd_opt(today.year() - 1, month, day) else {
        return Err(CoreError::invalid_birthday("Invalid date"));
    };

    let missed_last_year = match user_birthday.last_announced_at {
        None => true,
        Some(last_announced) => last_announced.date() < last_years_birthday,
    };

    if missed_last_year {
        handle_birthday_notification(core, user_birthday, today, last_years_birthday).await?;
        return Ok(());
    }

    Ok(())
}

async fn handle_birthday_notification(
    core: &NeobabuCore,
    user_birthday: user_birthday::Model,
    today: DateTime<chrono::Utc>,
    last_birthday: NaiveDate,
) -> CoreResult<()> {
    let belated_days = today
        .date_naive()
        .signed_duration_since(last_birthday)
        .num_days();
    if belated_days > 7 {
        return Ok(());
    }
    let is_belated = belated_days > 0;

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

    let mut active_user_birthday = user_birthday.into_active_model();
    active_user_birthday.last_announced_at = Set(Some(today.naive_utc()));
    core.stores
        .user_birthday
        .update(active_user_birthday)
        .await?;

    Ok(())
}

fn adjust_for_leap_day(year: i32, month: u32, day: u32) -> (u32, u32) {
    if month == 2 && day == 29 && !is_leap_year(year) {
        (2, 28)
    } else {
        (month, day)
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
