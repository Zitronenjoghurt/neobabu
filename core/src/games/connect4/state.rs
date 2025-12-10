use crate::games::connect4::board::Connect4Board;
use minimax::Winner;

#[derive(Debug, Clone, Copy)]
pub struct Connect4State {
    pub is_1_to_play: bool,
    pub board: Connect4Board,
}

impl Connect4State {
    pub fn play(&mut self, col: u8) {
        self.board.play(col, self.is_1_to_play);
        self.is_1_to_play = !self.is_1_to_play;
    }
}

impl Default for Connect4State {
    fn default() -> Self {
        Self {
            is_1_to_play: true,
            board: Connect4Board::default(),
        }
    }
}

impl minimax::Game for Connect4State {
    type S = Self;
    type M = u8;

    fn generate_moves(state: &Self::S, moves: &mut Vec<Self::M>) {
        moves.clear();
        for col in 0..7 {
            if state.board.heights[col as usize] < 6 {
                moves.push(col);
            }
        }
    }

    fn apply(state: &mut Self::S, m: Self::M) -> Option<Self::S> {
        let mut new_state = *state;
        new_state.play(m);
        Some(new_state)
    }

    fn get_winner(state: &Self::S) -> Option<Winner> {
        let just_moved = !state.is_1_to_play;
        if state.board.has_won(just_moved) {
            Some(Winner::PlayerJustMoved)
        } else {
            let mut moves = Vec::new();
            Self::generate_moves(state, &mut moves);
            if moves.is_empty() {
                Some(Winner::Draw)
            } else {
                None
            }
        }
    }
}
