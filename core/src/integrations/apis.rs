use crate::config::Config;
use std::sync::Arc;

pub mod discord;
pub mod nasa_apod;
pub mod youtube;

pub struct Apis {
    pub apod: Arc<nasa_apod::NasaApodApi>,
    pub youtube: Arc<youtube::YoutubeApi>,
}

impl Apis {
    pub fn initialize(config: &Arc<Config>) -> Arc<Self> {
        let apod = Arc::new(nasa_apod::NasaApodApi::new(config.nasa_api_key.clone()));
        let youtube = Arc::new(youtube::YoutubeApi::new(config.youtube_api_key.clone()));
        Arc::new(Self { apod, youtube })
    }
}
