use crate::context::ContextExt;
use crate::error::BotResult;
use crate::ui::color::UiColor;
use crate::ui::emoji::EmojiType;
use crate::ui::message::interactive::state::{InteractiveState, InteractiveStateResponse};
use crate::ui::message::CreateEmbedExt;
use crate::ui::time::format_time_relative_at;
use crate::Context;
use chrono::{DateTime, Utc};
use neobabu_core::games::rps::choice::RPSChoice;
use neobabu_core::games::rps::state::RPSState;
use neobabu_core::games::rps::RPSGame;
use poise::serenity_prelude::{
    ButtonStyle, ComponentInteraction, CreateActionRow, CreateButton, CreateEmbed,
};
use poise::CreateReply;

pub struct RPSUi {
    pub game: RPSGame,
    pub timeout_at: DateTime<Utc>,
    pub bot_session: bool,
}

impl RPSUi {
    pub fn build_embed(&self, ctx: &Context) -> CreateEmbed {
        let description = self.build_description(ctx);
        let color = self.build_color();

        CreateEmbed::new()
            .title("Rock Paper Scissors")
            .ui_color(color)
            .description(description)
    }

    fn build_description(&self, ctx: &Context) -> String {
        let state = self.game.state();

        let time_left = if state.is_ongoing() {
            format!(
                "\n\n*{} Game ends {}*",
                ctx.emoji_text(EmojiType::Clock),
                format_time_relative_at(self.timeout_at)
            )
        } else {
            "".to_string()
        };

        let face_1 = ctx.emoji_text(self.get_face(true));
        let choice_1 = self.get_choice_text(ctx, true);

        let face_2 = ctx.emoji_text(self.get_face(false));
        let choice_2 = self.get_choice_text(ctx, false);

        let state = self.get_state_text(ctx);

        format!(
            "{face_1} <@{}> {choice_1}\n{face_2} <@{}> {choice_2}{state}{time_left}",
            self.game.user_1.id, self.game.user_2.id
        )
    }

    fn build_color(&self) -> UiColor {
        match self.game.state() {
            RPSState::Winner1 => UiColor::Success,
            RPSState::Winner2 => UiColor::Error,
            RPSState::Draw => UiColor::Warning,
            _ => UiColor::Pink,
        }
    }

    fn get_face(&self, is_1: bool) -> EmojiType {
        if self.bot_session && !is_1 {
            return EmojiType::FaceRobot;
        }

        match self.game.state() {
            RPSState::WaitingForBoth => EmojiType::FaceThinking,
            RPSState::WaitingFor1 => {
                if is_1 {
                    EmojiType::FaceThinking
                } else {
                    EmojiType::FaceShushing
                }
            }
            RPSState::WaitingFor2 => {
                if is_1 {
                    EmojiType::FaceShushing
                } else {
                    EmojiType::FaceThinking
                }
            }
            RPSState::Winner1 => {
                if is_1 {
                    EmojiType::random_winner()
                } else {
                    EmojiType::random_loser()
                }
            }
            RPSState::Winner2 => {
                if is_1 {
                    EmojiType::random_loser()
                } else {
                    EmojiType::random_winner()
                }
            }
            RPSState::Draw => EmojiType::random_loser(),
        }
    }

    fn get_choice_text(&self, ctx: &Context, is_1: bool) -> String {
        let state = self.game.state();
        let choice = if is_1 {
            self.game.choice_1
        } else {
            self.game.choice_2
        };

        if let Some(choice) = choice {
            if state.is_ongoing() {
                "**chose `???`**".to_string()
            } else {
                format!("**chose** {}", Self::emoji_from_choice(ctx, choice))
            }
        } else {
            "*is thinking...*".to_string()
        }
    }

