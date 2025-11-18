use crate::state::BotState;
use crate::ui::embed::CreateEmbedExt;
use crate::ui::time::format_time_relative_in;
use crate::Context;
use nanoid::nanoid;
use poise::serenity_prelude::{CreateEmbed, Permissions};
use poise::FrameworkError;
use tracing::error;

pub type BotResult<T> = Result<T, BotError>;

#[derive(Debug, thiserror::Error)]
pub enum BotError {
    #[error("{0}")]
    Core(#[from] neobabu_core::error::CoreError),
    #[error("This command can only be used in a guild.")]
    GuildCommandOnly,
    #[error("Parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Serenity error: {0}")]
    Serenity(#[from] poise::serenity_prelude::Error),
}

impl BotError {
    pub fn is_user_error(&self) -> bool {
        match self {
            Self::Core(error) => error.is_user_error(),
            Self::GuildCommandOnly => true,
            Self::ParseInt(_) | Self::Serenity(_) => false,
        }
    }
}

pub async fn handler(framework_error: FrameworkError<'_, BotState, BotError>) {
    match framework_error {
        FrameworkError::CooldownHit {
            remaining_cooldown,
            ctx,
            ..
        } => handle_cooldown_hit(remaining_cooldown, &ctx).await,
        FrameworkError::Command { error, ctx, .. } => {
            let embed = handle_command_error(error, &ctx).await;
            let _ = ctx.send(embed.create_reply().ephemeral(true)).await;
        }
        FrameworkError::CommandPanic { payload, ctx, .. } => {
            handle_command_panic(payload, &ctx).await
        }
        FrameworkError::CommandCheckFailed { error, ctx, .. } => {
            handle_command_check_failure(error, &ctx).await
        }
        FrameworkError::DmOnly { ctx, .. } => handle_dm_only(&ctx).await,
        FrameworkError::EventHandler { error, event, .. } => {
            error!("An error occurred in event handler for event '{event:#?}': {error}")
        }
        FrameworkError::GuildOnly { ctx, .. } => handle_guild_only(&ctx).await,
        FrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
            ..
        } => handle_missing_bot_permissions(missing_permissions, &ctx).await,
        FrameworkError::MissingUserPermissions {
            missing_permissions,
            ctx,
            ..
        } => handle_missing_user_permissions(missing_permissions, &ctx).await,
        FrameworkError::NotAnOwner { ctx, .. } => handle_not_an_owner(&ctx).await,
        FrameworkError::NsfwOnly { ctx, .. } => handle_nsfw_only(&ctx).await,
        FrameworkError::Setup {
            error,
            data_about_bot,
            ..
        } => {
            error!("An error occurred during bot setup: {error} | {data_about_bot:?}")
        }
        _ => error!("An unhandled framework error occurred: {framework_error}"),
    }
}

async fn handle_cooldown_hit(remaining_cooldown: std::time::Duration, ctx: &Context<'_>) {
    let embed = CreateEmbed::new()
        .error()
        .title("Cooldown")
        .description(format!(
            "You can use this command again {}.",
            format_time_relative_in(remaining_cooldown)
        ));
    let _ = ctx.send(embed.create_reply().ephemeral(true)).await;
}

pub async fn handle_command_error(error: BotError, ctx: &Context<'_>) -> CreateEmbed {
    let text = if error.is_user_error() {
        error.to_string()
    } else {
        let id = nanoid!(12);
        error!(
            "[#{id}] An error occurred in command '{}' executed by '{}': {}",
            ctx.command().name,
            ctx.author().id,
            error.to_string()
        );
        format!("An unexpected error occurred. If you report this, please include the ID `#{id}`.")
    };

    CreateEmbed::new().error().title("ERROR").description(text)
}

async fn handle_command_panic(payload: Option<String>, ctx: &Context<'_>) {
    let id = nanoid!(12);
    error!(
        payload = payload,
        "CRITICAL [#{id}] A panic occurred in command '{}' executed by '{}'",
        ctx.command().name,
        ctx.author().id,
    );

    let error_text =
        format!("An unexpected error occurred. If you report this, please include the ID `#{id}`.");
    let embed = CreateEmbed::new()
        .error()
        .title("ERROR")
        .description(error_text);
    let _ = ctx.send(embed.create_reply().ephemeral(true)).await;
}

async fn handle_command_check_failure(error: Option<BotError>, ctx: &Context<'_>) {
    let embed = if let Some(error) = error {
        CreateEmbed::new()
            .error()
            .title("You cannot execute this command")
            .description(error.to_string())
    } else {
        CreateEmbed::new()
            .error()
            .title("Command failed")
            .description("You cannot execute this command.")
    };
    let _ = ctx.send(embed.create_reply().ephemeral(true)).await;
}

async fn handle_dm_only(ctx: &Context<'_>) {
    let embed = CreateEmbed::new()
        .error()
        .title("DM Only Command")
        .description("This command can only be used in direct messages with the bot.");
    let _ = ctx.send(embed.create_reply().ephemeral(true)).await;
}

async fn handle_guild_only(ctx: &Context<'_>) {
    let embed = CreateEmbed::new()
        .error()
        .title("Guild Only Command")
        .description("This command can only be used in a guild.");
    let _ = ctx.send(embed.create_reply().ephemeral(true)).await;
}

async fn handle_missing_bot_permissions(permissions: Permissions, ctx: &Context<'_>) {
    let embed = CreateEmbed::new()
        .error()
        .title("Missing Permissions")
        .description(format!(
            "I am missing the following permissions in this server to execute your action: `{permissions}`"
        ));
    let _ = ctx.send(embed.create_reply().ephemeral(true)).await;
}

async fn handle_missing_user_permissions(permissions: Option<Permissions>, ctx: &Context<'_>) {
    let mut embed = CreateEmbed::new().error().title("Missing Permissions");

    if let Some(permissions) = permissions {
        embed = embed.description(format!(
            "You are missing the following permissions to execute this action: `{permissions}`"
        ));
    } else {
        embed = embed.description("You are missing required permissions to execute this action.");
    }

    let _ = ctx.send(embed.create_reply().ephemeral(true)).await;
}

async fn handle_not_an_owner(ctx: &Context<'_>) {
    let embed = CreateEmbed::new()
        .error()
        .title("Not an Owner")
        .description("You are not an owner of this bot.");
    let _ = ctx.send(embed.create_reply().ephemeral(true)).await;
}

async fn handle_nsfw_only(ctx: &Context<'_>) {
    let embed = CreateEmbed::new()
        .error()
        .title("NSFW Only Command")
        .description("This command can only be used in NSFW channels.");
    let _ = ctx.send(embed.create_reply().ephemeral(true)).await;
}
