use crate::error::BotResult;
use crate::ui::games::blackjack::BlackjackUi;
use crate::ui::message::interactive::InteractiveMessage;
use crate::Context;
use neobabu_core::games::blackjack::BlackjackGame;
use std::time::Duration;

/// Start a game of Blackjack, which up to 4 players can join.
#[poise::command(slash_command, guild_only, user_cooldown = "30")]
pub async fn start(ctx: Context<'_>) -> BotResult<()> {
    ctx.defer().await?;

    let mut game = BlackjackGame::default();
    game.register_player(ctx.author().id.to_string());
    let ui = BlackjackUi::new(game);
    InteractiveMessage::new(&ctx, ui)
        .timeout(Duration::from_mins(14))
        .allow_anyone_to_interact(true)
        .tick_interval(Duration::from_secs(2))
        .run()
        .await?;

    Ok(())
}
