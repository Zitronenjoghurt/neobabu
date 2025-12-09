use crate::context::ContextExt;
use crate::error::{BotError, BotResult};
use crate::ui::games::blackjack::BlackjackUi;
use crate::ui::message::interactive::InteractiveMessage;
use crate::Context;
use neobabu_core::games::blackjack::BlackjackGame;
use neobabu_core::types::currency::Currency;
use std::time::Duration;

/// Start a game of Blackjack, which up to 4 players can join.
#[poise::command(slash_command, guild_only, user_cooldown = "20")]
pub async fn start(
    ctx: Context<'_>,
    #[description = "How much Citrine to bet"] wager: Option<u32>,
) -> BotResult<()> {
    ctx.defer().await?;
    let user = ctx.fetch_author_model().await?;

    let mut game = BlackjackGame::default();

    if let Some(wager) = wager {
        if wager == 0 {
            return Err(BotError::WagerZero);
        }

        let balance = ctx
            .stores()
            .economy
            .balance(&user, Currency::Citrine)
            .await?;
        if wager as i64 > balance.available {
            return Err(BotError::InsufficientFunds(Currency::Citrine));
        }
        game = game.with_wager(wager);
    }

    ctx.services()
        .blackjack
        .register_user(&mut game, &user, 4)
        .await?;

    let ui = BlackjackUi::new(game);
    InteractiveMessage::new(&ctx, ui)
        .timeout(Duration::from_mins(14))
        .allow_anyone_to_interact(true)
        .tick_interval(Duration::from_secs(2))
        .run()
        .await?;

    Ok(())
}
