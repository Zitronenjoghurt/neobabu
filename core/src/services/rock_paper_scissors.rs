use crate::database::entity::{rps_games, rps_user, user};
use crate::error::CoreResult;
use crate::stores::Stores;
use rand::prelude::IndexedRandom;
use sea_orm::{IntoActiveModel, Set};
use std::fmt::Display;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

impl RPSChoice {
    pub fn random() -> Self {
        let choice = [Self::Rock, Self::Paper, Self::Scissors]
            .choose(&mut rand::rng())
            .copied();
        choice.unwrap_or(Self::Rock)
    }
}

impl Display for RPSChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rock => write!(f, "Rock"),
            Self::Paper => write!(f, "Paper"),
            Self::Scissors => write!(f, "Scissors"),
        }
    }
}

pub struct RockPaperScissorsService {
    stores: Arc<Stores>,
}

impl RockPaperScissorsService {
    pub fn initialize(stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            stores: stores.clone(),
        })
    }

    pub async fn register_winner(
        &self,
        winner: &user::Model,
        loser: &user::Model,
    ) -> CoreResult<rps_games::Model> {
        let game = self.stores.rps_games.fetch_or_create(winner, loser).await?;
        let winner_is_1 = winner.id == game.user_id1;

        let wins_1 = game.wins1;
        let wins_2 = game.wins2;
        let mut active = game.into_active_model();
        if winner_is_1 {
            active.wins1 = Set(wins_1.saturating_add(1));
        } else {
            active.wins2 = Set(wins_2.saturating_add(1));
        }

        self.stores.rps_games.update(active).await
    }

    pub async fn register_draw(
        &self,
        user_1: &user::Model,
        user_2: &user::Model,
    ) -> CoreResult<rps_games::Model> {
        let game = self
            .stores
            .rps_games
            .fetch_or_create(user_1, user_2)
            .await?;

        let draws = game.draws;
        let mut active = game.into_active_model();
        active.draws = Set(draws.saturating_add(1));

        self.stores.rps_games.update(active).await
    }

    pub async fn register_choice(
        &self,
        user: &user::Model,
        choice: RPSChoice,
    ) -> CoreResult<rps_user::Model> {
        let rps_user = self.stores.rps_user.fetch_or_create(user).await?;

        let choice_count = match choice {
            RPSChoice::Rock => rps_user.times_rock,
            RPSChoice::Paper => rps_user.times_paper,
            RPSChoice::Scissors => rps_user.times_scissors,
        };

        let mut active = rps_user.into_active_model();
        match choice {
            RPSChoice::Rock => active.times_rock = Set(choice_count.saturating_add(1)),
            RPSChoice::Paper => active.times_paper = Set(choice_count.saturating_add(1)),
            RPSChoice::Scissors => active.times_scissors = Set(choice_count.saturating_add(1)),
        }

        self.stores.rps_user.update(active).await
    }
}
