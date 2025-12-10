#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Connect4Coords(u8);

impl Connect4Coords {
    const STRIDE: u8 = 7;

    #[inline(always)]
    pub fn from_row_col(row: u8, col: u8) -> Self {
        Self(col * Self::STRIDE + row)
    }

    #[inline(always)]
    pub fn from_row(row: u8) -> Self {
        Self(row)
    }

    #[inline(always)]
    pub fn with_row(self, row: u8) -> Self {
        let col_part = self.0 / Self::STRIDE;
        Self(col_part * Self::STRIDE + row)
    }

    #[inline(always)]
    pub fn from_col(col: u8) -> Self {
        Self(col * Self::STRIDE)
    }

    #[inline(always)]
    pub fn with_col(self, col: u8) -> Self {
        let row_part = self.0 % Self::STRIDE;
        Self(col * Self::STRIDE + row_part)
    }

    #[inline(always)]
    pub fn row(&self) -> u8 {
        self.0 % Self::STRIDE
    }

    #[inline(always)]
    pub fn col(&self) -> u8 {
        self.0 / Self::STRIDE
    }

    #[inline(always)]
    pub fn board_index(&self) -> usize {
        self.0 as usize
    }

    pub fn iter_all() -> impl Iterator<Item = Self> {
        (0..6).flat_map(move |row| (0..7).map(move |col| Self::from_row_col(row, col)))
    }
}
