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

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn test_cell_is_empty() {
        assert!(Cell::Empty.is_empty());
        assert!(!Cell::Blocked.is_empty());
        assert!(!Cell::Queen.is_empty());
    }

    #[test]
    fn test_cell_is_blocked() {
        assert!(Cell::Blocked.is_blocked());
        assert!(!Cell::Empty.is_blocked());
        assert!(!Cell::Queen.is_blocked());
    }

    #[test]
    fn test_cell_is_queen() {
        assert!(Cell::Queen.is_queen());
        assert!(!Cell::Empty.is_queen());
        assert!(!Cell::Blocked.is_queen());
    }
}
