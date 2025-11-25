use crate::error::ServerResult;

pub struct ServerConfig {
    pub client_id: String,
    pub client_secret: String,
    pub db_url: String,
    pub discord_oauth_url: String,
    pub discord_oauth_token_url: String,
    pub oauth_redirect_uri: String,
    pub session_secret: String,
    pub token_secret: String,
    pub vite_dev_server: Option<String>,
}

impl ServerConfig {
    pub fn load_from_env() -> ServerResult<Self> {
        let client_id = std::env::var("CLIENT_ID")?;
        let client_secret = std::env::var("CLIENT_SECRET")?;
        let token_secret = std::env::var("TOKEN_SECRET")?;
        let oauth_redirect_uri = std::env::var("OAUTH_REDIRECT_URI")?;
        let discord_oauth_url = std::env::var("DISCORD_OAUTH_URL")?;
        let discord_oauth_token_url = std::env::var("DISCORD_OAUTH_TOKEN_URL")?;
        let db_url = std::env::var("DATABASE_URL")?;
        let session_secret = std::env::var("SESSION_SECRET")?;
        let vite_dev_server = std::env::var("VITE_DEV_SERVER").ok();
        Ok(Self {
            client_id,
            client_secret,
            token_secret,
            oauth_redirect_uri,
            discord_oauth_url,
            discord_oauth_token_url,
            db_url,
            session_secret,
            vite_dev_server,
        })
    }

    pub fn is_dev_mode(&self) -> bool {
        self.vite_dev_server.is_some()
    }
}
