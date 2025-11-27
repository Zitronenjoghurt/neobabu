use crate::error::BotResult;
use crate::state::BotState;
use crate::ui::embed::CreateEmbedExt;
use neobabu_core::events::new_youtube_video::NewYoutubeVideo;
use poise::futures_util::StreamExt;
use poise::serenity_prelude::{ChannelId, Context, CreateEmbed, CreateEmbedAuthor, CreateMessage};

pub async fn handle(ctx: &Context, state: &BotState, event: NewYoutubeVideo) -> BotResult<()> {
    let mut author =
        CreateEmbedAuthor::new(&event.channel_model.name).url(event.channel_model.url());
    if let Some(icon_url) = &event.channel_model.icon_url {
        author = author.icon_url(icon_url);
    }

    let note = if event.is_live {
        "\n\n*Note: This video is a live stream*"
    } else if event.is_upcoming_live {
        "\n\n*Note: This video is scheduled to be a live stream*"
    } else {
        ""
    };

    let description = format!(
        "*Click [here]({}) to watch the video*{note}",
        event.video_model.url()
    );
    let mut embed = CreateEmbed::default()
        .description(description)
        .author(author)
        .url(event.video_model.url())
        .footer_text(format!(
            "Length: {}",
            humantime::format_duration(event.video_duration)
        ));
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

        let Some(guild_youtube) = state
            .core
            .stores
            .guild_youtube
            .find_by_guild_id(&guild_yt_channel.guild_id)
            .await?
        else {
            continue;
        };

        if !guild_youtube.enabled {
            continue;
        }

        let Some(channel_id_string) = guild_youtube.notification_channel_id else {
            continue;
        };
        let channel_id = ChannelId::new(channel_id_string.parse()?);

        let role = if let Some(role_id) = guild_youtube.notification_role_id {
            if role_id == guild_youtube.guild_id {
                "@everyone, ".to_string()
            } else {
                format!("<@&{role_id}>, ")
            }
        } else {
            "".to_string()
        };

        let message = CreateMessage::new()
            .content(format!(
                "{role}**`{}` just uploaded a new video!**",
                &event.channel_model.name
            ))
            .embed(embed.clone());
        let _ = channel_id.send_message(ctx, message).await?;
    }

    Ok(())
}
