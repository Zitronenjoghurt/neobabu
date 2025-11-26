use crate::database::entity::apod;
use crate::events::birthday_dm::BirthdayDM;
use crate::events::birthday_notification::BirthdayNotification;
use std::sync::Arc;
use tokio::sync::broadcast;

pub mod birthday_dm;
pub mod birthday_notification;
pub mod new_youtube_video;

#[derive(Debug, Clone, Copy)]
pub enum CoreEventType {
    BirthdayDM,
    BirthdayNotification,
    NewApod,
    NewYoutubeVideo,
}

#[derive(Debug, Clone)]
pub enum CoreEvent {
    BirthdayDM(Box<BirthdayDM>),
    BirthdayNotification(Box<BirthdayNotification>),
    NewApod(Box<apod::Model>),
    NewYoutubeVideo(Box<new_youtube_video::NewYoutubeVideo>),
}

impl CoreEvent {
    pub fn event_type(&self) -> CoreEventType {
        match self {
            Self::BirthdayDM(_) => CoreEventType::BirthdayDM,
            Self::BirthdayNotification(_) => CoreEventType::BirthdayNotification,
            Self::NewApod(_) => CoreEventType::NewApod,
            Self::NewYoutubeVideo(_) => CoreEventType::NewYoutubeVideo,
        }
    }

    pub fn birthday_dm(user_id: impl Into<String>, is_belated: bool) -> Self {
        Self::BirthdayDM(Box::new(BirthdayDM {
            user_id: user_id.into(),
            is_belated,
        }))
    }

    pub fn birthday_notification(notification: BirthdayNotification) -> Self {
        Self::BirthdayNotification(Box::new(notification))
    }

    pub fn new_apod(apod: apod::Model) -> Self {
        Self::NewApod(Box::new(apod))
    }

    pub fn new_youtube_video(video: new_youtube_video::NewYoutubeVideo) -> Self {
        Self::NewYoutubeVideo(Box::new(video))
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
