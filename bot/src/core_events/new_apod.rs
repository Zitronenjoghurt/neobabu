use crate::error::BotResult;
use crate::state::BotState;
use crate::ui::color::UiColor;
use crate::ui::embed::CreateEmbedExt;
use neobabu_core::database::entity::apod;
use poise::futures_util::StreamExt;
use poise::serenity_prelude::{ChannelId, Context, CreateEmbed, CreateEmbedAuthor, CreateMessage};
use std::ops::Add;

pub async fn handle(ctx: &Context, state: &BotState, apod: apod::Model) -> BotResult<()> {
    let embed = build_embed(apod);
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

        let _ = channel_id
            .send_message(
                ctx,
                CreateMessage::new()
                    .content(format!(
                        "**{role}NASA has just posted a new Astronomy Picture of the Day!**"
                    ))
                    .embed(embed.clone()),
            )
            .await;
    }
    Ok(())
}

fn build_embed(apod: apod::Model) -> CreateEmbed {
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
    let image_url = if let Some(thumbnail) = apod.thumbnail_url.clone() {
        Some(thumbnail)
    } else if let Some(hd_url) = apod.hd_url.clone() {
        Some(hd_url)
    } else {
        apod.url.clone()
    };

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

    if let Some(image_url) = image_url {
        embed = embed.image(image_url);
    }

    embed
}
