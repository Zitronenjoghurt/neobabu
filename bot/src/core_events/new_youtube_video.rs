use crate::error::BotResult;
use crate::state::BotState;
use neobabu_core::events::new_youtube_video::NewYoutubeVideo;
use poise::futures_util::StreamExt;
use poise::serenity_prelude::{Context, CreateEmbed, CreateEmbedAuthor};

pub async fn handle(ctx: &Context, state: &BotState, event: NewYoutubeVideo) -> BotResult<()> {
    let mut author =
        CreateEmbedAuthor::new(&event.channel_model.name).url(event.channel_model.url());
    if let Some(icon_url) = &event.channel_model.icon_url {
        author = author.icon_url(icon_url);
    }

    let description = format!(
        "*Click [here]({}) to watch the video*",
        event.video_model.url()
    );
    let mut embed = CreateEmbed::default()
        .description(description)
        .author(author)
        .url(event.video_model.url());
    if let Some(title) = &event.video_model.title {
        embed = embed.title(title);
    }
    if let Some(thumbnail_url) = &event.video_model.thumbnail_url {
        embed = embed.thumbnail(thumbnail_url);
    }

    let mut guild_yt_channels = state
        .core
        .stores
        .guild_youtube_channel
        .stream_by_channel_id(&event.channel_model.id)
        .await?;
    while let Some(guild_yt_channel) = guild_yt_channels.next().await {
        let guild_yt_channel = guild_yt_channel?;
    }

    Ok(())
}
