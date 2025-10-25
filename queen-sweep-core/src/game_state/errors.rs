#[derive(Debug)]
pub enum GameStateError {
    InvalidBoardSize(usize),

    NonSquareBoard { rows: usize, cols: usize },

    ColorsNotStartingFromZero { first_color: u8 },

    NonContinuousColors { expected: u8, found: u8 },

    InvalidCellCount { expected: usize, found: usize },

    BoardTooLarge { size: usize, max_size: usize },
}

impl std::fmt::Display for GameStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidBoardSize(size) => {
                write!(f, "Invalid board size: {} (must be greater than 0)", size)
            }
            Self::NonSquareBoard { rows, cols } => {
                write!(f, "Board is not square: {} rows × {} columns", rows, cols)
            }
            Self::ColorsNotStartingFromZero { first_color } => {
                write!(
                    f,
                    "Colors must start from 0, but first color is {}",
                    first_color
                )
            }
            Self::NonContinuousColors { expected, found } => {
                write!(
                    f,
                    "Colors are not continuous: expected {}, found {}",
                    expected, found
                )
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
