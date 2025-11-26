use crate::state::ServerState;
use axum::body::Bytes;
use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use hmac::{Hmac, Mac};
use neobabu_core::database::entity::youtube_video;
use neobabu_core::stores::{IntoActiveModel, Set};
use reqwest::StatusCode;
use serde::Deserialize;
use sha1::Sha1;
use std::ops::Add;
use std::time::Duration;
use tracing::{error, info};

#[derive(serde::Deserialize)]
struct WebhookQuery {
    #[serde(default, alias = "hub.challenge")]
    hub_challenge: Option<String>,
    #[serde(default, alias = "hub.topic")]
    hub_topic: Option<String>,
    #[serde(default, alias = "hub.lease_seconds")]
    hub_lease_seconds: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "feed")]
struct PushFeed {
    pub entry: FeedEntry,
}

#[derive(Debug, Deserialize)]
struct FeedEntry {
    #[serde(alias = "videoId", alias = "yt:videoId")]
    pub video_id: String,
    #[serde(alias = "channelId", alias = "yt:channelId")]
    pub channel_id: String,
}

async fn get_webhook(
    State(state): State<ServerState>,
    Query(query): Query<WebhookQuery>,
) -> impl IntoResponse {
    if let Some(challenge) = query.hub_challenge
        && let Some(topic) = query.hub_topic
        && let Some(lease_seconds) = query.hub_lease_seconds
    {
        let Some(channel_id) = topic
            .split("channel_id=")
            .nth(1)
            .and_then(|tail| tail.split('&').next())
        else {
            return StatusCode::BAD_REQUEST.into_response();
        };

        let channel = match state
            .core
            .stores
            .youtube_channel
            .find_by_id(channel_id)
            .await
        {
            Ok(Some(channel)) => channel,
            _ => return StatusCode::BAD_REQUEST.into_response(),
        };

        if !channel.requested_resubscription {
            return StatusCode::BAD_REQUEST.into_response();
        }

        let next_resubscribe = chrono::Utc::now()
            .naive_utc()
            .add(Duration::from_secs((lease_seconds as f64 * 0.95) as u64));

        let mut active = channel.into_active_model();
        active.requested_resubscription = Set(false);
        active.next_resubscription_at = Set(next_resubscribe);
        let Ok(channel) = state.core.stores.youtube_channel.update(active).await else {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        };

        info!(
            "Successfully resubscribed to channel '{}' ({}), echoing back challenge...",
            channel.name, channel.id
        );
        return challenge.into_response();
    }

    StatusCode::BAD_REQUEST.into_response()
}

async fn post_webhook(
    State(state): State<ServerState>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    let Some(signature) = headers.get("x-hub-signature").and_then(|v| v.to_str().ok()) else {
        return StatusCode::FORBIDDEN;
    };

    if !validate_signature(&state, signature, &body) {
        return StatusCode::FORBIDDEN;
    };

    let feed: PushFeed = match serde_xml_rs::from_reader(body.as_ref()) {
        Ok(feed) => feed,
        Err(e) => {
            error!("Failed to parse YouTube hub notification: {}", e);
            return StatusCode::BAD_REQUEST;
        }
    };

    match state
        .core
        .stores
        .youtube_video
        .find_by_id(&feed.entry.video_id)
        .await
    {
        Ok(None) => {}
        Ok(Some(_)) => return StatusCode::OK,
        Err(e) => {
            error!(
                "Failed to find YouTube video '{}' for hub notification: {}",
                feed.entry.video_id, e
            );
            return StatusCode::OK;
        }
    }

    let new = youtube_video::ActiveModel {
        id: Set(feed.entry.video_id.clone()),
        channel_id: Set(feed.entry.channel_id),
        ..Default::default()
    };
    if let Err(err) = state.core.stores.youtube_video.insert(new).await {
        error!(
            "Failed to insert YouTube video '{}' for hub notification: {}",
            feed.entry.video_id, err
        );
    } else {
        info!(
            "Inserted YouTube video '{}' for hub notification",
            feed.entry.video_id
        );
    }

    StatusCode::OK
}

fn validate_signature(state: &ServerState, signature: &str, body: &[u8]) -> bool {
    let Some(secret) = &state.core.apis.youtube.hub_secret else {
        error!("YouTube Hub secret not configured, skipping signature validation");
        return false;
    };

    let expected = signature.trim_start_matches("sha1=");
    let Ok(mut mac) = Hmac::<Sha1>::new_from_slice(secret.as_bytes()) else {
        error!("Failed to create HMAC for YouTube Hub signature validation: invalid length");
        return false;
    };
    mac.update(body);
    let calculated = hex::encode(mac.finalize().into_bytes());

    calculated == expected
}

pub fn router() -> Router<ServerState> {
    Router::<ServerState>::new().route("/", get(get_webhook).post(post_webhook))
}
