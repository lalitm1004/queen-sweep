#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    Empty = 0,
    Blocked = 1,
    Queen = 2,
}

impl Cell {
    #[inline]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }

    #[inline]
    pub const fn is_blocked(&self) -> bool {
        matches!(self, Cell::Blocked)
    }

    #[inline]
    pub const fn is_queen(&self) -> bool {
        matches!(self, Cell::Queen)
    }
}
