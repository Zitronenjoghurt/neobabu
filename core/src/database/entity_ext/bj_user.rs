use crate::database::entity::black_jack_user;

impl black_jack_user::Model {
    pub fn games_played(&self) -> i32 {
        self.wins + self.losses + self.draws
    }

    pub fn avg_stand(&self) -> f32 {
        (self.final_stand_score_total as f32 / self.times_final_stand as f32)
    }

    pub fn avg_dealer(&self) -> f32 {
        (self.dealer_score_total as f32 / self.games_played() as f32)
    }

    pub fn win_chance(&self) -> f32 {
        self.wins as f32 / self.games_played() as f32
    }

    pub fn net_gain(&self) -> i64 {
        self.total_citrine_won
            .saturating_sub(self.total_citrine_lost)
    }
}
