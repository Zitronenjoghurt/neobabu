use crate::database::entity::{youtube_channel, youtube_video};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct NewYoutubeVideo {
    pub channel_model: youtube_channel::Model,
    pub video_model: youtube_video::Model,
    pub video_duration: Duration,
    pub is_live: bool,
    pub is_upcoming_live: bool,
}
