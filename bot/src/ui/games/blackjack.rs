use crate::context::ContextExt;
use crate::error::BotResult;
use crate::ui::emoji::EmojiType;
use crate::ui::message::interactive::state::{InteractiveState, InteractiveStateResponse};
use crate::ui::time::format_time_relative_at;
use crate::Context;
use neobabu_core::games::blackjack::{BlackjackGame, BlackjackMove, BlackjackOutcome};
use neobabu_core::games::playing_cards::PlayingCardDeck;
use poise::serenity_prelude::{
    ButtonStyle, Color, ComponentInteraction, CreateActionRow, CreateButton, CreateEmbed,
};
use std::collections::HashMap;
use std::ops::Add;

pub struct BlackjackUi {
    pub game: BlackjackGame,
    pub dealer_emoji: EmojiType,
    pub player_emoji: HashMap<String, EmojiType>,
    pub starts_at: chrono::DateTime<chrono::Utc>,
}

impl Default for BlackjackUi {
    fn default() -> Self {
        Self {
            game: BlackjackGame::new(),
            dealer_emoji: EmojiType::random_professional(),
            player_emoji: HashMap::new(),
            starts_at: chrono::Utc::now().add(chrono::Duration::seconds(20)),
        }
    }
}

impl BlackjackUi {
    fn player_to_move_till(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        let last_move = self.game.last_move?;
        let elapsed = last_move.elapsed();
        Some(
            chrono::Utc::now() - chrono::Duration::from_std(elapsed).ok()?
                + chrono::Duration::seconds(20),
        )
    }

    fn format_deck(&self, ctx: &Context, deck: &PlayingCardDeck) -> String {
        let mut result = String::new();
        for card in deck.iter_cards() {
            result.push_str(&ctx.emoji_text(card.into()));
        }
        result
    }

    fn format_dealer(&self, ctx: &Context) -> String {
        let emoji = ctx.emoji_text(self.dealer_emoji);

        if !self.game.has_started() {
            format!("{emoji} **`Dealer`** *is waiting...*")
        } else {
            let score = self.game.dealer.score();
            let deck = self.format_deck(ctx, self.game.dealer.deck());
            format!("{emoji} **`Dealer`** **`{score}`** {deck}")
        }
    }

    fn format_player(&self, ctx: &Context, id: impl AsRef<str>) -> String {
        let emoji_type = self
            .player_emoji
            .get(id.as_ref())
            .unwrap_or(&EmojiType::FaceBeamingSmile);
        let emoji = ctx.emoji_text(*emoji_type);

        if !self.game.has_started() {
            format!("{emoji} <@{}> *is waiting...*", id.as_ref())
        } else {
            let score = self
                .game
                .players
                .get(id.as_ref())
                .map(|p| p.score())
                .unwrap_or(0);
            let deck = self
                .game
                .players
                .get(id.as_ref())
                .map(|p| self.format_deck(ctx, p.deck()))
                .unwrap_or_default();

            let base_text = format!("{emoji} <@{}> **`{score}`** {deck}", id.as_ref());
            if self.game.is_to_play(id.as_ref()) {
                let to_move_till = self
                    .player_to_move_till()
                    .map(|time| format!(" | **AUTO-STAND** {}", format_time_relative_at(time)))
                    .unwrap_or_default();
                format!("{base_text}{to_move_till}")
            } else if self.game.is_bust(id.as_ref()) {
                format!("{base_text} | **BUST**")
            } else if self.game.is_stand(id.as_ref()) {
                format!("{base_text} | **STANDING**")
            } else {
                format!("{base_text} | *waiting...*")
            }
        }
    }

    fn format_players(&self, ctx: &Context) -> String {
        let wager = if let Some(wager) = self.game.wager {
            format!(
                "**WAGER: `{wager}`** {}\n\n",
                ctx.emoji_text(EmojiType::Citrine)
            )
        } else {
            "".to_string()
        };
        let dealer = self.format_dealer(ctx);
        let mut text = format!("{wager}{dealer}\n\n");
        for (id, _) in self.game.iter_players() {
            text.push_str(&self.format_player(ctx, id));
            text.push('\n');
        }
        text
    }

    fn embed_waiting(&self, ctx: &Context) -> CreateEmbed {
        let players = self.format_players(ctx);
        let description = format!(
            "*Game starts {}*\n\n{players}\n*4 people can join the game by clicking the button below.\nIf its your turn, you will have 20 seconds to hit or stand.\nThe game will start automatically.*",
            format_time_relative_at(self.starts_at)
        );

        CreateEmbed::default()
            .title("BLACKJACK | WAITING FOR PLAYERS")
            .description(description)
            .color(Color::default())
    }

    fn embed_playing(&self, ctx: &Context) -> CreateEmbed {
        let players = self.format_players(ctx);

        CreateEmbed::default()
            .title("BLACKJACK | GAME ON")
            .description(players)
            .color(Color::default())
    }

