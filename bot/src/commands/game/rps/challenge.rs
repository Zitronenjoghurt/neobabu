use crate::context::ContextExt;
use crate::error::{BotError, BotResult};
use crate::ui::color::UiColor;
use crate::ui::games::rps::RPSUi;
use crate::ui::message::interactive::InteractiveMessage;
use crate::ui::message::CreateEmbedExt;
use crate::Context;
use chrono::Utc;
use neobabu_core::games::rps::choice::RPSChoice;
use neobabu_core::games::rps::RPSGame;
use poise::serenity_prelude::{CreateEmbed, User};
use std::time::Duration;

/// Challenge a user to a game of Rock Paper Scissors.
#[poise::command(slash_command, guild_only, user_cooldown = "20")]
pub async fn challenge(ctx: Context<'_>, opponent: User) -> BotResult<()> {
    ctx.defer().await?;

    let author = ctx.author();
    if author.id == opponent.id {
        return Err(BotError::TargetYourself);
    }

    let user_1 = ctx.fetch_author_model().await?;
    let user_2 = ctx
        .stores()
        .user
        .fetch_or_create(opponent.id.to_string())
        .await?;

    let timeout_embed = CreateEmbed::new()
        .ui_color(UiColor::Gray)
        .title("Rock Paper Scissors")
        .description(format!(
            "The game between <@{}> and <@{}> has ended without a winner.",
            user_1.id, user_2.id
        ));

    let mut game = RPSGame::new(user_1, user_2);
    if opponent.bot {
        game.choice_2 = Some(RPSChoice::random());
    }

    let state = RPSUi {
        game,
        timeout_at: Utc::now() + Duration::from_secs(300),
        bot_session: opponent.bot,
    };

    InteractiveMessage::new(&ctx, state)
        .timeout(Duration::from_secs(300))
        .on_timeout(timeout_embed)
        .allow_anyone_to_interact(true)
        .run()
        .await?;

    Ok(())
}
