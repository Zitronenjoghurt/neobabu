use crate::error::BotResult;
use neobabu_core::events::birthday_dm::BirthdayDM;
use poise::serenity_prelude::{Context, CreateMessage, UserId};

pub async fn handle(ctx: &Context, event: BirthdayDM) -> BotResult<()> {
    let user_id = UserId::new(event.user_id.parse()?);

    let belated_text = if event.is_belated { " BELATED" } else { "" };
    let message = format!("🎂 **HAPPY{belated_text} BIRTHDAY!** 🎉");
    user_id
        .dm(ctx, CreateMessage::new().content(message))
        .await?;

    Ok(())
}
