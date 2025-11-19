use crate::context::ContextExt;
use crate::error::BotResult;
use crate::ui::color::UiColor;
use crate::ui::embed::CreateEmbedExt;
use crate::utils::formatting::format_bool;
use crate::Context;
use neobabu_core::stores::{IntoActiveModel, Set};
use poise::serenity_prelude::{Channel, CreateEmbed};

/// Customize server-wide birthday settings.
#[poise::command(slash_command, guild_only, required_permissions = "ADMINISTRATOR")]
pub async fn admin(
    ctx: Context<'_>,
    #[description = "Whether to enable or disable birthday notifications."] enable: Option<bool>,
    #[description = "The channel to send birthday notifications to."]
    #[channel_types("Text")]
    channel: Option<Channel>,
) -> BotResult<()> {
    ctx.defer_ephemeral().await?;

    let guild = ctx.fetch_guild_model().await?;
    let guild_birthday = ctx.stores().guild_birthday.fetch_or_create(&guild).await?;

    let mut updated = false;
    let mut active = guild_birthday.into_active_model();
    if let Some(enable) = enable {
        active.enabled = Set(enable);
        updated = true;
    }

    if let Some(channel) = channel {
        active.notification_channel_id = Set(Some(channel.id().to_string()));
        updated = true;
    }

    let embed = if updated {
        let guild_birthday = ctx.stores().guild_birthday.update(active).await?;

        let channel = if let Some(channel_id) = guild_birthday.notification_channel_id {
            format!("<#{}>", channel_id)
        } else {
            "`None`".to_string()
        };

        CreateEmbed::default()
            .success_user(&ctx.author())
            .title("Birthday Settings Updated")
            .field(
                "Enabled",
                format!("`{}`", format_bool(guild_birthday.enabled)),
                false,
            )
            .field("Channel", channel, false)
    } else {
        let channel = if let Some(channel_id) = active.notification_channel_id.as_ref() {
            format!("<#{}>", channel_id)
        } else {
            "`None`".to_string()
        };

        CreateEmbed::default()
            .ui_color(UiColor::Gray)
            .user(&ctx.author())
            .title("Birthday Settings")
            .field(
                "Enabled",
                format!("`{}`", format_bool(*active.enabled.as_ref())),
                false,
            )
            .field("Channel", channel, false)
    };

    ctx.send(embed.create_reply().ephemeral(true)).await?;

    Ok(())
}
