use crate::config::Config;
use std::sync::Arc;

pub mod nasa_apod;

pub struct Apis {
    pub apod: Arc<nasa_apod::NasaApodApi>,
}

impl Apis {
    pub fn initialize(config: &Arc<Config>) -> Arc<Self> {
        let apod = Arc::new(nasa_apod::NasaApodApi::new(config.nasa_api_key.clone()));
        Arc::new(Self { apod })
    }
}
