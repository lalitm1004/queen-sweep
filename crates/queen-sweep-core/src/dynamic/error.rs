//! Errors produced while building a dynamic solver.

use crate::board::BoardInitError;

/// Errors produced while building a [`DynamicSolver`](crate::dynamic::DynamicSolver).
#[derive(Debug, thiserror::Error)]
pub enum DynamicSolverError {
    /// The supplied color regions were empty.
    #[error("color regions are empty")]
    Empty,

    /// The grid is not square.
    #[error("color regions are not square: {rows} rows but a row has {cols} columns")]
    NonSquare { rows: usize, cols: usize },

    /// The flat list length is not a perfect square.
    #[error("flat color list has length {0}, which is not a perfect square")]
    NotPerfectSquare(usize),

    /// The board size is not supported.
    #[error("board size {0} is not supported (supported sizes: 1 and 4..=16)")]
    UnsupportedSize(usize),

    /// The color regions failed board initialization (a color index out of
    /// range, or the wrong number of distinct colors).
    #[error(transparent)]
    BoardInit(#[from] BoardInitError),
}
