use crate::database::entity::user;
use crate::error::CoreResult;
use crate::games::rps::choice::RPSChoice;
use crate::games::rps::state::RPSState;
use crate::NeobabuCore;

pub mod choice;
pub mod state;
pub mod stats;

pub struct RPSGame {
    pub user_1: user::Model,
    pub user_2: user::Model,
    pub choice_1: Option<RPSChoice>,
    pub choice_2: Option<RPSChoice>,
}

impl RPSGame {
    pub fn new(user_1: user::Model, user_2: user::Model) -> Self {
        Self {
            user_1,
            user_2,
            choice_1: None,
            choice_2: None,
        }
    }

    pub fn state(&self) -> RPSState {
        if self.choice_1.is_none() && self.choice_2.is_none() {
            return RPSState::WaitingForBoth;
        };

        let Some(choice_1) = self.choice_1 else {
            return RPSState::WaitingFor1;
        };

        let Some(choice_2) = self.choice_2 else {
            return RPSState::WaitingFor2;
        };

        if choice_1 == choice_2 {
            return RPSState::Draw;
        };

        let is_user_1_winner = match (choice_1, choice_2) {
            (RPSChoice::Rock, RPSChoice::Scissors) => true,
            (RPSChoice::Paper, RPSChoice::Rock) => true,
            (RPSChoice::Scissors, RPSChoice::Paper) => true,
            _ => false,
        };

        if is_user_1_winner {
            RPSState::Winner1
        } else {
            RPSState::Winner2
        }
    }

    pub fn play(&mut self, id: impl AsRef<str>, choice: RPSChoice) -> bool {
        if id.as_ref() == self.user_1.id && self.choice_1.is_none() {
            self.choice_1 = Some(choice);
            true
        } else if id.as_ref() == self.user_2.id && self.choice_2.is_none() {
            self.choice_2 = Some(choice);
            true
        } else {
            false
        }
    }

    pub async fn register_end(&self, core: &NeobabuCore) -> CoreResult<()> {
        match self.state() {
            RPSState::Winner1 => {
                core.services
                    .rps
                    .register_winner(&self.user_1, &self.user_2)
                    .await?;
            }
            RPSState::Winner2 => {
                core.services
                    .rps
                    .register_winner(&self.user_2, &self.user_1)
                    .await?;
            }
            RPSState::Draw => {
                core.services
                    .rps
                    .register_draw(&self.user_1, &self.user_2)
                    .await?;
            }
            RPSState::WaitingForBoth | RPSState::WaitingFor1 | RPSState::WaitingFor2 => {
                return Ok(());
            }
        };

        if let Some(choice_1) = self.choice_1 {
            core.services
                .rps
                .register_choice(&self.user_1, choice_1)
                .await?;
        };

        if let Some(choice_2) = self.choice_2 {
            core.services
                .rps
                .register_choice(&self.user_2, choice_2)
                .await?;
        };

        Ok(())
    }
}
