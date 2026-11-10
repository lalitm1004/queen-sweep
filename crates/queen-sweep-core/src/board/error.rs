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
