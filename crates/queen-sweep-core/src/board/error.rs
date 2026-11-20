/// Errors returned when a queen cannot be placed at a requested position.
///
/// This type is produced by [`QueensBoard::place_queen`](crate::board::QueensBoard::place_queen)
/// and implements [`std::error::Error`] via [`thiserror`].
#[derive(Debug, thiserror::Error)]
pub enum QueensPlacementError {
    /// The target cell is already blocked and cannot receive a queen.
    #[error("Target cell at ({r}, {c}) is not empty.")]
    NonEmptyPlacementTarget { r: usize, c: usize },

    /// The target coordinates lie outside the board.
    #[error("Target coordinates ({r}, {c}) are out of bounds.")]
    OutOfBounds { r: usize, c: usize },
}

/// Errors returned when a board cannot be initialized with the given color regions.
///
/// This type is produced by [`QueensBoard::new`](crate::board::QueensBoard::new)
/// and implements [`std::error::Error`] via [`thiserror`].
#[derive(Debug, thiserror::Error)]
pub enum BoardInitError {
    /// A color index is greater than or equal to N.
    #[error("Color index {color} is out of bounds. Expected a value in 0..{max_colors}.")]
    ColorOutOfRange { color: usize, max_colors: usize },

    /// The color regions do not contain exactly N distinct colors.
    #[error("Invalid number of distinct colors. Expected {expected}, found {found}.")]
    InvalidColorCount { expected: usize, found: usize },
}

#[cfg(test)]
mod tests {
    use super::{BoardInitError, QueensPlacementError};

    #[test]
    fn placement_error_messages() {
        assert_eq!(
            QueensPlacementError::NonEmptyPlacementTarget { r: 1, c: 2 }.to_string(),
            "Target cell at (1, 2) is not empty."
        );
        assert_eq!(
            QueensPlacementError::OutOfBounds { r: 4, c: 0 }.to_string(),
            "Target coordinates (4, 0) are out of bounds."
        );
    }

    #[test]
    fn board_init_error_messages() {
        assert_eq!(
            BoardInitError::ColorOutOfRange {
                color: 9,
                max_colors: 4
            }
            .to_string(),
            "Color index 9 is out of bounds. Expected a value in 0..4."
        );
        assert_eq!(
            BoardInitError::InvalidColorCount {
                expected: 4,
                found: 3
            }
            .to_string(),
            "Invalid number of distinct colors. Expected 4, found 3."
        );
    }

    #[test]
    fn errors_are_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<QueensPlacementError>();
        assert_send_sync::<BoardInitError>();
    }
}
