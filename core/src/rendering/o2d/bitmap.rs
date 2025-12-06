pub struct Bitmap<'a> {
    pub data: &'a [u8],
    pub width: usize,
}

impl<'a> Bitmap<'a> {
    pub fn iter_pixels(&self) -> impl Iterator<Item = (isize, isize, bool)> + '_ {
        self.data
            .iter()
            .enumerate()
            .flat_map(move |(row_idx, &row)| {
                (0..self.width).map(move |bit_idx| {
                    let is_set = (row >> bit_idx) & 1 == 1;
                    let x = bit_idx as isize;
                    let y = -(self.height() as isize) + row_idx as isize;
                    (x, y, is_set)
                })
            })
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }
}
