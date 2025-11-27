use crate::error::{CoreError, CoreResult};
use crate::integrations::client::IntegrationClient;
use crate::integrations::request::RequestBuilder;
use std::collections::HashMap;
use std::time::Duration;

pub struct YoutubeApi {
    client: IntegrationClient,
    token: Option<String>,
    hub_callback_url: Option<String>,
    pub hub_secret: Option<String>,
}

impl YoutubeApi {
    pub fn new(
        token: Option<String>,
        hub_callback_url: Option<String>,
        hub_secret: Option<String>,
    ) -> Self {
        Self {
            client: IntegrationClient::new(1000, 1, Duration::from_secs(9)),
            token,
            hub_callback_url,
            hub_secret,
        }
    }

    fn base_request(&'_ self, endpoint: impl AsRef<str>) -> CoreResult<RequestBuilder<'_>> {
        let Some(token) = &self.token else {
            return Err(CoreError::MissingYoutubeApiKey);
        };

        Ok(self
            .client
            .request(format!(
                "https://www.googleapis.com/youtube/v3/{}",
                endpoint.as_ref()
            ))?
            .query("key", token)
            .query("part", "snippet,statistics,contentDetails"))
    }

    pub async fn fetch_video(&self, video_id: impl AsRef<str>) -> CoreResult<Option<YoutubeVideo>> {
        let response: YoutubeApiResponse<YoutubeVideoItem> = self
            .base_request("videos")?
            .query("id", video_id)
            .get_json()
            .await?;
        Ok(response.items.into_iter().next().map(|item| item.into()))
    }

    pub async fn fetch_channel_by_id(
        &self,
        id: impl AsRef<str>,
    ) -> CoreResult<Option<YoutubeChannel>> {
        let response: YoutubeApiResponse<YoutubeChannelItem> = self
            .base_request("channels")?
            .query("id", id)
            .get_json()
            .await?;
        Ok(response.items.into_iter().next().map(|item| item.into()))
    }

    pub async fn fetch_channel_by_handle(
        &self,
        handle: impl AsRef<str>,
    ) -> CoreResult<Option<YoutubeChannel>> {
        let response: YoutubeApiResponse<YoutubeChannelItem> = self
            .base_request("channels")?
            .query("forHandle", handle)
            .get_json()
            .await?;
        Ok(response.items.into_iter().next().map(|item| item.into()))
    }

    pub async fn subscribe(&self, channel_id: impl AsRef<str>) -> CoreResult<()> {
        let Some(callback_url) = &self.hub_callback_url else {
            return Err(CoreError::YoutubeHubCallbackUrlMissing);
        };

        let Some(secret) = &self.hub_secret else {
            return Err(CoreError::YoutubeHubSecretMissing);
        };

        let topic = format!(
            "https://www.youtube.com/xml/feeds/videos.xml?channel_id={}",
            channel_id.as_ref()
        );

        let params = [
            ("hub.callback", callback_url.as_ref()),
            ("hub.mode", "subscribe"),
            ("hub.topic", topic.as_ref()),
            ("hub.lease_seconds", "864000"),
            ("hub.secret", secret.as_ref()),
        ];

        self.client
            .request("https://pubsubhubbub.appspot.com/subscribe")?
            .cost(0)
            .post_form(params)
            .await?;

        Ok(())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct YoutubeVideo {
    pub id: String,
    pub published_at: chrono::DateTime<chrono::Utc>,
    pub title: String,
    pub description: String,
    pub thumbnail_url: Option<String>,
    pub channel_id: String,
    pub channel_title: String,
    pub live_status: String,
    pub duration: Duration,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct YoutubeChannel {
    pub id: String,
    pub title: String,
    pub description: String,
    pub custom_url: Option<String>,
    pub icon_url: Option<String>,
    pub subscriber_count: Option<u64>,
    pub video_count: Option<u64>,
}

#[derive(serde::Deserialize)]
struct YoutubeApiResponse<T> {
    #[serde(default = "Vec::new")]
    items: Vec<T>,
}

#[derive(serde::Deserialize)]
struct YoutubeThumbnail {
    url: String,
    #[serde(default)]
    width: u32,
    #[serde(default)]
    height: u32,
}

#[derive(serde::Deserialize)]
struct YoutubeVideoItem {
    id: String,
    snippet: VideoSnippet,
    #[serde(alias = "contentDetails")]
    content_details: VideoContentDetails,
}

#[derive(serde::Deserialize)]
struct VideoSnippet {
    #[serde(alias = "publishedAt")]
    published_at: chrono::DateTime<chrono::Utc>,
    title: String,
    description: String,
    #[serde(alias = "channelId")]
    channel_id: String,
    #[serde(alias = "channelTitle")]
    channel_title: String,
    #[serde(default)]
    thumbnails: HashMap<String, YoutubeThumbnail>,
    #[serde(alias = "liveBroadcastContent")]
    live_broadcast_content: String,
}

#[derive(serde::Deserialize)]
struct VideoContentDetails {
    duration: String,
}

#[derive(serde::Deserialize)]
struct YoutubeChannelItem {
    id: String,
    snippet: ChannelSnippet,
    statistics: ChannelStatistics,
}

#[derive(serde::Deserialize)]
struct ChannelSnippet {
    title: String,
    description: String,
    #[serde(default, alias = "customUrl")]
    custom_url: Option<String>,
    #[serde(default)]
    thumbnails: HashMap<String, YoutubeThumbnail>,
}

#[derive(serde::Deserialize)]
struct ChannelStatistics {
    #[serde(default, alias = "subscriberCount")]
    subscriber_count: Option<String>,
    #[serde(default, alias = "videoCount")]
    video_count: Option<String>,
}

fn get_best_thumbnail(thumbnails: &HashMap<String, YoutubeThumbnail>) -> Option<String> {
    thumbnails
        .values()
        .max_by_key(|t| t.width * t.height)
        .map(|t| t.url.clone())
}

impl From<YoutubeVideoItem> for YoutubeVideo {
    fn from(item: YoutubeVideoItem) -> Self {
        Self {
            id: item.id,
            published_at: item.snippet.published_at,
            title: item.snippet.title,
            description: item.snippet.description,
            thumbnail_url: get_best_thumbnail(&item.snippet.thumbnails),
            channel_id: item.snippet.channel_id,
            channel_title: item.snippet.channel_title,
            live_status: item.snippet.live_broadcast_content,
            duration: iso8601::duration(&item.content_details.duration)
                .unwrap_or_default()
                .into(),
        }
    }
}

impl From<YoutubeChannelItem> for YoutubeChannel {
    fn from(item: YoutubeChannelItem) -> Self {
        Self {
            id: item.id,
            title: item.snippet.title,
            description: item.snippet.description,
            custom_url: item.snippet.custom_url,
            icon_url: get_best_thumbnail(&item.snippet.thumbnails),
            subscriber_count: item
                .statistics
                .subscriber_count
                .and_then(|s| s.parse().ok()),
            video_count: item.statistics.video_count.and_then(|s| s.parse().ok()),
        }
    }
}
