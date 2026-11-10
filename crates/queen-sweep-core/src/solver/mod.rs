//! Backtracking solvers for the Queens puzzle.
//!
//! The [`QueensSolver`] trait abstracts over solving strategies, and
//! [`DepthFirstSearchSolver`] provides a depth-first backtracking
//! implementation.

use crate::{board::QueensBoard, heuristic::QueensHeuristic, primitives::ValidBoardSize};

mod dfs;
pub use dfs::DepthFirstSearchSolver;

/// The outcome of a solve attempt.
pub struct QueensSolveResult<const N: usize>
where
    [(); N]: ValidBoardSize,
{
    /// The solved board, or `None` if no solution exists.
    pub solved_board: Option<QueensBoard<N>>,
    /// The number of board states visited during the search.
    pub num_states_visited: usize,
}

/// A strategy for solving a Queens board.
///
/// `H` is the move-ordering [`QueensHeuristic`] used to prioritize placements
/// during search.
pub trait QueensSolver<const N: usize, H>
where
    [(); N]: ValidBoardSize,
    H: QueensHeuristic<N>,
{
    /// Attempts to solve `board`, returning the solution (if any) and the
    /// number of states visited.
    fn solve(board: QueensBoard<N>) -> QueensSolveResult<N>;
}
