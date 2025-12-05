use crate::config::ServerConfig;
use crate::error::ServerResult;
use neobabu_core::config::Config;
use neobabu_core::cryptor::Cryptor;
use neobabu_core::NeobabuCore;
use oauth2::{basic::*, *};
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
#[allow(clippy::type_complexity)]
pub struct ServerState {
    pub config: Arc<ServerConfig>,
    pub core: Arc<NeobabuCore>,
    pub oauth_client: Arc<
        Client<
            BasicErrorResponse,
            BasicTokenResponse,
            BasicTokenIntrospectionResponse,
            StandardRevocableToken,
            BasicRevocationErrorResponse,
            EndpointSet,
            EndpointNotSet,
            EndpointNotSet,
            EndpointNotSet,
            EndpointSet,
        >,
    >,
    pub oauth_cryptor: Arc<Cryptor>,
}

impl ServerState {
    pub async fn initialize() -> ServerResult<Self> {
        info!("Initializing server state...");

        info!("Loading server config...");
        let config = ServerConfig::load_from_env()?;

        info!("Loading core config...");
        let core_config = load_core_config(&config)?;
        info!("Core config loaded");

        info!("Initializing core...");
        let core = NeobabuCore::initialize(core_config).await?;
        info!("Core initialized");

        info!("Initializing oauth client...");
        let oauth_client = BasicClient::new(ClientId::new(config.client_id.clone()))
            .set_client_secret(ClientSecret::new(config.client_secret.clone()))
            .set_auth_uri(AuthUrl::new(config.discord_oauth_url.clone())?)
            .set_token_uri(TokenUrl::new(config.discord_oauth_token_url.clone())?)
            .set_redirect_uri(RedirectUrl::new(config.oauth_redirect_uri.clone())?);
        info!("Oauth client initialized");

        info!("Initializing oauth cryptor");
        let oauth_cryptor = Cryptor::new(&config.token_secret)?;
        info!("Oauth cryptor initialized");

        info!("Server state initialized");

        Ok(Self {
            config: Arc::new(config),
            core: Arc::new(core),
            oauth_client: Arc::new(oauth_client),
            oauth_cryptor,
        })
    }
}

fn load_core_config(config: &ServerConfig) -> ServerResult<Config> {
    let youtube_api_key = std::env::var("YOUTUBE_API_KEY")?;
    let youtube_hub_callback_url = std::env::var("YOUTUBE_HUB_CALLBACK_URL")?;
    let youtube_hub_secret = std::env::var("YOUTUBE_HUB_SECRET")?;
    Ok(Config {
        db_url: config.db_url.clone(),
        youtube_api_key: Some(youtube_api_key),
        youtube_hub_callback_url: Some(youtube_hub_callback_url),
        youtube_hub_secret: Some(youtube_hub_secret),
        ..Default::default()
    })
}
