use crate::games::connect4::state::Connect4State;
use minimax::{Evaluation, Game, Negamax, Strategy};

pub struct Connect4AI;

impl Connect4AI {
    pub fn best_move(state: &Connect4State, depth: u8) -> Option<u8> {
        Negamax::new(Self, depth).choose_move(state)
    }
}

impl minimax::Evaluator for Connect4AI {
    type G = Connect4State;

    fn evaluate(&self, state: &<Self::G as Game>::S) -> Evaluation {
        let my_board = if state.is_1_to_play {
            state.board.player_1.value()
        } else {
            state.board.player_2.value()
        };
        let opp_board = if state.is_1_to_play {
            state.board.player_2.value()
        } else {
            state.board.player_1.value()
        };

        let my_score = evaluate_position(my_board, opp_board);
        let opp_score = evaluate_position(opp_board, my_board);

        my_score - opp_score
    }
}

fn evaluate_position(player: u64, opponent: u64) -> Evaluation {
    let mut score: Evaluation = 0;

    score += (player & (0x3F << 21)).count_ones() as Evaluation * 4;
    score += (player & (0x3F << 14)).count_ones() as Evaluation * 2;
    score += (player & (0x3F << 28)).count_ones() as Evaluation * 2;
    score += count_threats(player, opponent) as Evaluation * 10;

    score
}

fn count_threats(player: u64, opponent: u64) -> u32 {
    let empty = !(player | opponent);
    let mut threats = 0;

    threats += count_line_threats(player, empty, 7);
    threats += count_line_threats(player, empty, 1);
    threats += count_line_threats(player, empty, 8);
    threats += count_line_threats(player, empty, 6);

    threats
}

fn count_line_threats(player: u64, empty: u64, shift: u32) -> u32 {
    let p1 = player;
    let p2 = player >> shift;
    let p3 = player >> (shift * 2);
    let p4 = player >> (shift * 3);

    let e1 = empty;
    let e2 = empty >> shift;
    let e3 = empty >> (shift * 2);
    let e4 = empty >> (shift * 3);

    let threats =
        (p1 & p2 & p3 & e4) | (p1 & p2 & e3 & p4) | (p1 & e2 & p3 & p4) | (e1 & p2 & p3 & p4);

    threats.count_ones()
}
