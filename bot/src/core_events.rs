use crate::error::BotResult;
use neobabu_core::events::CoreEvent;
use poise::serenity_prelude as serenity;
use tokio::sync::broadcast;
use tracing::{error, info};

mod birthday_notification;

pub async fn listen(ctx: serenity::Context, mut rx: broadcast::Receiver<CoreEvent>) {
    tokio::spawn(async move {
        info!("Core event listener started");

        while let Ok(event) = rx.recv().await {
            let event_type = event.event_type();
            if let Err(err) = handle_event(&ctx, event).await {
                error!("Failed to handle core event '{event_type:?}': {err}")
            }
        }

        error!("Core event listener stopped")
    });
}

async fn handle_event(ctx: &serenity::Context, event: CoreEvent) -> BotResult<()> {
    match event {
        CoreEvent::BirthdayNotification(event) => {
            birthday_notification::handle(ctx, *event).await?
        }
    }
    Ok(())
}