    fn embed_finished(&self, ctx: &Context) -> CreateEmbed {
        let outcomes = self.game.get_outcomes().unwrap_or_default();

        let dealer = self.format_dealer(ctx);

        let mut players = String::new();
        for (user_id, outcome) in outcomes {
            let Some(player) = self.game.players.get(&user_id) else {
                continue;
            };

            let emotion = match outcome {
                BlackjackOutcome::Win => ctx.emoji_text(EmojiType::random_winner()),
                BlackjackOutcome::Loss => ctx.emoji_text(EmojiType::random_loser()),
                BlackjackOutcome::Push => ctx.emoji_text(EmojiType::random_waiting()),
            };
            let score = player.score();
            let deck = self.format_deck(ctx, player.deck());

            let outcome = if let Some(wager) = self.game.wager {
                match outcome {
                    BlackjackOutcome::Win => {
                        format!("**`+{wager}`** {}", ctx.emoji_text(EmojiType::Citrine))
                    }
                    BlackjackOutcome::Loss => {
                        format!("**`-{wager}`** {}", ctx.emoji_text(EmojiType::Citrine))
                    }
                    BlackjackOutcome::Push => {
                        format!("**`±0`** {}", ctx.emoji_text(EmojiType::Citrine))
                    }
                }
            } else {
                match outcome {
                    BlackjackOutcome::Win => "**`WON`**",
                    BlackjackOutcome::Loss => "**`LOST`**",
                    BlackjackOutcome::Push => "**`PUSHED`**",
                }
                .to_string()
            };

            let text = format!("{emotion} <@{user_id}> **`{score}`** {deck} | {outcome}");
            players.push_str(&text);
            players.push('\n');
        }

        let description = format!("{dealer}\n\n{players}");

        CreateEmbed::default()
            .title("BLACKJACK | FINISHED")
            .description(description)
            .color(Color::default())
    }

    async fn handle_join(
        &mut self,
        ctx: &Context<'_>,
        id: impl AsRef<str>,
    ) -> BotResult<InteractiveStateResponse> {
        if !self.game.players.contains_key(id.as_ref()) {
            let user = ctx.stores().user.fetch_or_create(&id).await?;
            ctx.services()
                .blackjack
                .register_user(&mut self.game, &user, 4)
                .await?;
            self.player_emoji
                .insert(id.as_ref().to_string(), EmojiType::random_waiting());
            Ok(InteractiveStateResponse::new_update())
        } else {
            Ok(InteractiveStateResponse::default())
        }
    }

    fn handle_play(
        &mut self,
        id: impl AsRef<str>,
        move_: BlackjackMove,
    ) -> InteractiveStateResponse {
        let did_play = self.game.play(id.as_ref(), move_);
        if did_play {
            InteractiveStateResponse::new_update()
        } else {
            InteractiveStateResponse::default()
        }
    }

    async fn handle_finish(&self, ctx: &Context<'_>) -> BotResult<()> {
        Ok(ctx.services().blackjack.resolve_game(&self.game).await?)
    }
}

impl BlackjackUi {
    pub fn new(game: BlackjackGame) -> Self {
        Self {
            game,
            ..Default::default()
        }
    }
}

#[async_trait::async_trait]
impl InteractiveState for BlackjackUi {
    async fn handle_interaction(
        &mut self,
        ctx: &Context,
        interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveStateResponse> {
        let id = interaction.user.id.to_string();
        let response = match interaction.data.custom_id.as_str() {
            "join_game" => self.handle_join(ctx, id).await?,
            "play_hit" => self.handle_play(id, BlackjackMove::Hit),
            "play_stand" => self.handle_play(id, BlackjackMove::Stand),
            _ => return Ok(InteractiveStateResponse::default()),
        };

        if self.game.is_over() {
            self.handle_finish(ctx).await?;
            Ok(response.stop(true))
        } else {
            Ok(response)
        }
    }

    async fn render_embed(&self, ctx: &Context) -> BotResult<CreateEmbed> {
        if !self.game.has_started() {
            Ok(self.embed_waiting(ctx))
        } else if !self.game.is_over() {
            Ok(self.embed_playing(ctx))
        } else {
            Ok(self.embed_finished(ctx))
        }
    }

    async fn render_rows(&self, _ctx: &Context) -> BotResult<Vec<CreateActionRow>> {
        if !self.game.has_started() {
            let text = if let Some(wager) = self.game.wager {
                format!("Join ({wager} Citrine)")
            } else {
                "Join".to_string()
            };

            Ok(vec![CreateActionRow::Buttons(vec![
                CreateButton::new("join_game")
                    .label(text)
                    .style(ButtonStyle::Success),
            ])])
        } else {
            Ok(vec![CreateActionRow::Buttons(vec![
                CreateButton::new("play_hit")
                    .label("Hit")
                    .style(ButtonStyle::Success),
                CreateButton::new("play_stand")
                    .label("Stand")
                    .style(ButtonStyle::Danger),
            ])])
        }
    }

    async fn on_tick(&mut self, ctx: &Context) -> BotResult<InteractiveStateResponse> {
        let now = chrono::Utc::now();
        let mut update = false;
        let mut stop = false;

        if !self.game.has_started() && now >= self.starts_at {
            self.game.start_game();
            update = true;
        }

        if let Some(player_id) = self.game.current_player.clone()
            && let Some(to_move_till) = self.player_to_move_till()
            && now >= to_move_till
        {
            self.game.play(player_id, BlackjackMove::Stand);
            update = true;
            if self.game.is_over() {
                self.handle_finish(ctx).await?;
                stop = true;
            }
        }

        Ok(InteractiveStateResponse::default()
            .update(update)
            .stop(stop))
    }
}
