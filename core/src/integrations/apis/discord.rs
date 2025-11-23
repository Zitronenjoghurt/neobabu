use crate::error::CoreResult;
use crate::integrations::apis::discord::guild_iterator::GuildIterator;
use serenity::all::CurrentUser;
use std::sync::Arc;

mod guild_iterator;

pub struct DiscordClient {
    http: Arc<serenity::http::Http>,
}

impl DiscordClient {
    pub fn with_bot_token(token: impl AsRef<str>) -> Self {
        let http = serenity::http::HttpBuilder::new(token.as_ref()).build();
        Self {
            http: Arc::new(http),
        }
    }

    pub fn with_user_token(token: impl AsRef<str>) -> Self {
        let http = serenity::http::HttpBuilder::new(format!("Bearer {}", token.as_ref())).build();
        Self {
            http: Arc::new(http),
        }
    }

    pub async fn me(&self) -> CoreResult<CurrentUser> {
        Ok(self.http.get_current_user().await?)
    }

    pub fn guilds(&self) -> GuildIterator {
        GuildIterator::new(&self.http)
    }
}
