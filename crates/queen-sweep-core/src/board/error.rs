#[derive(Debug, thiserror::Error)]
pub enum QueensPlacementError {
    #[error("Target cell at ({r}, {c}) is not empty.")]
    NonEmptyPlacementTarget { r: usize, c: usize },

    #[error("Target coordinates ({r}, {c}) are out of bounds.")]
    OutOfBounds { r: usize, c: usize },
}
