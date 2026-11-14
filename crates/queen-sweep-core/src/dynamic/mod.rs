//! Runtime, type-erased solver interface.
//!
//! The board and solver in this crate are generic over a compile-time size `N`
//! and heuristic `H`. This module bridges that to runtime inputs: it accepts
//! color regions whose cells are `Color` or anything `Into<Color>` (for example
//! `usize`), validates them, infers the board size, dispatches to the
//! appropriate monomorphization, and returns a single [`DynamicSolver`] object
//! whose [`solve`](DynamicSolver::solve) method can be called repeatedly.

mod error;
mod input;
mod options;
mod result;
mod solver;

pub use error::DynamicSolverError;
pub use input::RegionInput;
pub use options::{HeuristicOption, SolverOption};
pub use result::DynamicSolveResult;
pub use solver::{DynamicSolver, QueensSolverConstructor};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{board::BoardInitError, primitives::Color};

    fn quadrant() -> Vec<Vec<usize>> {
        vec![
            vec![0, 0, 1, 1],
            vec![0, 0, 1, 1],
            vec![2, 2, 3, 3],
            vec![2, 2, 3, 3],
        ]
    }

    fn quadrant_colors() -> Vec<Vec<Color>> {
        quadrant()
            .into_iter()
            .map(|row| row.into_iter().map(Color::from).collect())
            .collect()
    }

    fn unsolvable() -> Vec<Vec<usize>> {
        vec![
            vec![0, 0, 1, 1],
            vec![2, 2, 2, 2],
            vec![3, 3, 3, 3],
            vec![3, 3, 3, 3],
        ]
    }

    fn build_grid(regions: Vec<Vec<usize>>) -> DynamicSolver {
        QueensSolverConstructor::build_grid(
            regions,
            SolverOption::DFSSolver,
            HeuristicOption::NoOpHeuristic,
        )
        .unwrap()
    }

    #[test]
    fn solves_quadrant_from_grid() {
        let result = build_grid(quadrant()).solve();

        assert!(result.solved_board.is_some());
        assert_eq!(result.size, 4);
        assert!(result.num_states_visited > 0);
    }

    #[test]
    fn solves_quadrant_from_grid_of_colors() {
        let solver = QueensSolverConstructor::build_grid(
            quadrant_colors(),
            SolverOption::DFSSolver,
            HeuristicOption::NoOpHeuristic,
        )
        .unwrap();

        assert!(solver.solve().solved_board.is_some());
    }

    #[test]
    fn solves_quadrant_from_flat() {
        let flat = vec![0, 0, 1, 1, 0, 0, 1, 1, 2, 2, 3, 3, 2, 2, 3, 3];
        let solver = QueensSolverConstructor::build_flat(
            flat,
            SolverOption::DFSSolver,
            HeuristicOption::SmallestRegionFirstHeuristic,
        )
        .unwrap();

        assert!(solver.solve().solved_board.is_some());
    }

    #[test]
    fn solves_quadrant_from_flat_of_colors() {
        let flat: Vec<Color> = vec![0, 0, 1, 1, 0, 0, 1, 1, 2, 2, 3, 3, 2, 2, 3, 3]
            .into_iter()
            .map(Color::from)
            .collect();

        let solver = QueensSolverConstructor::build_flat(
            flat,
            SolverOption::DFSSolver,
            HeuristicOption::NoOpHeuristic,
        )
        .unwrap();

        assert!(solver.solve().solved_board.is_some());
    }

    #[test]
    fn unsolvable_returns_none() {
        assert!(build_grid(unsolvable()).solve().solved_board.is_none());
    }

    #[test]
    fn solve_is_repeatable_and_deterministic() {
        let solver = build_grid(quadrant());
        let first = solver.solve();
        let second = solver.solve();

        assert_eq!(first, second);
    }

    #[test]
    fn rejects_empty_grid() {
        let err = QueensSolverConstructor::build_grid(
            Vec::<Vec<usize>>::new(),
            SolverOption::DFSSolver,
            HeuristicOption::NoOpHeuristic,
        )
        .unwrap_err();

        assert!(matches!(err, DynamicSolverError::Empty));
    }

    #[test]
    fn rejects_non_square_grid() {
        let regions = vec![vec![0, 0, 1], vec![1, 1, 0]];

        let err = QueensSolverConstructor::build_grid(
            regions,
            SolverOption::DFSSolver,
            HeuristicOption::NoOpHeuristic,
        )
        .unwrap_err();

        assert!(matches!(err, DynamicSolverError::NonSquare { .. }));
    }

    #[test]
    fn rejects_unsupported_size() {
        let regions = vec![vec![0, 1], vec![1, 0]];

        let err = QueensSolverConstructor::build_grid(
            regions,
            SolverOption::DFSSolver,
            HeuristicOption::NoOpHeuristic,
        )
        .unwrap_err();

        assert!(matches!(err, DynamicSolverError::UnsupportedSize(2)));
    }

    #[test]
    fn rejects_color_out_of_range() {
        let mut regions = quadrant();
        regions[3][3] = 4;

        let err = QueensSolverConstructor::build_grid(
            regions,
            SolverOption::DFSSolver,
            HeuristicOption::NoOpHeuristic,
        )
        .unwrap_err();

        assert!(matches!(
            err,
            DynamicSolverError::BoardInit(BoardInitError::ColorOutOfRange { color: 4, .. })
        ));
    }

    #[test]
    fn rejects_missing_color() {
        let regions = vec![
            vec![0, 0, 1, 1],
            vec![0, 0, 1, 1],
            vec![2, 2, 2, 2],
            vec![2, 2, 2, 2],
        ];

        let err = QueensSolverConstructor::build_grid(
            regions,
            SolverOption::DFSSolver,
            HeuristicOption::NoOpHeuristic,
        )
        .unwrap_err();

        assert!(matches!(
            err,
            DynamicSolverError::BoardInit(BoardInitError::InvalidColorCount {
                expected: 4,
                found: 3,
            })
        ));
    }

    #[test]
    fn rejects_flat_not_perfect_square() {
        let flat = vec![0, 1, 2];

        let err = QueensSolverConstructor::build_flat(
            flat,
            SolverOption::DFSSolver,
            HeuristicOption::NoOpHeuristic,
        )
        .unwrap_err();

        assert!(matches!(err, DynamicSolverError::NotPerfectSquare(3)));
    }
}
