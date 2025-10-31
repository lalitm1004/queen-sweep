#[derive(Debug)]
pub enum GameStateError {
    InexistentBoard,

    NonSquareBoard { rows: usize, cols: usize },

    InvalidCellCount { expected: usize, found: usize },

    BoardTooLarge { size: usize, max_size: usize },
}

impl std::fmt::Display for GameStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InexistentBoard => {
                write!(f, "Board size must be greater than 0")
            }
            Self::NonSquareBoard { rows, cols } => {
                write!(f, "Board is not square: {} rows x {} columns", rows, cols)
            }
            Self::InvalidCellCount { expected, found } => {
                write!(
                    f,
                    "Invalid cell count: expected {}, found {}",
                    expected, found
                )
            }
            Self::BoardTooLarge { size, max_size } => {
                write!(
                    f,
                    "Board size {} exceeds maximum supported size {}",
                    size, max_size
                )
            }
        }
    }
}
