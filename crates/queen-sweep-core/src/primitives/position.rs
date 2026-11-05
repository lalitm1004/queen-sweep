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

#[cfg(test)]
mod tests {
    use super::Position;

    #[test]
    fn test_position_is_in_bounds() {
        assert!(Position::new(0, 0).is_in_bounds::<8>());
        assert!(Position::new(7, 7).is_in_bounds::<8>());
        assert!(Position::new(3, 5).is_in_bounds::<8>());

        assert!(!Position::new(8, 0).is_in_bounds::<8>());
        assert!(!Position::new(0, 8).is_in_bounds::<8>());
        assert!(!Position::new(8, 8).is_in_bounds::<8>());
    }

    #[test]
    fn test_position_is_in_bounds_zero_size() {
        assert!(!Position::new(0, 0).is_in_bounds::<0>());
    }

    #[test]
    fn test_position_displace_positive() {
        let position = Position::new(2, 3);

        assert_eq!(position.displace::<8>(1, 2), Some(Position::new(3, 5)));
    }

    #[test]
    fn test_position_displace_negative() {
        let position = Position::new(5, 5);

        assert_eq!(position.displace::<8>(-2, -3), Some(Position::new(3, 2)));
    }

    #[test]
    fn test_position_displace_mixed() {
        let position = Position::new(3, 3);

        assert_eq!(position.displace::<8>(2, -1), Some(Position::new(5, 2)));

        assert_eq!(position.displace::<8>(-1, 2), Some(Position::new(2, 5)));
    }

    #[test]
    fn test_position_displace_zero() {
        let position = Position::new(3, 4);

        assert_eq!(position.displace::<8>(0, 0), Some(position));
    }

    #[test]
    fn test_position_displace_out_of_bounds() {
        let position = Position::new(7, 7);

        assert_eq!(position.displace::<8>(1, 0), None);
        assert_eq!(position.displace::<8>(0, 1), None);
        assert_eq!(position.displace::<8>(1, 1), None);
    }

    #[test]
    fn test_position_displace_negative_out_of_bounds() {
        let position = Position::new(0, 0);

        assert_eq!(position.displace::<8>(-1, 0), None);
        assert_eq!(position.displace::<8>(0, -1), None);
        assert_eq!(position.displace::<8>(-1, -1), None);
    }

    #[test]
    fn test_position_displace_large_offset() {
        let position = Position::new(2, 2);

        assert_eq!(position.displace::<8>(100, 0), None);
        assert_eq!(position.displace::<8>(0, 100), None);
        assert_eq!(position.displace::<8>(-100, 0), None);
        assert_eq!(position.displace::<8>(0, -100), None);
    }

    #[test]
    fn test_position_displace_zero_size() {
        let position = Position::new(0, 0);

        assert_eq!(position.displace::<0>(0, 0), None);
    }
}
