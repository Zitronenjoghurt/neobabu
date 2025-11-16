use crate::state::BotState;
use crate::ui::embed::CreateEmbedExt;
use crate::ui::time::format_time_relative_in;
use crate::Context;
use nanoid::nanoid;
use poise::serenity_prelude::CreateEmbed;
use poise::FrameworkError;
use tracing::error;

pub type BotResult<T> = Result<T, BotError>;

#[derive(Debug, thiserror::Error)]
pub enum BotError {
    #[error("{0}")]
    Core(#[from] neobabu_core::error::CoreError),
    #[error("This command can only be used in a guild.")]
    GuildCommandOnly,
    #[error("Serenity error: {0}")]
    Serenity(#[from] poise::serenity_prelude::Error),
}

impl BotError {
    pub fn is_user_error(&self) -> bool {
        match self {
            Self::Core(error) => error.is_user_error(),
            Self::GuildCommandOnly => true,
            Self::Serenity(_) => false,
        }
    }
}

pub async fn handler(framework_error: FrameworkError<'_, BotState, BotError>) {
    match framework_error {
        FrameworkError::CooldownHit {
            remaining_cooldown,
            ctx,
            ..
        } => handle_cooldown_hit(remaining_cooldown, ctx).await,
        FrameworkError::Command { error, ctx, .. } => {
            handle_command(error, ctx).await;
        }
        _ => error!("An unhandled error occurred: {framework_error}"),
    }
}

async fn handle_cooldown_hit(remaining_cooldown: std::time::Duration, ctx: Context<'_>) {
    let embed = CreateEmbed::new()
        .error()
        .title("Cooldown")
        .description(format!(
            "You can use this command again {}.",
            format_time_relative_in(remaining_cooldown)
        ));
    let _ = ctx.send(embed.create_reply().ephemeral(true)).await;
}

async fn handle_command(error: BotError, ctx: Context<'_>) {
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

    let embed = CreateEmbed::new().error().title("ERROR").description(text);
    let _ = ctx.send(embed.create_reply().ephemeral(true)).await;
}
