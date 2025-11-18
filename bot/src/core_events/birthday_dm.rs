use crate::error::BotResult;
use crate::state::BotState;
use crate::ui::emoji::EmojiType;
use neobabu_core::events::birthday_dm::BirthdayDM;
use poise::serenity_prelude::{Context, CreateMessage, UserId};

pub async fn handle(ctx: &Context, state: &BotState, event: BirthdayDM) -> BotResult<()> {
    let user_id = UserId::new(event.user_id.parse()?);

    let balloon = state.get_emoji_text(EmojiType::BalloonRed);
    let sparkle = state.get_emoji_text(EmojiType::Sparkle);
    let belated_text = if event.is_belated { " BELATED" } else { "" };
    let message = format!("{balloon} **HAPPY{belated_text} BIRTHDAY!** {sparkle}");
    user_id
        .dm(ctx, CreateMessage::new().content(message))
        .await?;

    Ok(())
}
