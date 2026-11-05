#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    #[inline]
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    #[inline]
    pub const fn to_tuple(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    #[inline]
    pub const fn row(&self) -> usize {
        self.row
    }

    #[inline]
    pub const fn col(&self) -> usize {
        self.col
    }

    #[inline]
    pub const fn is_in_bounds<const N: usize>(&self) -> bool {
        self.row < N && self.col < N
    }

    #[inline]
    pub const fn displace<const N: usize>(self, dr: i32, dc: i32) -> Option<Self> {
        let new_row = self.row as i32 + dr;
        let new_col = self.col as i32 + dc;

        if new_row < 0 || new_col < 0 {
            return None;
        }

        let new_pos = Self::new(new_row as usize, new_col as usize);

        match new_pos.is_in_bounds::<N>() {
            true => Some(new_pos),
            false => None,
        }
    }
}
