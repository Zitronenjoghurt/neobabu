use crate::error::BotResult;
use crate::state::BotState;
use crate::ui::color::UiColor;
use crate::ui::embed::CreateEmbedExt;
use neobabu_core::database::entity::apod;
use poise::futures_util::StreamExt;
use poise::serenity_prelude::{
    ChannelId, Context, CreateAttachment, CreateEmbed, CreateEmbedAuthor, CreateMessage,
};
use std::ops::Add;
use tracing::{info, warn};

pub async fn handle(ctx: &Context, state: &BotState, apod: apod::Model) -> BotResult<()> {
    let attachment = download_image(&apod).await?;
    let image_url = apod.image_url();

    let mut embed = build_embed(apod, &attachment);
    if attachment.is_none()
        && let Some(image_url) = image_url
    {
        embed = embed.image(image_url);
    };

    let mut guild_apods = state.core.stores.guild_apod.stream_all_enabled().await?;
    while let Some(guild_apod) = guild_apods.next().await {
        let guild_apod = guild_apod?;
        let Some(notification_channel) = guild_apod.notification_channel_id else {
            continue;
        };
        let channel_id = ChannelId::new(notification_channel.parse()?);

        let role = if let Some(role_id) = guild_apod.notification_role_id {
            if role_id == guild_apod.guild_id {
                "@everyone, ".to_string()
            } else {
                format!("<@&{role_id}>, ")
            }
        } else {
            "".to_string()
        };

        let mut message = CreateMessage::new()
            .content(format!(
                "**{role}NASA has just posted a new Astronomy Picture of the Day!**"
            ))
            .embed(embed.clone());

        if let Some(attachment) = &attachment {
            message = message.add_file(attachment.clone());
        }

        let _ = channel_id.send_message(ctx, message).await;
    }
    Ok(())
}

async fn download_image(apod: &apod::Model) -> BotResult<Option<CreateAttachment>> {
    if let Some(media_type) = apod.media_type.as_ref() {
        if media_type != "image" {
            return Ok(None);
        }
    } else {
        return Ok(None);
    }

    let Some(image_url) = apod.image_url() else {
        return Ok(None);
    };

    info!("Downloading APOD image from: {}", image_url);

    let response = reqwest::get(&image_url).await?;

    if let Err(err) = response.error_for_status_ref() {
        warn!(error = %err, "Failed to download image from: {}", image_url);
        return Ok(None);
    }

    let bytes = response.bytes().await?;

    if bytes.is_empty() {
        warn!("Downloaded empty image from: {}", image_url);
        return Ok(None);
    }

    if bytes.len() > 8_388_608 {
        warn!(
            "Downloaded image from: {} is too large ({} bytes) to upload",
            image_url,
            bytes.len()
        );
        return Ok(None);
    }

    let filename = image_url
        .rsplit('/')
        .next()
        .and_then(|s| s.split('?').next())
        .unwrap_or("apod.png")
        .to_string();
    info!(filename = %filename, "Downloaded APOD image");

    Ok(Some(CreateAttachment::bytes(bytes.to_vec(), filename)))
}

fn build_embed(apod: apod::Model, attachment: &Option<CreateAttachment>) -> CreateEmbed {
    let site_url = apod.site_url();

    let title = apod
        .title
        .clone()
        .unwrap_or("New Astronomy Picture Of The Day".to_string());
    let explanation = apod
        .explanation
        .clone()
        .unwrap_or("No explanation available.".to_string())
        .add(&format!(
            "\n\n[View this entry on the official APOD website]({site_url})"
        ));

    let copyright = if let Some(copyright) = apod.copyright.clone() {
        format!(" | Copyright: {}", copyright.trim_matches('\n'))
    } else {
        "".to_string()
    };
    let footer = format!("{}{copyright}", apod.date_string());

    let mut embed = CreateEmbed::default()
        .ui_color(UiColor::Nasa)
        .title(title)
        .url(site_url)
        .description(explanation)
        .author(
            CreateEmbedAuthor::new("NASA (Astronomy Picture of the Day)")
                .icon_url("https://api.nasa.gov/assets/img/favicons/favicon-192.png")
                .url("https://apod.nasa.gov/apod/astropix.html"),
        )
        .footer_text(footer);

    if let Some(attachment) = attachment {
        embed = embed.image(format!("attachment://{}", attachment.filename));
    }

    embed
}
