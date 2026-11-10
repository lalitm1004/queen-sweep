use std::{collections::HashSet, marker::PhantomData};

use crate::{
    board::QueensBoard,
    heuristic::QueensHeuristic,
    primitives::ValidBoardSize,
    solver::{QueensSolveResult, QueensSolver},
};

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
            // Safety:
            //  Placements are sourced from get_valid_placements()
            let new_board = unsafe { current_board.place_queen_unchecked(placement) };
            if let Some(solution) = Self::dfs_helper(new_board, seen, num_states_visited) {
                return Some(solution);
            }
        }

        None
    }
}
