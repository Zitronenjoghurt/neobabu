use crate::database::entity::user;
use crate::error::CoreResult;
use crate::games::blackjack::{BlackjackGame, BlackjackOutcome};
use crate::types::currency::Currency;
use sea_orm::{IntoActiveModel, Set};
use std::sync::Arc;
use std::time::Duration;

pub struct BlackjackService {
    stores: Arc<crate::stores::Stores>,
}

impl BlackjackService {
    pub fn initialize(stores: &Arc<crate::stores::Stores>) -> Arc<Self> {
        Arc::new(Self {
            stores: stores.clone(),
        })
    }

    pub async fn resolve_game(&self, game: &BlackjackGame) -> CoreResult<()> {
        let Some(outcomes) = game.get_outcomes() else {
            return Ok(());
        };

        for (id, outcome) in outcomes {
            self.resolve_player(game, id, outcome).await?;
        }

        Ok(())
    }

    async fn resolve_player(
        &self,
        game: &BlackjackGame,
        player_id: String,
        outcome: BlackjackOutcome,
    ) -> CoreResult<()> {
        let Some(player) = game.players.get(&player_id) else {
            return Ok(());
        };

        let user = self.stores.user.fetch_or_create(&player_id).await?;
        let bj_user = self.stores.bj_user.fetch_or_create(&user).await?;
        let mut active = bj_user.clone().into_active_model();

        active.dealer_score_total = Set(bj_user
            .dealer_score_total
            .saturating_add(game.dealer.score() as i64));

        if player.standing {
            active.times_final_stand = Set(bj_user.times_final_stand.saturating_add(1));
            active.final_stand_score_total = Set(bj_user
                .final_stand_score_total
                .saturating_add(player.score() as i64));
        }

        if player.is_bust() {
            active.times_final_hit = Set(bj_user.times_final_hit.saturating_add(1));
            active.final_hit_score_total = Set(bj_user
                .final_hit_score_total
                .saturating_add(player.score() as i64))
        }

        if player.score() == 21 {
            active.blackjack_count = Set(bj_user.blackjack_count.saturating_add(1));
            active.blackjack_streak = Set(bj_user.blackjack_streak.saturating_add(1));
            if bj_user.blackjack_streak >= bj_user.longest_blackjack_streak {
                active.longest_blackjack_streak = Set(bj_user.blackjack_streak.saturating_add(1));
            }
        } else {
            active.blackjack_streak = Set(0);
        }

        match outcome {
            BlackjackOutcome::Win => {
                active.wins = Set(bj_user.wins.saturating_add(1));
                active.win_streak = Set(bj_user.win_streak.saturating_add(1));
                if bj_user.win_streak >= bj_user.longest_win_streak {
                    active.longest_win_streak = Set(bj_user.win_streak.saturating_add(1));
                }
                if bj_user.loss_streak > 0 {
                    active.loss_streak = Set(0);
                }
                if bj_user.draw_streak > 0 {
                    active.draw_streak = Set(0);
                }

                if let Some(wager) = game.wager {
                    active.total_citrine_wagered =
                        Set(bj_user.total_citrine_wagered.saturating_add(wager as i64));
                    active.total_citrine_won =
                        Set(bj_user.total_citrine_won.saturating_add(wager as i64));
                    let reference_id = format!("bj-{}", game.id);
                    self.stores
                        .economy
                        .cancel(&reference_id, &user, Currency::Citrine)
                        .await?;
                    self.stores
                        .economy
                        .add(&user, Currency::Citrine, wager as i64)
                        .await?;
                }
            }
            BlackjackOutcome::Loss => {
                active.losses = Set(bj_user.losses.saturating_add(1));
                active.loss_streak = Set(bj_user.loss_streak.saturating_add(1));
                if bj_user.loss_streak >= bj_user.longest_loss_streak {
                    active.longest_loss_streak = Set(bj_user.loss_streak.saturating_add(1));
                }
                if bj_user.win_streak > 0 {
                    active.win_streak = Set(0);
                }
                if bj_user.draw_streak > 0 {
                    active.draw_streak = Set(0);
                }

                if let Some(wager) = game.wager {
                    active.total_citrine_wagered =
                        Set(bj_user.total_citrine_wagered.saturating_add(wager as i64));
                    active.total_citrine_lost =
                        Set(bj_user.total_citrine_lost.saturating_add(wager as i64));
                    let reference_id = format!("bj-{}", game.id);
                    self.stores
                        .economy
                        .commit(&reference_id, &user, Currency::Citrine)
                        .await?;
                }
            }
            BlackjackOutcome::Push => {
                active.draws = Set(bj_user.draws.saturating_add(1));
                active.draw_streak = Set(bj_user.draw_streak.saturating_add(1));
                if bj_user.draw_streak >= bj_user.longest_draw_streak {
                    active.longest_draw_streak = Set(bj_user.draw_streak.saturating_add(1));
                }
                if bj_user.win_streak > 0 {
                    active.win_streak = Set(0);
                }
                if bj_user.loss_streak > 0 {
                    active.loss_streak = Set(0);
                }

                if let Some(wager) = game.wager {
                    active.total_citrine_wagered =
                        Set(bj_user.total_citrine_wagered.saturating_add(wager as i64));
                    let reference_id = format!("bj-{}", game.id);
                    self.stores
                        .economy
                        .cancel(&reference_id, &user, Currency::Citrine)
                        .await?;
                }
            }
        }

        self.stores.bj_user.update(active).await?;

        Ok(())
    }

    pub async fn register_user(
        &self,
        game: &mut BlackjackGame,
        user: &user::Model,
        max: usize,
    ) -> CoreResult<()> {
        if game.players.contains_key(&user.id) || game.players.len() >= max {
            return Ok(());
        };

        if let Some(wager) = game.wager {
            let reference_id = format!("bj-{}", game.id);
            let reservation_successful = self
                .stores
                .economy
                .reserve(
                    reference_id,
                    Duration::from_mins(20),
                    user,
                    Currency::Citrine,
                    wager as i64,
                )
                .await?;
            if !reservation_successful {
                return Ok(());
            }
        };

        game.register_player(&user.id);
        Ok(())
    }
}
