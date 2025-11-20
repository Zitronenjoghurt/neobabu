use crate::error::BotResult;
use crate::state::BotState;
use neobabu_core::events::CoreEvent;
use poise::serenity_prelude as serenity;
use tokio::sync::broadcast;
use tracing::{error, info};

mod birthday_dm;
mod birthday_notification;
mod new_apod;

pub async fn listen(
    ctx: serenity::Context,
    state: BotState,
    mut rx: broadcast::Receiver<CoreEvent>,
) {
    tokio::spawn(async move {
        info!("Core event listener started");

        while let Ok(event) = rx.recv().await {
            let event_type = event.event_type();
            if let Err(err) = handle_event(&ctx, &state, event).await {
                error!("Failed to handle core event '{event_type:?}': {err}")
            }
        }

        error!("Core event listener stopped")
    });
}

async fn handle_event(
    ctx: &serenity::Context,
    state: &BotState,
    event: CoreEvent,
) -> BotResult<()> {
    match event {
        CoreEvent::BirthdayDM(event) => birthday_dm::handle(ctx, state, *event).await?,
        CoreEvent::BirthdayNotification(event) => {
            birthday_notification::handle(ctx, state, *event).await?
        }
        CoreEvent::NewApod(apod) => new_apod::handle(ctx, state, *apod).await?,
    }
    Ok(())
}
