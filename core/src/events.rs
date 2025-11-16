use crate::events::birthday_notification::BirthdayNotification;
use std::sync::Arc;
use tokio::sync::broadcast;

pub mod birthday_notification;

#[derive(Debug, Clone, Copy)]
pub enum CoreEventType {
    BirthdayNotification,
}

#[derive(Debug, Clone)]
pub enum CoreEvent {
    BirthdayNotification(Box<BirthdayNotification>),
}

impl CoreEvent {
    pub fn event_type(&self) -> CoreEventType {
        match self {
            Self::BirthdayNotification(_) => CoreEventType::BirthdayNotification,
        }
    }

    pub fn birthday_notification(notification: BirthdayNotification) -> Self {
        Self::BirthdayNotification(Box::new(notification))
    }
}

#[derive(Clone)]
pub struct CoreEventBus {
    tx: broadcast::Sender<CoreEvent>,
}

impl CoreEventBus {
    pub fn initialize() -> Arc<Self> {
        let (tx, _) = broadcast::channel(1000);
        Arc::new(Self { tx })
    }

    pub fn send(&self, event: CoreEvent) {
        let event_type = event.event_type();
        match self.tx.send(event) {
            Ok(_) => {
                tracing::debug!("Sent core event '{event_type:?}'");
            }
            Err(err) => tracing::error!("Failed to send core event '{event_type:?}': {err}"),
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<CoreEvent> {
        self.tx.subscribe()
    }
}
