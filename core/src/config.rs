#[derive(Default)]
pub struct Config {
    pub db_url: String,
    pub nasa_api_key: Option<String>,
    pub youtube_api_key: Option<String>,
    pub youtube_hub_callback_url: Option<String>,
    pub youtube_hub_secret: Option<String>,
}
