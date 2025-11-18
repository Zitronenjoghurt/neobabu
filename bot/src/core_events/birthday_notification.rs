use crate::error::BotResult;
use crate::state::BotState;
use crate::ui::emoji::EmojiType;
use neobabu_core::events::birthday_notification::BirthdayNotification;
use poise::serenity_prelude::{ChannelId, Context, CreateMessage, GuildId, UserId};

pub async fn handle(ctx: &Context, state: &BotState, event: BirthdayNotification) -> BotResult<()> {
    let guild_id = GuildId::new(event.guild_id.parse()?);
    let user_id = UserId::new(event.user_id.parse()?);
    let channel_id = ChannelId::new(event.channel_id.parse()?);

    let Ok(_member) = guild_id.member(ctx, user_id).await else {
        return Ok(());
    };

    let balloon = state.get_emoji_text(EmojiType::BalloonRed);
    let sparkle = state.get_emoji_text(EmojiType::Sparkle);
    let belated_text = if event.is_belated { " belated" } else { "" };
    let age_text = match event.age {
        Some(age) => format!(" turned `{age}`"),
        None => "".to_string(),
    };

    let server_message =
        format!("{balloon} <@{user_id}>{age_text}, **Happy{belated_text} birthday!** {sparkle}");
    channel_id
        .send_message(ctx, CreateMessage::new().content(server_message))
        .await?;

    Ok(())
}
