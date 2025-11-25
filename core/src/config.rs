#[derive(Default)]
pub struct Config {
    pub db_url: String,
    pub nasa_api_key: Option<String>,
    pub youtube_api_key: Option<String>,
}
