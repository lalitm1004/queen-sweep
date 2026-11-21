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

    #[test]
    fn supports_eighteen_by_eighteen_boards() {
        let n = 18;
        // Column-stripe layout: each column is its own color.
        let flat: Vec<usize> = (0..n).flat_map(|_| 0..n).collect();

        QueensSolverConstructor::build_flat(
            flat,
            SolverOption::DFSSolver,
            HeuristicOption::SmallestRegionFirstHeuristic,
        )
        .expect("18x18 board should build");
    }

    #[test]
    fn builds_every_supported_size() {
        for n in [1usize].into_iter().chain(4..=18) {
            // Column-stripe layout: each column is its own color, yielding exactly
            // `n` distinct colors as required.
            let flat: Vec<usize> = (0..n).flat_map(|_| 0..n).collect();

            QueensSolverConstructor::build_flat(
                flat,
                SolverOption::DFSSolver,
                HeuristicOption::NoOpHeuristic,
            )
            .unwrap_or_else(|e| panic!("size {n} should build: {e:?}"));
        }
    }

    #[test]
    fn builds_from_region_input_enum_variants() {
        let grid = QueensSolverConstructor::build(
            RegionInput::Grid(quadrant()),
            SolverOption::DFSSolver,
            HeuristicOption::NoOpHeuristic,
        )
        .unwrap();
        assert!(grid.solve().solved_board.is_some());

        let flat = vec![0, 0, 1, 1, 0, 0, 1, 1, 2, 2, 3, 3, 2, 2, 3, 3];
        let flat_solver = QueensSolverConstructor::build(
            RegionInput::Flat(flat),
            SolverOption::DFSSolver,
            HeuristicOption::NoOpHeuristic,
        )
        .unwrap();
        assert!(flat_solver.solve().solved_board.is_some());
    }

    #[test]
    fn non_square_reports_row_and_col_counts() {
        // 3 rows, but the last row has only 2 columns.
        let regions = vec![vec![0, 0, 1], vec![1, 1, 0], vec![2, 2]];

        let err = QueensSolverConstructor::build_grid(
            regions,
            SolverOption::DFSSolver,
            HeuristicOption::NoOpHeuristic,
        )
        .unwrap_err();

        assert!(matches!(
            err,
            DynamicSolverError::NonSquare { rows: 3, cols: 2 }
        ));
    }

    #[test]
    fn rejects_size_three() {
        let flat = vec![0usize; 9];
        let err = QueensSolverConstructor::build_flat(
            flat,
            SolverOption::DFSSolver,
            HeuristicOption::NoOpHeuristic,
        )
        .unwrap_err();
        assert!(matches!(err, DynamicSolverError::UnsupportedSize(3)));
    }

    #[test]
    fn rejects_size_nineteen() {
        let flat = vec![0usize; 19 * 19];
        let err = QueensSolverConstructor::build_flat(
            flat,
            SolverOption::DFSSolver,
            HeuristicOption::NoOpHeuristic,
        )
        .unwrap_err();
        assert!(matches!(err, DynamicSolverError::UnsupportedSize(19)));
    }

    #[test]
    fn solve_result_reports_size_and_valid_board() {
        let result = build_grid(quadrant()).solve();

        assert_eq!(result.size, 4);
        assert!(result.num_states_visited > 0);

        let board = result.solved_board.expect("quadrant should solve");
        assert_eq!(board.len(), 4);
        assert!(board.iter().all(|row| row.len() == 4));

        let queen_count: usize = board
            .iter()
            .flatten()
            .filter(|&&cell| cell == crate::primitives::Cell::Queen)
            .count();
        assert_eq!(queen_count, 4);

        // Exactly one queen per row.
        for row in &board {
            assert_eq!(
                row.iter()
                    .filter(|&&cell| cell == crate::primitives::Cell::Queen)
                    .count(),
                1
            );
        }
    }

    #[test]
    fn board_init_error_displays_underlying_message() {
        // `#[error(transparent)]` must forward Display to the wrapped error.
        let err = DynamicSolverError::BoardInit(BoardInitError::InvalidColorCount {
            expected: 4,
            found: 1,
        });

        assert_eq!(
            err.to_string(),
            "Invalid number of distinct colors. Expected 4, found 1."
        );
    }

    #[test]
    fn board_init_error_converts_via_from() {
        let inner = BoardInitError::ColorOutOfRange {
            color: 9,
            max_colors: 4,
        };

        let err: DynamicSolverError = inner.into();

        assert!(matches!(
            err,
            DynamicSolverError::BoardInit(BoardInitError::ColorOutOfRange {
                color: 9,
                max_colors: 4
            })
        ));
    }

    #[test]
    fn grid_and_flat_produce_identical_solutions() {
        let grid_solver = QueensSolverConstructor::build_grid(
            quadrant(),
            SolverOption::DFSSolver,
            HeuristicOption::SmallestRegionFirstHeuristic,
        )
        .unwrap();

        let flat = vec![0, 0, 1, 1, 0, 0, 1, 1, 2, 2, 3, 3, 2, 2, 3, 3];
        let flat_solver = QueensSolverConstructor::build_flat(
            flat,
            SolverOption::DFSSolver,
            HeuristicOption::SmallestRegionFirstHeuristic,
        )
        .unwrap();

        assert_eq!(grid_solver.solve(), flat_solver.solve());
    }
}
