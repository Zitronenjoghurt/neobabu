use crate::context::ContextExt;
use crate::error::{BotError, BotResult};
use crate::ui::embed::CreateEmbedExt;
use crate::ui::emoji::EmojiType;
use crate::Context;
use poise::serenity_prelude::{CreateEmbed, Member};

/// View your or another user's overall Rock Paper Scissors stats.
#[poise::command(slash_command, guild_only, user_cooldown = "30")]
pub async fn stats(ctx: Context<'_>, member: Option<Member>) -> BotResult<()> {
    ctx.defer().await?;

    let author_member = ctx.author_member().await;
    let target = match &member {
        Some(m) => m,
        None => author_member.as_deref().ok_or(BotError::GuildCommandOnly)?,
    };

    let user = ctx
        .stores()
        .user
        .fetch_or_create(target.user.id.to_string())
        .await?;

    let stats = ctx.data().core.services.rps.get_stats(&user).await?;

    let embed = CreateEmbed::new()
        .member_full(&ctx, target)
        .await
        .title("Rock Paper Scissors Stats")
        .field(
            "Wins",
            format!("{} **`{}`**", ctx.emoji_text(EmojiType::Trophy), stats.wins),
            true,
        )
        .field(
            "Losses",
            format!(
                "{} **`{}`**",
                ctx.emoji_text(EmojiType::FaceConfusedSad),
                stats.losses
            ),
            true,
        )
        .field(
            "Draws",
            format!("{} **`{}`**", ctx.emoji_text(EmojiType::Pvp), stats.draws),
            true,
        )
        .field(
            "Rock",
            format!("{} **`{}`**", ctx.emoji_text(EmojiType::Rock), stats.rock),
            true,
        )
        .field(
            "Paper",
            format!("{} **`{}`**", ctx.emoji_text(EmojiType::Paper), stats.paper),
            true,
        )
        .field(
            "Scissors",
            format!(
                "{} **`{}`**",
                ctx.emoji_text(EmojiType::Scissors),
                stats.scissors
            ),
            true,
        )
        .field(
            "Total Played",
            format!(
                "{} **`{}`**",
                ctx.emoji_text(EmojiType::Friends),
                stats.total_played()
            ),
            true,
        )
        .field(
            "Win Rate",
            format!(
                "{} **`{:.2}%`**",
                ctx.emoji_text(EmojiType::Stats),
                stats.win_rate() * 100.0
            ),
            true,
        );

    ctx.send(embed.create_reply()).await?;
    Ok(())
}
