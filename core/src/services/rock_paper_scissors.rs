use crate::database::entity::{rps_games, rps_user, user};
use crate::error::CoreResult;
use crate::games::rps::choice::RPSChoice;
use crate::games::rps::stats::RPSStats;
use crate::stores::Stores;
use sea_orm::{IntoActiveModel, Set};
use std::sync::Arc;

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

    pub async fn get_stats(&self, user: &user::Model) -> CoreResult<RPSStats> {
        let user_stats = self.stores.rps_games.get_user_stats(&user.id).await?;
        let rps_user = self.stores.rps_user.fetch_or_create(user).await?;

        Ok(RPSStats {
            wins: user_stats.wins as u64,
            losses: user_stats.losses as u64,
            draws: user_stats.draws as u64,
            rock: rps_user.times_rock as u64,
            paper: rps_user.times_paper as u64,
            scissors: rps_user.times_scissors as u64,
        })
    }
}
