use crate::context::ContextExt;
use crate::error::BotResult;
use crate::ui::color::UiColor;
use crate::ui::embed::CreateEmbedExt;
use crate::utils::formatting::format_bool;
use crate::Context;
use neobabu_core::stores::{IntoActiveModel, Set};
use poise::serenity_prelude::{Channel, CreateEmbed, Role};

#[poise::command(slash_command, guild_only, required_permissions = "ADMINISTRATOR")]
pub async fn admin(
    ctx: Context<'_>,
    #[description = "Whether to enable or disable apod notifications."] enable: Option<bool>,
    #[description = "The channel to send apod notifications to."]
    #[channel_types("Text")]
    channel: Option<Channel>,
    #[description = "The role to mention in apod notifications."] role: Option<Role>,
) -> BotResult<()> {
    ctx.defer_ephemeral().await?;

    let guild = ctx.fetch_guild_model().await?;
    let guild_apod = ctx.stores().guild_apod.fetch_or_create(&guild).await?;

    let mut updated = false;
    let mut active = guild_apod.into_active_model();
    if let Some(enable) = enable {
        active.enabled = Set(enable);
        updated = true;
    }

    if let Some(channel) = channel {
        active.notification_channel_id = Set(Some(channel.id().to_string()));
        updated = true;
    }

    if let Some(role) = role {
        active.notification_role_id = Set(Some(role.id.to_string()));
        updated = true;
    }

    let embed = if updated {
        let guild_apod = ctx.stores().guild_apod.update(active).await?;

        let channel = if let Some(channel_id) = guild_apod.notification_channel_id {
            format!("<#{}>", channel_id)
        } else {
            "`None`".to_string()
        };

        let role = if let Some(role_id) = guild_apod.notification_role_id {
            if role_id == guild_apod.guild_id {
                "@everyone".to_string()
            } else {
                format!("<@&{}>", role_id)
            }
        } else {
            "`None`".to_string()
        };

        CreateEmbed::default()
            .success_user(&ctx.author())
            .title("APOD Settings Updated")
            .field(
                "Enabled",
                format!("`{}`", format_bool(guild_apod.enabled)),
                false,
            )
            .field("Channel", channel, false)
            .field("Role", role, false)
    } else {
        let channel = if let Some(channel_id) = active.notification_channel_id.as_ref() {
            format!("<#{}>", channel_id)
        } else {
            "`None`".to_string()
        };

        let role = if let Some(role_id) = active.notification_role_id.as_ref() {
            if role_id == active.guild_id.as_ref() {
                "@everyone".to_string()
            } else {
                format!("<@&{}>", role_id)
            }
        } else {
            "`None`".to_string()
        };

        CreateEmbed::default()
            .ui_color(UiColor::Gray)
            .user(&ctx.author())
            .title("APOD Settings")
            .field(
                "Enabled",
                format!("`{}`", format_bool(*active.enabled.as_ref())),
                false,
            )
            .field("Channel", channel, false)
            .field("Role", role, false)
    };

    ctx.send(embed.create_reply().ephemeral(true)).await?;

    Ok(())
}
