use crate::context::ContextExt;
use crate::error::{BotError, BotResult};
use crate::ui::emoji::EmojiType;
use crate::ui::message::CreateEmbedExt;
use crate::Context;
use poise::serenity_prelude::{CreateEmbed, Member};

/// View your or another user's Blackjack stats.
#[poise::command(slash_command, guild_only)]
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
    let bj_user = ctx.stores().bj_user.fetch_or_create(&user).await?;

    let streak = if bj_user.win_streak > 0 {
        format!("**`{} WINS`**", bj_user.win_streak)
    } else if bj_user.loss_streak > 0 {
        format!("**`{} LOSSES`**", bj_user.loss_streak)
    } else if bj_user.draw_streak > 0 {
        format!("**`{} PUSHES`**", bj_user.draw_streak)
    } else {
        "None".to_string()
    };

    let embed = CreateEmbed::default()
        .member_full(&ctx, target)
        .await
        .title("Blackjack Stats")
        .field(
            "Games Played",
            format!("**`{}`**", bj_user.games_played()),
            true,
        )
        .field(
            "Blackjacks",
            format!("**`{}`**", bj_user.blackjack_count),
            true,
        )
        .field("Streak", streak, true)
        .field(
            "Times Busted",
            format!("**`{}`**", bj_user.times_final_hit),
            true,
        )
        .field(
            "avg. Stand",
            format!("**`{:.2}`**", bj_user.avg_stand()),
            true,
        )
        .field(
            "avg. Dealer",
            format!("**`{:.2}`**", bj_user.avg_dealer()),
            true,
        )
        .field("Wins", format!("**`{}`**", bj_user.wins), true)
        .field("Losses", format!("**`{}`**", bj_user.losses), true)
        .field("Pushes", format!("**`{}`**", bj_user.draws), true)
        .field(
            "max. Win Streak",
            format!("**`{}`**", bj_user.longest_win_streak),
            true,
        )
        .field(
            "max. Loss Streak",
            format!("**`{}`**", bj_user.longest_loss_streak),
            true,
        )
        .field(
            "max. Push Streak",
            format!("**`{}`**", bj_user.longest_draw_streak),
            true,
        )
        .field(
            "Citrine Wagered",
            format!(
                "**`{}`** {}",
                bj_user.total_citrine_wagered,
                ctx.emoji_text(EmojiType::Citrine)
            ),
            true,
        )
        .field(
            "Citrine Won",
            format!(
                "**`{}`** {}",
                bj_user.total_citrine_won,
                ctx.emoji_text(EmojiType::Citrine)
            ),
            true,
        )
        .field(
            "Citrine Lost",
            format!(
                "**`{}`** {}",
                bj_user.total_citrine_lost,
                ctx.emoji_text(EmojiType::Citrine)
            ),
            true,
        )
        .field(
            "Win Chance",
            format!("**`{:.2}%`**", bj_user.win_chance() * 100.0),
            true,
        )
        .field(
            "Net Gain",
            format!(
                "**`{}`** {}",
                bj_user.net_gain(),
                ctx.emoji_text(EmojiType::Citrine)
            ),
            true,
        );

    ctx.send(embed.create_reply()).await?;
    Ok(())
}
