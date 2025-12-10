use crate::games::connect4::coords::Connect4Coords;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, Copy)]
pub struct Connect4Board {
    pub player_1: Connect4BoardMask,
    pub player_2: Connect4BoardMask,
    pub heights: [u8; 7],
}

impl Connect4Board {
    pub fn play(&mut self, col: u8, player_1: bool) {
        let height = self.heights[col as usize];
        let coords = Connect4Coords::from_row_col(height, col);

        if player_1 {
            self.player_1.set(coords);
        } else {
            self.player_2.set(coords);
        }

        self.heights[col as usize] += 1;
    }

    pub fn has_won(&self, player_1: bool) -> bool {
        if player_1 {
            self.player_1.has_won()
        } else {
            self.player_2.has_won()
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Connect4BoardMask(u64);

impl Connect4BoardMask {
    #[inline(always)]
    pub fn value(&self) -> u64 {
        self.0
    }

    #[inline(always)]
    pub fn is_set(&self, coords: Connect4Coords) -> bool {
        self.0 & (1 << coords.board_index()) != 0
    }

    #[inline(always)]
    pub fn set(&mut self, coords: Connect4Coords) {
        self.0 |= 1 << coords.board_index();
    }

    #[inline(always)]
    pub fn clear(&mut self, coords: Connect4Coords) {
        self.0 &= !(1 << coords.board_index());
    }

    #[inline(always)]
    pub fn has_won(&self) -> bool {
        let b = self.0;

        let v = b & (b >> 1);
        if v & (v >> 2) != 0 {
            return true;
        }

        let h = b & (b >> 7);
        if h & (h >> 14) != 0 {
            return true;
        }

        let d1 = b & (b >> 6);
        if d1 & (d1 >> 12) != 0 {
            return true;
        }

        let d2 = b & (b >> 8);
        if d2 & (d2 >> 16) != 0 {
            return true;
        }

        false
    }
}

impl Display for Connect4Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+---------------+")?;

        for row in (0..6).rev() {
            write!(f, "|")?;
            for col in 0..7 {
                let coords = Connect4Coords::from_row_col(row, col);
                let symbol = if self.player_1.is_set(coords) {
                    " X"
                } else if self.player_2.is_set(coords) {
                    " O"
                } else {
                    " ·"
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f, " |")?;
        }

        writeln!(f, "+---------------+")?;
        write!(f, "  0 1 2 3 4 5 6  ")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win_check() {
        let mut horizontal = Connect4BoardMask::default();
        horizontal.set(Connect4Coords::from_row_col(0, 0));
        assert!(!horizontal.has_won());
        horizontal.set(Connect4Coords::from_row_col(0, 1));
        assert!(!horizontal.has_won());
        horizontal.set(Connect4Coords::from_row_col(0, 2));
        assert!(!horizontal.has_won());
        horizontal.set(Connect4Coords::from_row_col(0, 3));
        assert!(horizontal.has_won());

        let mut vertical = Connect4BoardMask::default();
        vertical.set(Connect4Coords::from_row_col(0, 0));
        assert!(!vertical.has_won());
        vertical.set(Connect4Coords::from_row_col(1, 0));
        assert!(!vertical.has_won());
        vertical.set(Connect4Coords::from_row_col(2, 0));
        assert!(!vertical.has_won());
        vertical.set(Connect4Coords::from_row_col(3, 0));

        let mut diag_ur = Connect4BoardMask::default();
        diag_ur.set(Connect4Coords::from_row_col(0, 0));
        assert!(!diag_ur.has_won());
        diag_ur.set(Connect4Coords::from_row_col(1, 1));
        assert!(!diag_ur.has_won());
        diag_ur.set(Connect4Coords::from_row_col(2, 2));
        assert!(!diag_ur.has_won());
        diag_ur.set(Connect4Coords::from_row_col(3, 3));
        assert!(diag_ur.has_won());

        let mut diag_ul = Connect4BoardMask::default();
        diag_ul.set(Connect4Coords::from_row_col(0, 6));
        assert!(!diag_ul.has_won());
        diag_ul.set(Connect4Coords::from_row_col(1, 5));
        assert!(!diag_ul.has_won());
        diag_ul.set(Connect4Coords::from_row_col(2, 4));
        assert!(!diag_ul.has_won());
        diag_ul.set(Connect4Coords::from_row_col(3, 3));
        assert!(diag_ul.has_won());
    }
}
