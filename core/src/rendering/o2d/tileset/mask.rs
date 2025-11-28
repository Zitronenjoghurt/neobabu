#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum TileMaskLayout {
    Blob,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
/// Bitmask for cardinal directions
pub struct TileMask(u8);

impl TileMask {
    pub fn new(n: bool, e: bool, s: bool, w: bool, ne: bool, se: bool, sw: bool, nw: bool) -> Self {
        let mask = (n as u8)
            | (e as u8) << 1
            | (s as u8) << 2
            | (w as u8) << 3
            | (ne as u8) << 4
            | (se as u8) << 5
            | (sw as u8) << 6
            | (nw as u8) << 7;
        Self(mask)
    }
}

impl TileMask {
    pub fn tile_position(&self, layout: TileMaskLayout) -> (u8, u8) {
        match layout {
            TileMaskLayout::Blob => self.blob_position(),
        }
    }

    fn blob_position(&self) -> (u8, u8) {
        let n = self.0 & 0b0001 != 0;
        let e = self.0 & 0b0010 != 0;
        let s = self.0 & 0b0100 != 0;
        let w = self.0 & 0b1000 != 0;

        let ne = self.0 & 0b0001_0000 != 0 && n && e;
        let se = self.0 & 0b0010_0000 != 0 && s && e;
        let sw = self.0 & 0b0100_0000 != 0 && s && w;
        let nw = self.0 & 0b1000_0000 != 0 && n && w;

        match (n, e, s, w) {
            // Only center
            (false, false, false, false) => (0, 3),
            // Single edge
            (false, false, false, true) => (3, 3),
            (false, false, true, false) => (0, 0),
            (false, true, false, false) => (1, 3),
            (true, false, false, false) => (0, 2),
            // North-South
            (true, false, true, false) => (0, 1),
            // East-West
            (false, true, false, true) => (2, 3),
            // North-East
            (true, true, false, false) => {
                if ne {
                    (8, 3)
                } else {
                    (1, 2)
                }
            }
            // South-East
            (false, true, true, false) => {
                if se {
                    (8, 0)
                } else {
                    (1, 0)
                }
            }
            // South-West
            (false, false, true, true) => {
                if sw {
                    (11, 0)
                } else {
                    (3, 0)
                }
            }
            // North-West
            (true, false, false, true) => {
                if nw {
                    (11, 3)
                } else {
                    (3, 2)
                }
            }
            // North-East-South
            (true, true, true, false) => {
                if ne && se {
                    (8, 1)
                } else if ne {
                    (4, 2)
                } else if se {
                    (4, 1)
                } else {
                    (1, 1)
                }
            }
            // East-South-West
            (false, true, true, true) => {
                if se && sw {
                    (10, 0)
                } else if se {
                    (5, 0)
                } else if sw {
                    (6, 0)
                } else {
                    (2, 0)
                }
            }
            // North-West-South
            (true, false, true, true) => {
                if nw && sw {
                    (11, 2)
                } else if nw {
                    (7, 2)
                } else if sw {
                    (7, 1)
                } else {
                    (3, 1)
                }
            }
            // North-East-West
            (true, true, false, true) => {
                if nw && ne {
                    (9, 3)
                } else if nw {
                    (6, 3)
                } else if ne {
                    (5, 3)
                } else {
                    (2, 2)
                }
            }
            // All edges
            (true, true, true, true) => match (ne, se, sw, nw) {
                (false, false, false, false) => (2, 1),
                (false, false, false, true) => (4, 0),
                (false, false, true, false) => (4, 3),
                (false, false, true, true) => (11, 1),
                (false, true, false, false) => (7, 3),
                (false, true, false, true) => (10, 2),
                (false, true, true, false) => (9, 0),
                (false, true, true, true) => (6, 1),
                (true, false, false, false) => (7, 0),
                (true, false, false, true) => (10, 3),
                (true, false, true, false) => (9, 1),
                (true, false, true, true) => (6, 2),
                (true, true, false, false) => (8, 2),
                (true, true, false, true) => (5, 2),
                (true, true, true, false) => (5, 1),
                (true, true, true, true) => (9, 2),
            },
        }
    }
}
