use crate::games::connect4::state::Connect4State;
use uuid::Uuid;

pub mod ai;
pub mod board;
pub mod coords;
pub mod state;

#[derive(Debug)]
pub struct Connect4Game {
    pub id: Uuid,
    pub player_1: String,
    pub player_2: String,
    pub state: Connect4State,
}

impl Connect4Game {
    pub fn new(player_1: impl Into<String>, player_2: impl Into<String>) -> Self {
        let original_1 = player_1.into();
        let original_2 = player_2.into();

        let (player_1, player_2) = if original_1 >= original_2 {
            (original_1, original_2)
        } else {
            (original_2, original_1)
        };

        Self {
            id: Uuid::new_v4(),
            player_1,
            player_2,
            state: Connect4State::default(),
        }
    }

    pub fn play(&mut self, player_id: impl AsRef<str>, col: u8) -> bool {
        let is_player_1 = player_id.as_ref() == self.player_1.as_str();
        if !is_player_1 && player_id.as_ref() != self.player_2.as_str() {
            return false;
        }

        if is_player_1 != self.state.is_1_to_play {
            return false;
        }

        self.state.play(col);
        true
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn play_ai(&mut self) {
        let focus = rand::random();

        let depth = match focus {
            0.0..=0.2 => 1,
            0.2..=0.50 => 2,
            0.5..=0.8 => 3,
            _ => 4,
        };

        let Some(best_move) = ai::Connect4AI::best_move(&self.state, depth) else {
            return;
        };

        self.state.play(best_move);
    }

    pub fn winner(&self) -> Option<bool> {
        if self.state.board.has_won(true) {
            Some(true)
        } else if self.state.board.has_won(false) {
            Some(false)
        } else {
            None
        }
    }
}
