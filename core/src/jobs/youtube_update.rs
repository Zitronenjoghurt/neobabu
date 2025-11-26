use crate::database::entity::{youtube_channel, youtube_video};
use crate::error::CoreResult;
use crate::events::new_youtube_video::NewYoutubeVideo;
use crate::events::CoreEvent;
use crate::NeobabuCore;
use futures::StreamExt;
use sea_orm::{IntoActiveModel, Set};
use tracing::{error, info, warn};

pub async fn run(core: NeobabuCore) -> CoreResult<()> {
    let mut pending_channels = core
        .stores
        .youtube_channel
        .stream_pending_resubscription()
        .await?;
    while let Some(channel) = pending_channels.next().await {
        let channel = channel?;
        let channel_name = channel.name.clone();
        let channel_id = channel.id.clone();
        if let Err(err) = trigger_resubscribe(&core, channel).await {
            error!(
                "An error occurred while resubscribing to channel '{channel_name}' ({channel_id}): {err}",
            );
        }
    }

    let mut pending_videos = core.stores.youtube_video.stream_unannounced().await?;
    while let Some(video) = pending_videos.next().await {
        let video = video?;
        let video_id = video.id.clone();
        if let Err(err) = process_video(&core, video).await {
            error!("An error occurred while processing video '{video_id}': {err}",);
        }
    }

    Ok(())
}

async fn trigger_resubscribe(
    core: &NeobabuCore,
    channel: youtube_channel::Model,
) -> CoreResult<()> {
    let mut active = channel.into_active_model();
    active.requested_resubscription = Set(true);
    let channel = core.stores.youtube_channel.update(active).await?;

    if let Err(err) = core.apis.youtube.subscribe(&channel.id).await {
        let mut active = channel.into_active_model();
        active.requested_resubscription = Set(false);
        let channel = core.stores.youtube_channel.update(active).await?;
        error!(
            "An error occurred while resubscribing to channel '{}' ({}): {}",
            channel.name, channel.id, err
        );
    } else {
        info!(
            "Triggered resubscription to channel '{}' ({})",
            channel.name, channel.id
        );
    }
    Ok(())
}

async fn process_video(core: &NeobabuCore, video: youtube_video::Model) -> CoreResult<()> {
    let Some(video_info) = core.apis.youtube.fetch_video(&video.id).await? else {
        warn!("Video '{}' not found, removing...", video.id);
        core.stores.youtube_video.delete(&video.id).await?;
        return Ok(());
    };

    let Some(channel) = core
        .services
        .youtube
        .find_channel_by_id(&video.channel_id)
        .await?
    else {
        warn!("Channel for video '{}' not found, removing...", video.id);
        core.stores.youtube_video.delete(&video.id).await?;
        return Ok(());
    };

    let Some(channel_model) = core
        .services
        .youtube
        .update_channel_if_needed(channel)
        .await?
    else {
        warn!("Channel for video '{}' not found, removing...", video.id);
        core.stores.youtube_video.delete(&video.id).await?;
        return Ok(());
    };

    let mut active = video.into_active_model();
    active.title = Set(Some(video_info.title));
    active.thumbnail_url = Set(video_info.thumbnail_url);
    active.notification_sent = Set(true);
    let video_model = core.stores.youtube_video.update(active).await?;

    let is_live = video_info.live_status == "live";
    let is_upcoming_live = video_info.live_status == "upcoming";

    info!(
        "Sending event for new youtube video '{:?}' ({}) from '{}' ({})",
        video_model.title, video_model.id, channel_model.name, channel_model.id
    );
    core.event_bus
        .send(CoreEvent::new_youtube_video(NewYoutubeVideo {
            channel_model,
            video_model,
            video_duration: video_info.duration,
            is_live,
            is_upcoming_live,
        }));

    Ok(())
}
