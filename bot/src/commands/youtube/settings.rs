use crate::context::ContextExt;
use crate::error::BotResult;
use crate::ui::color::UiColor;
use crate::ui::embed::CreateEmbedExt;
use crate::utils::formatting::format_bool;
use crate::Context;
use neobabu_core::stores::{IntoActiveModel, Set};
use poise::serenity_prelude::{Channel, CreateEmbed, Role};

/// Customize server-wide youtube notification settings.
#[poise::command(slash_command, guild_only, required_permissions = "MANAGE_GUILD")]
pub async fn settings(
    ctx: Context<'_>,
    #[description = "Whether to enable or disable youtube notifications."] enable: Option<bool>,
    #[description = "The channel to send youtube notifications to."]
    #[channel_types("Text")]
    channel: Option<Channel>,
    #[description = "The role to mention in youtube notifications."] role: Option<Role>,
) -> BotResult<()> {
    ctx.defer_ephemeral().await?;

    let guild = ctx.fetch_guild_model().await?;
    let guild_youtube = ctx.stores().guild_youtube.fetch_or_create(&guild).await?;
    let guild_id = guild_youtube.guild_id.clone();

    let mut active = guild_youtube.into_active_model();
    let updated = enable.is_some() || channel.is_some() || role.is_some();

    if let Some(enable) = enable {
        active.enabled = Set(enable);
    }
    if let Some(channel) = channel {
        active.notification_channel_id = Set(Some(channel.id().to_string()));
    }
    if let Some(role) = role {
        active.notification_role_id = Set(Some(role.id.to_string()));
    }

    let (enabled, channel_id, role_id) = if updated {
        let model = ctx.stores().guild_youtube.update(active).await?;
        (
            model.enabled,
            model.notification_channel_id,
            model.notification_role_id,
        )
    } else {
        (
            *active.enabled.as_ref(),
            active.notification_channel_id.as_ref().clone(),
            active.notification_role_id.as_ref().clone(),
        )
    };

    let format_channel = |id: Option<String>| id.map_or("`None`".into(), |id| format!("<#{id}>"));
    let format_role = |id: Option<String>| match id {
        Some(id) if id == guild_id => "@everyone".into(),
        Some(id) => format!("<@&{id}>"),
        None => "`None`".into(),
    };

    let embed = CreateEmbed::default()
        .title(if updated {
            "Youtube Settings Updated"
        } else {
            "Youtube Settings"
        })
        .field("Enabled", format!("`{}`", format_bool(enabled)), false)
        .field("Channel", format_channel(channel_id), false)
        .field("Role", format_role(role_id), false);

    let embed = if updated {
        embed.success_user(ctx.author())
    } else {
        embed.ui_color(UiColor::Gray).user(ctx.author())
    };

    ctx.send(embed.create_reply().ephemeral(true)).await?;
    Ok(())
}
