//! The result of a dynamic solve.

use crate::{
    primitives::{Cell, ValidBoardSize},
    solver::QueensSolveResult,
};

/// The result of a dynamic solve.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicSolveResult {
    /// The board size `N`.
    pub size: usize,

    /// The solved board as a grid of [`Cell`] values, or `None` if no solution
    /// exists.
    pub solved_board: Option<Vec<Vec<Cell>>>,

    /// The number of board states visited during the search.
    pub num_states_visited: usize,
}

impl DynamicSolveResult {
    /// Builds a dynamic result from a const-generic [`QueensSolveResult`],
    /// converting the fixed-size solution grid into a `Vec<Vec<Cell>>`.
    pub(crate) fn from_const<const N: usize>(result: QueensSolveResult<N>) -> Self
    where
        [(); N]: ValidBoardSize,
    {
        Self {
            size: N,
            solved_board: result.solved_board.map(|board| {
                board
                    .get_cells()
                    .into_iter()
                    .map(|row| row.into_iter().collect())
                    .collect()
            }),
            num_states_visited: result.num_states_visited,
        }
    }
}