    fn get_state_text(&self, ctx: &Context) -> String {
        match self.game.state() {
            RPSState::WaitingForBoth | RPSState::WaitingFor1 | RPSState::WaitingFor2 => {
                "".to_string()
            }
            RPSState::Winner1 => {
                format!(
                    "\n\n{} <@{}> **wins!**",
                    ctx.emoji_text(EmojiType::Trophy),
                    self.game.user_1.id,
                )
            }
            RPSState::Winner2 => {
                format!(
                    "\n\n{} <@{}> **wins!**",
                    ctx.emoji_text(EmojiType::Trophy),
                    self.game.user_2.id,
                )
            }
            RPSState::Draw => {
                format!("\n\n**{} It's a draw!**", ctx.emoji_text(EmojiType::Pvp))
            }
        }
    }

    fn emoji_from_choice(ctx: &Context, choice: RPSChoice) -> String {
        match choice {
            RPSChoice::Rock => ctx.emoji_text(EmojiType::Rock),
            RPSChoice::Paper => ctx.emoji_text(EmojiType::Paper),
            RPSChoice::Scissors => ctx.emoji_text(EmojiType::Scissors),
        }
    }

    pub async fn on_choice(
        &mut self,
        ctx: &Context<'_>,
        interaction: &ComponentInteraction,
        choice: RPSChoice,
    ) -> BotResult<InteractiveStateResponse> {
        let did_play = self.game.play(interaction.user.id.to_string(), choice);
        if !did_play {
            return Ok(InteractiveStateResponse::new());
        }

        let state = self.game.state();
        if state.is_finished() {
            self.game.register_end(&ctx.data().core).await?;
        }

        match state {
            RPSState::Winner1 => {
                ctx.send(CreateReply::default().content(format!(
                    "**<@{}> `won` against <@{}>, in Rock Paper Scissors!!**",
                    self.game.user_1.id, self.game.user_2.id
                )))
                .await?;
            }
            RPSState::Winner2 => {
                ctx.send(CreateReply::default().content(format!(
                    "**<@{}> `won` against <@{}>, in Rock Paper Scissors!!**",
                    self.game.user_2.id, self.game.user_1.id
                )))
                .await?;
            }
            RPSState::Draw => {
                ctx.send(CreateReply::default().content(format!(
                    "**<@{}> and <@{}>, your game of Rock Paper Scissors ended in a `draw`!**",
                    self.game.user_1.id, self.game.user_2.id
                )))
                .await?;
            }
            _ => {}
        }

        Ok(InteractiveStateResponse::new()
            .update(true)
            .stop(self.game.state().is_finished()))
    }
}

#[async_trait::async_trait]
impl InteractiveState for RPSUi {
    async fn handle_interaction(
        &mut self,
        ctx: &Context,
        interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveStateResponse> {
        match interaction.data.custom_id.as_str() {
            "rps_rock" => self.on_choice(ctx, interaction, RPSChoice::Rock).await,
            "rps_paper" => self.on_choice(ctx, interaction, RPSChoice::Paper).await,
            "rps_scissors" => self.on_choice(ctx, interaction, RPSChoice::Scissors).await,
            _ => Ok(InteractiveStateResponse::new()),
        }
    }

    async fn render_content(&self, _ctx: &Context) -> BotResult<Option<String>> {
        if !self.game.state().is_finished() {
            Ok(Some(format!(
                "**<@{}>, you were challenged to a game of Rock Paper Scissors by <@{}>!**",
                self.game.user_2.id, self.game.user_1.id
            )))
        } else {
            Ok(None)
        }
    }

    async fn render_embed(&self, ctx: &Context) -> BotResult<CreateEmbed> {
        Ok(self.build_embed(ctx))
    }

    async fn render_rows(&self, ctx: &Context) -> BotResult<Vec<CreateActionRow>> {
        if self.game.state().is_finished() {
            Ok(vec![])
        } else {
            Ok(vec![CreateActionRow::Buttons(vec![
                CreateButton::new("rps_rock")
                    .style(ButtonStyle::Secondary)
                    .emoji(ctx.emoji(EmojiType::Rock)),
                CreateButton::new("rps_paper")
                    .style(ButtonStyle::Secondary)
                    .emoji(ctx.emoji(EmojiType::Paper)),
                CreateButton::new("rps_scissors")
                    .style(ButtonStyle::Secondary)
                    .emoji(ctx.emoji(EmojiType::Scissors)),
            ])])
        }
    }
}
