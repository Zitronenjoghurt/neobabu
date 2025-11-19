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
use neobabu_core::database::entity::user;
use neobabu_core::services::rock_paper_scissors::RPSChoice;
use poise::serenity_prelude::{
    ButtonStyle, ComponentInteraction, CreateActionRow, CreateButton, CreateEmbed, User,
};
use poise::CreateReply;
use std::time::Duration;

/// Challenge a user to a game of Rock Paper Scissors.
#[poise::command(slash_command, guild_only, user_cooldown = "30")]
pub async fn challenge(ctx: Context<'_>, opponent: User) -> BotResult<()> {
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

    let choice_2 = if opponent.bot {
        Some(RPSChoice::random())
    } else {
        None
    };

    let row = RPSRow {
        user_1,
        user_2,
        choice_1: None,
        choice_2,
        timeout_at: Utc::now() + Duration::from_secs(300),
        bot_session: opponent.bot,
    };

    let embed = row.build_embed(&ctx, &None);

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
    user_1: user::Model,
    user_2: user::Model,
    choice_1: Option<RPSChoice>,
    choice_2: Option<RPSChoice>,
    timeout_at: DateTime<Utc>,
    bot_session: bool,
}

impl RPSRow {
    pub fn build_embed(&self, context: &Context, win_state: &Option<(bool, bool)>) -> CreateEmbed {
        let description = self.build_description(context, win_state);
        let color = self.build_color(win_state);

        CreateEmbed::new()
            .title("Rock Paper Scissors")
            .ui_color(color)
            .description(description)
    }

    fn build_description(&self, context: &Context, win_state: &Option<(bool, bool)>) -> String {
        let time_left = if win_state.is_none() {
            format!(
                "\n\n*{} Game ends {}*",
                context.emoji_text(EmojiType::Clock),
                format_time_relative_at(self.timeout_at)
            )
        } else {
            "".to_string()
        };

        let face_1 = if let Some((winner_1, draw)) = win_state {
            if *draw || !*winner_1 {
                context.emoji_text(EmojiType::random_loser())
            } else {
                context.emoji_text(EmojiType::random_winner())
            }
        } else {
            if self.choice_1.is_some() {
                context.emoji_text(EmojiType::FaceShushing)
            } else {
                context.emoji_text(EmojiType::FaceThinking)
            }
        };

        let choice_1 = if let Some(choice) = &self.choice_1 {
            if win_state.is_some() {
                format!("**chose** {}", Self::emoji_from_choice(context, *choice))
            } else {
                "**chose `???`**".to_string()
            }
        } else {
            "*is thinking...*".to_string()
        };

        let face_2 = if self.bot_session {
            context.emoji_text(EmojiType::FaceRobot)
        } else if let Some((winner_1, draw)) = win_state {
            if *draw || *winner_1 {
                context.emoji_text(EmojiType::random_loser())
            } else {
                context.emoji_text(EmojiType::random_winner())
            }
        } else {
            if self.choice_2.is_some() {
                context.emoji_text(EmojiType::FaceShushing)
            } else {
                context.emoji_text(EmojiType::FaceThinking)
            }
        };

        let choice_2 = if let Some(choice) = &self.choice_2 {
            if win_state.is_some() {
                format!("**chose** {}", Self::emoji_from_choice(context, *choice))
            } else {
                "**chose `???`**".to_string()
            }
        } else {
            "*is thinking...*".to_string()
        };

        let state = match win_state {
            Some((_, true)) => format!(
                "\n\n**{} It's a draw!**",
                context.emoji_text(EmojiType::Pvp)
            ),
            Some((true, false)) => format!(
                "\n\n{} <@{}> **wins!**",
                context.emoji_text(EmojiType::Trophy),
                self.user_1.id,
            ),
            Some((false, false)) => format!(
                "\n\n{} <@{}> **wins!**",
                context.emoji_text(EmojiType::Trophy),
                self.user_2.id,
            ),
            None => "".to_string(),
        };

        format!(
            "{face_1} <@{}> {choice_1}\n{face_2} <@{}> {choice_2}{state}{time_left}",
            self.user_1.id, self.user_2.id
        )
    }

    fn build_color(&self, win_state: &Option<(bool, bool)>) -> UiColor {
        match win_state {
            Some((true, false)) => UiColor::Success,
            Some((false, false)) => UiColor::Error,
            Some((_, true)) => UiColor::Warning,
            None => UiColor::Pink,
        }
    }

    fn determine_winner(&self) -> Option<(bool, bool)> {
        let Some(choice_1) = self.choice_1 else {
            return None;
        };
        let Some(choice_2) = self.choice_2 else {
            return None;
        };

        let is_draw = choice_1 == choice_2;
        if is_draw {
            return Some((false, true));
        };

        let is_user_1_winner = match (choice_1, choice_2) {
            (RPSChoice::Rock, RPSChoice::Scissors) => true,
            (RPSChoice::Paper, RPSChoice::Rock) => true,
            (RPSChoice::Scissors, RPSChoice::Paper) => true,
            _ => false,
        };
        Some((is_user_1_winner, false))
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
        if interaction.user.id.to_string() == self.user_1.id && self.choice_1.is_none() {
            self.choice_1 = Some(choice);
        } else if interaction.user.id.to_string() == self.user_2.id && self.choice_2.is_none() {
            self.choice_2 = Some(choice);
        } else {
            return Ok(InteractiveEmbedResponse::new());
        };

        let win_state = self.determine_winner();
        let embed = self.build_embed(context, &win_state);
        let row_update = if win_state.is_some() {
            InteractiveEmbedRowUpdate::Remove
        } else {
            InteractiveEmbedRowUpdate::Keep
        };

        if let Some((user_1_won, draw)) = win_state {
            if draw {
                context
                    .services()
                    .rps
                    .register_draw(&self.user_1, &self.user_2)
                    .await?;
            } else if user_1_won {
                context
                    .services()
                    .rps
                    .register_winner(&self.user_1, &self.user_2)
                    .await?;
            } else {
                context
                    .services()
                    .rps
                    .register_winner(&self.user_2, &self.user_1)
                    .await?;
            }

            if let Some(choice) = self.choice_1 {
                context
                    .services()
                    .rps
                    .register_choice(&self.user_1, choice)
                    .await?;
            }

            if let Some(choice) = self.choice_2 {
                context
                    .services()
                    .rps
                    .register_choice(&self.user_2, choice)
                    .await?;
            }

            if draw {
                context
                    .send(CreateReply::default().content(format!(
                        "**<@{}> and <@{}>, your game of Rock Paper Scissors ended in a `draw`!**",
                        self.user_1.id, self.user_2.id
                    )))
                    .await?;
            } else if user_1_won {
                context
                    .send(CreateReply::default().content(format!(
                        "**<@{}> `won` against <@{}>, in Rock Paper Scissors!!**",
                        self.user_1.id, self.user_2.id
                    )))
                    .await?;
            } else {
                context
                    .send(CreateReply::default().content(format!(
                        "**<@{}> `won` against <@{}>, in Rock Paper Scissors!!**",
                        self.user_2.id, self.user_1.id
                    )))
                    .await?;
            }
        }

        Ok(InteractiveEmbedResponse::new()
            .embed(embed)
            .row_update(row_update)
            .clear_content(win_state.is_some())
            .do_stop(win_state.is_some()))
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
