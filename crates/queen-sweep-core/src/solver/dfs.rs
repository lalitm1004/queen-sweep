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

#[cfg(test)]
mod tests {
    use super::DepthFirstSearchSolver;
    use crate::{
        board::QueensBoard,
        heuristic::{NoOpHeuristic, SmallestRegionFirstHeuristic},
        primitives::{Color, Position, ValidBoardSize},
        solver::QueensSolver,
    };

    fn quadrant_regions() -> [[Color; 4]; 4] {
        [
            [0.into(), 0.into(), 1.into(), 1.into()],
            [0.into(), 0.into(), 1.into(), 1.into()],
            [2.into(), 2.into(), 3.into(), 3.into()],
            [2.into(), 2.into(), 3.into(), 3.into()],
        ]
    }

    fn quadrant_board() -> QueensBoard<4> {
        unsafe { QueensBoard::new_unchecked(quadrant_regions()) }
    }

    /// Independently verifies a board is a valid Queens solution.
    fn assert_valid_solution<const N: usize>(board: &QueensBoard<N>)
    where
        [(); N]: ValidBoardSize,
    {
        assert!(board.is_solved());
        let queens = board.get_queen_col_per_row();
        let regions = board.get_color_regions();

        let mut cols = vec![false; N];
        let mut colors = vec![false; N];
        let mut positions = Vec::with_capacity(N);

        for (row, &qcol) in queens.iter().enumerate() {
            let col = qcol.expect("queen in every row");
            assert!(!cols[col], "two queens in column {col}");
            cols[col] = true;

            let color = usize::from(regions[row][col]);
            assert!(!colors[color], "two queens in color {color}");
            colors[color] = true;

            positions.push((row, col));
        }

        assert!(cols.iter().all(|&c| c) && colors.iter().all(|&c| c));

        for i in 0..N {
            for j in (i + 1)..N {
                let (r1, c1) = positions[i];
                let (r2, c2) = positions[j];
                let dr = (r1 as isize - r2 as isize).unsigned_abs();
                let dc = (c1 as isize - c2 as isize).unsigned_abs();
                assert!(
                    dr > 1 || dc > 1,
                    "adjacent queens at ({r1},{c1}) ({r2},{c2})"
                );
            }
        }
    }

    /// The quadrant board forces backtracking: the first row-major placement
    /// `(0, 0)` is a dead end. Both heuristics must still recover a solution.
    #[test]
    fn both_heuristics_solve_board_requiring_backtracking() {
        let no_op = DepthFirstSearchSolver::<4, NoOpHeuristic>::solve(quadrant_board());
        let smallest =
            DepthFirstSearchSolver::<4, SmallestRegionFirstHeuristic>::solve(quadrant_board());

        let no_op_solved = no_op.solved_board.expect("NoOp should solve the board");
        let smallest_solved = smallest
            .solved_board
            .expect("SmallestRegionFirst should solve");

        assert_valid_solution(&no_op_solved);
        assert_valid_solution(&smallest_solved);
    }

    #[test]
    fn already_solved_board_is_returned_immediately() {
        // (0,1), (1,3), (2,0), (3,2) is a known solution of the quadrant layout.
        let solved = quadrant_board()
            .place_queen(Position::new(0, 1))
            .unwrap()
            .place_queen(Position::new(1, 3))
            .unwrap()
            .place_queen(Position::new(2, 0))
            .unwrap()
            .place_queen(Position::new(3, 2))
            .unwrap();

        assert!(solved.is_solved());

        let result = DepthFirstSearchSolver::<4, NoOpHeuristic>::solve(solved.clone());

        // Early return on a goal state means exactly one state is visited.
        assert_eq!(result.num_states_visited, 1);
        assert_eq!(result.solved_board, Some(solved));
    }

    #[test]
    fn unsolvable_board_visits_states_then_returns_none() {
        let regions: [[Color; 4]; 4] = [
            [0.into(), 0.into(), 1.into(), 1.into()],
            [2.into(), 2.into(), 2.into(), 2.into()],
            [3.into(), 3.into(), 3.into(), 3.into()],
            [3.into(), 3.into(), 3.into(), 3.into()],
        ];
        let board = unsafe { QueensBoard::new_unchecked(regions) };

        let result = DepthFirstSearchSolver::<4, NoOpHeuristic>::solve(board);

        assert!(result.solved_board.is_none());
        assert!(result.num_states_visited > 0);
    }

    #[test]
    fn num_states_visited_is_deterministic_across_runs() {
        let run = || DepthFirstSearchSolver::<4, NoOpHeuristic>::solve(quadrant_board());

        let a = run();
        let b = run();

        assert_eq!(a.num_states_visited, b.num_states_visited);
        assert_eq!(a.solved_board, b.solved_board);
    }

    /// A solved board's state count must never exceed the total number of
    /// distinct partial placements representable on an `N=4` board.
    #[test]
    fn num_states_visited_is_bounded() {
        let result = DepthFirstSearchSolver::<4, NoOpHeuristic>::solve(quadrant_board());

        // (columns per row: 0..=3 queen, or None) = 5 choices per row => 5^4.
        assert!(result.num_states_visited > 0);
        assert!(result.num_states_visited <= 5usize.pow(4));
    }
}
