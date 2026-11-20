/// The state of a single cell on a [`QueensBoard`](crate::board::QueensBoard).
///
/// A cell starts as [`Cell::Empty`] and, as queens are placed and blocking is
/// applied, transitions to either [`Cell::Blocked`] or [`Cell::Queen`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    /// A cell with no queen and no blocking applied; still eligible for a queen.
    Empty = 0,
    /// A cell that may not receive a queen (blocked by row, column, neighbor,
    /// or color-region constraints).
    Blocked = 1,
    /// A cell that contains a placed queen.
    Queen = 2,
}

impl Cell {
    /// Returns `true` if the cell is [`Cell::Empty`].
    #[inline]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }

    /// Returns `true` if the cell is [`Cell::Blocked`].
    #[inline]
    pub const fn is_blocked(&self) -> bool {
        matches!(self, Cell::Blocked)
    }

    /// Returns `true` if the cell is [`Cell::Queen`].
    #[inline]
    pub const fn is_queen(&self) -> bool {
        matches!(self, Cell::Queen)
    }
}
