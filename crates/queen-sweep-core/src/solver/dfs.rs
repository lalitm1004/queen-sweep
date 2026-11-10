//! A depth-first backtracking [`QueensSolver`].

use std::{collections::HashSet, marker::PhantomData};

use crate::{
    board::QueensBoard,
    heuristic::QueensHeuristic,
    primitives::ValidBoardSize,
    solver::{QueensSolveResult, QueensSolver},
};

/// A depth-first search solver with a configurable heuristic `H`.
///
/// The search explores valid queen placements in depth-first order, using
/// [`QueensHeuristic::order_by_heuristic`] to order the placements tried at
/// each step, and a [`HashSet`] of visited board states to avoid re-exploring
/// equivalent positions.
pub struct DepthFirstSearchSolver<const N: usize, H: QueensHeuristic<N>>
where
    [(); N]: ValidBoardSize,
{
    _heuristic: PhantomData<H>,
}

impl<const N: usize, H: QueensHeuristic<N>> QueensSolver<N, H> for DepthFirstSearchSolver<N, H>
where
    [(); N]: ValidBoardSize,
{
    fn solve(board: QueensBoard<N>) -> super::QueensSolveResult<N> {
        let mut seen = HashSet::new();
        let mut num_states_visited = 0;
        let solved_board = Self::dfs_helper(board, &mut seen, &mut num_states_visited);

        QueensSolveResult {
            solved_board,
            num_states_visited,
        }
    }
}

impl<const N: usize, H: QueensHeuristic<N>> DepthFirstSearchSolver<N, H>
where
    [(); N]: ValidBoardSize,
{
    /// Recursively explores placements from `current_board`, returning the
    /// first solution found (or `None`).
    ///
    /// `seen` deduplicates board states across the whole search, and
    /// `num_states_visited` counts each distinct state examined.
    fn dfs_helper(
        current_board: QueensBoard<N>,
        seen: &mut HashSet<QueensBoard<N>>,
        num_states_visited: &mut usize,
    ) -> Option<QueensBoard<N>> {
        if seen.contains(&current_board) {
            return None;
        }

        *num_states_visited += 1;

        seen.insert(current_board.clone());

        if current_board.is_solved() {
            return Some(current_board);
        }

        let valid_placements = current_board.get_valid_placements();
        let valid_placements = H::order_by_heuristic(valid_placements, &current_board);

        for placement in valid_placements {
            // SAFETY: `placement` is sourced from `get_valid_placements`, which
            // only yields in-bounds, unblocked cells, satisfying the
            // preconditions of `place_queen_unchecked`.
            let new_board = unsafe { current_board.place_queen_unchecked(placement) };
            if let Some(solution) = Self::dfs_helper(new_board, seen, num_states_visited) {
                return Some(solution);
            }
        }

        None
    }
}
