use crate::context::ContextExt;
use crate::error::{BotError, BotResult};
use crate::ui::color::UiColor;
use crate::ui::embed::interactive::response::{
    InteractiveEmbedResponse, InteractiveEmbedRowUpdate,
};
use crate::ui::embed::interactive::rows::InteractiveRow;
use crate::ui::embed::interactive::InteractiveEmbed;
use crate::ui::embed::CreateEmbedExt;
use crate::ui::emoji::EmojiType;
use crate::ui::time::format_time_relative_at;
use crate::Context;
use chrono::{DateTime, Utc};
use neobabu_core::games::rps::choice::RPSChoice;
use neobabu_core::games::rps::state::RPSState;
use neobabu_core::games::rps::RPSGame;
use poise::serenity_prelude::{
    ButtonStyle, ComponentInteraction, CreateActionRow, CreateButton, CreateEmbed, User,
};
use poise::CreateReply;
use std::time::Duration;

/// Challenge a user to a game of Rock Paper Scissors.
#[poise::command(slash_command, guild_only, user_cooldown = "30")]
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

    let message = format!(
        "**<@{}>, you were challenged to a game of Rock Paper Scissors by <@{}>!**",
        user_2.id, user_1.id
    );
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

    let row = RPSRow {
        game,
        timeout_at: Utc::now() + Duration::from_secs(300),
        bot_session: opponent.bot,
    };

    let embed = row.build_embed(&ctx);

    InteractiveEmbed::new(&ctx, embed)
        .row(row)
        .content(message)
        .timeout(Duration::from_secs(300))
        .on_timeout(timeout_embed)
        .allow_anyone_to_interact(true)
        .run()
        .await?;

    Ok(())
}

struct RPSRow {
    game: RPSGame,
    timeout_at: DateTime<Utc>,
    bot_session: bool,
}

impl RPSRow {
    pub fn build_embed(&self, context: &Context) -> CreateEmbed {
        let description = self.build_description(context);
        let color = self.build_color();

        CreateEmbed::new()
            .title("Rock Paper Scissors")
            .ui_color(color)
            .description(description)
    }

    fn build_description(&self, context: &Context) -> String {
        let state = self.game.state();

        let time_left = if state.is_ongoing() {
            format!(
                "\n\n*{} Game ends {}*",
                context.emoji_text(EmojiType::Clock),
                format_time_relative_at(self.timeout_at)
            )
        } else {
            "".to_string()
        };

        let face_1 = context.emoji_text(self.get_face(true));
        let choice_1 = self.get_choice_text(context, true);

        let face_2 = context.emoji_text(self.get_face(false));
        let choice_2 = self.get_choice_text(context, false);

        let state = self.get_state_text(context);

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

    fn get_choice_text(&self, context: &Context, is_1: bool) -> String {
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
                format!("**chose** {}", Self::emoji_from_choice(context, choice))
            }
        } else {
            "*is thinking...*".to_string()
        }
    }

    fn get_state_text(&self, context: &Context) -> String {
        match self.game.state() {
            RPSState::WaitingForBoth | RPSState::WaitingFor1 | RPSState::WaitingFor2 => {
                "".to_string()
            }
            RPSState::Winner1 => {
                format!(
                    "\n\n{} <@{}> **wins!**",
                    context.emoji_text(EmojiType::Trophy),
                    self.game.user_1.id,
                )
            }
            RPSState::Winner2 => {
                format!(
                    "\n\n{} <@{}> **wins!**",
                    context.emoji_text(EmojiType::Trophy),
                    self.game.user_2.id,
                )
            }
            RPSState::Draw => {
                format!(
                    "\n\n**{} It's a draw!**",
                    context.emoji_text(EmojiType::Pvp)
                )
            }
        }
    }

    fn emoji_from_choice(context: &Context, choice: RPSChoice) -> String {
        match choice {
            RPSChoice::Rock => context.emoji_text(EmojiType::Rock),
            RPSChoice::Paper => context.emoji_text(EmojiType::Paper),
            RPSChoice::Scissors => context.emoji_text(EmojiType::Scissors),
        }
    }

    pub async fn on_choice(
        &mut self,
        context: &Context<'_>,
        interaction: &ComponentInteraction,
        choice: RPSChoice,
    ) -> BotResult<InteractiveEmbedResponse> {
        let did_play = self.game.play(interaction.user.id.to_string(), choice);
        if !did_play {
            return Ok(InteractiveEmbedResponse::new());
        }

        let state = self.game.state();
        if state.is_finished() {
            self.game.register_end(&context.data().core).await?;
        }

        let embed = self.build_embed(context);
        let row_update = if state.is_finished() {
            InteractiveEmbedRowUpdate::Remove
        } else {
            InteractiveEmbedRowUpdate::Keep
        };

        match state {
            RPSState::Winner1 => {
                context
                    .send(CreateReply::default().content(format!(
                        "**<@{}> `won` against <@{}>, in Rock Paper Scissors!!**",
                        self.game.user_1.id, self.game.user_2.id
                    )))
                    .await?;
            }
            RPSState::Winner2 => {
                context
                    .send(CreateReply::default().content(format!(
                        "**<@{}> `won` against <@{}>, in Rock Paper Scissors!!**",
                        self.game.user_2.id, self.game.user_1.id
                    )))
                    .await?;
            }
            RPSState::Draw => {
                context
                    .send(CreateReply::default().content(format!(
                        "**<@{}> and <@{}>, your game of Rock Paper Scissors ended in a `draw`!**",
                        self.game.user_1.id, self.game.user_2.id
                    )))
                    .await?;
            }
            _ => {}
        }

        Ok(InteractiveEmbedResponse::new()
            .embed(embed)
            .row_update(row_update)
            .clear_content(state.is_finished())
            .do_stop(state.is_finished()))
    }
}

#[async_trait::async_trait]
impl InteractiveRow for RPSRow {
    fn render(&self, context: &Context) -> Option<CreateActionRow> {
        Some(CreateActionRow::Buttons(vec![
            CreateButton::new("rps_rock")
                .style(ButtonStyle::Secondary)
                .emoji(context.emoji(EmojiType::Rock)),
            CreateButton::new("rps_paper")
                .style(ButtonStyle::Secondary)
                .emoji(context.emoji(EmojiType::Paper)),
            CreateButton::new("rps_scissors")
                .style(ButtonStyle::Secondary)
                .emoji(context.emoji(EmojiType::Scissors)),
        ]))
    }

    fn matches(&self, custom_id: &str) -> bool {
        custom_id == "rps_rock" || custom_id == "rps_paper" || custom_id == "rps_scissors"
    }

    async fn handle(
        &mut self,
        context: &Context,
        interaction: &ComponentInteraction,
    ) -> BotResult<InteractiveEmbedResponse> {
        match interaction.data.custom_id.as_str() {
            "rps_rock" => self.on_choice(context, interaction, RPSChoice::Rock).await,
            "rps_paper" => self.on_choice(context, interaction, RPSChoice::Paper).await,
            "rps_scissors" => {
                self.on_choice(context, interaction, RPSChoice::Scissors)
                    .await
            }
            _ => Ok(InteractiveEmbedResponse::new()),
        }
    }
}
