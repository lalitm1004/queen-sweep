//! End-to-end tests for `DepthFirstSearchSolver`, exercising the solver
//! strictly through the public API.
//!
//! These tests verify the two properties that actually matter:
//!
//! - A solvable board yields *some* board that is a genuinely valid solution,
//!   independently re-checked from first principles (one queen per row, column,
//!   and color region, with no two queens 8-adjacent).
//! - An unsolvable board yields `None`, and the solver terminates.

use queen_sweep_core::{
    board::QueensBoard,
    dynamic::{HeuristicOption, QueensSolverConstructor, SolverOption},
    heuristic::{NoOpHeuristic, SmallestRegionFirstHeuristic},
    primitives::{Color, ValidBoardSize},
    solver::{DepthFirstSearchSolver, QueensSolveResult, QueensSolver},
};

use proptest::prelude::*;

fn solve<const N: usize>(board: QueensBoard<N>) -> QueensSolveResult<N>
where
    [(); N]: ValidBoardSize,
{
    DepthFirstSearchSolver::<N, NoOpHeuristic>::solve(board)
}

/// The N=4 quadrant layout: each color is a 2x2 quadrant.
fn quadrant_regions() -> [[Color; 4]; 4] {
    [
        [0.into(), 0.into(), 1.into(), 1.into()],
        [0.into(), 0.into(), 1.into(), 1.into()],
        [2.into(), 2.into(), 3.into(), 3.into()],
        [2.into(), 2.into(), 3.into(), 3.into()],
    ]
}

/// Each column is its own color. Solvable for any `N` via a column permutation
/// with no two consecutive columns within distance 1.
fn column_stripe_regions<const N: usize>() -> [[Color; N]; N] {
    std::array::from_fn(|_row| std::array::from_fn(|col| Color::from(col)))
}

/// Unsolvable: colors 0 and 1 both exist only in row 0, which can hold at most
/// one queen.
fn unsolvable_regions() -> [[Color; 4]; 4] {
    [
        [0.into(), 0.into(), 1.into(), 1.into()],
        [2.into(), 2.into(), 2.into(), 2.into()],
        [3.into(), 3.into(), 3.into(), 3.into()],
        [3.into(), 3.into(), 3.into(), 3.into()],
    ]
}

/// Verifies, independently of the solver's own bookkeeping, that `board` is a
/// valid Queens solution: exactly one queen per row, column, and color region,
/// and no two queens are 8-adjacent (including diagonals).
fn assert_valid_solution<const N: usize>(board: &QueensBoard<N>)
where
    [(); N]: ValidBoardSize,
{
    assert!(board.is_solved(), "solver returned an unsolved board");

    let queens = board.get_queen_col_per_row();
    let regions = board.get_color_regions();

    let mut seen_cols = vec![false; N];
    let mut seen_colors = vec![false; N];
    let mut positions = Vec::with_capacity(N);

    for (row, &queen_col) in queens.iter().enumerate() {
        let col = queen_col.expect("every row must contain a queen");
        assert!(col < N, "queen column {col} is out of bounds");

        assert!(!seen_cols[col], "two queens in column {col}");
        seen_cols[col] = true;

        let color = usize::from(regions[row][col]);
        assert!(!seen_colors[color], "two queens in color {color}");
        seen_colors[color] = true;

        positions.push((row, col));
    }

    assert!(seen_cols.iter().all(|&x| x), "not every column has a queen");
    assert!(
        seen_colors.iter().all(|&x| x),
        "not every color has a queen"
    );

    for i in 0..N {
        for j in (i + 1)..N {
            let (r1, c1) = positions[i];
            let (r2, c2) = positions[j];
            let dr = (r1 as isize - r2 as isize).unsigned_abs();
            let dc = (c1 as isize - c2 as isize).unsigned_abs();
            assert!(
                dr > 1 || dc > 1,
                "queens at ({r1},{c1}) and ({r2},{c2}) are adjacent"
            );
        }
    }
}

#[test]
fn solves_quadrant_board_n4() {
    let board = unsafe { QueensBoard::new_unchecked(quadrant_regions()) };
    let result = solve(board);

    let solved = result
        .solved_board
        .expect("quadrant board should be solvable");
    assert_valid_solution(&solved);
    assert!(result.num_states_visited > 0);
}

#[test]
fn solves_column_stripe_n4() {
    let board = unsafe { QueensBoard::new_unchecked(column_stripe_regions::<4>()) };
    assert_valid_solution(&solve(board).solved_board.expect("should be solvable"));
}

#[test]
fn solves_column_stripe_n5() {
    let board = unsafe { QueensBoard::new_unchecked(column_stripe_regions::<5>()) };
    assert_valid_solution(&solve(board).solved_board.expect("should be solvable"));
}

#[test]
fn solves_column_stripe_n6() {
    let board = unsafe { QueensBoard::new_unchecked(column_stripe_regions::<6>()) };
    assert_valid_solution(&solve(board).solved_board.expect("should be solvable"));
}

#[test]
fn solves_column_stripe_n8() {
    let board = unsafe { QueensBoard::new_unchecked(column_stripe_regions::<8>()) };
    assert_valid_solution(&solve(board).solved_board.expect("should be solvable"));
}

#[test]
fn solves_trivial_n1() {
    let regions: [[Color; 1]; 1] = [[Color::from(0)]];
    let board = unsafe { QueensBoard::new_unchecked(regions) };
    assert_valid_solution(&solve(board).solved_board.expect("1x1 board is solvable"));
}

#[test]
fn returns_none_for_unsolvable_board() {
    let board = unsafe { QueensBoard::new_unchecked(unsolvable_regions()) };
    let result = solve(board);

    assert!(result.solved_board.is_none(), "expected no solution");
    assert!(result.num_states_visited > 0);
}

#[test]
fn solver_is_deterministic() {
    let run = || solve(unsafe { QueensBoard::new_unchecked(quadrant_regions()) });

    let first = run();
    let second = run();

    assert_eq!(first.num_states_visited, second.num_states_visited);
    assert_eq!(first.solved_board, second.solved_board);
}

#[test]
fn solves_column_stripe_n18() {
    let board = unsafe { QueensBoard::new_unchecked(column_stripe_regions::<18>()) };
    let result = DepthFirstSearchSolver::<18, SmallestRegionFirstHeuristic>::solve(board);
    assert_valid_solution(
        &result
            .solved_board
            .expect("18x18 column stripe is solvable"),
    );
}

#[test]
fn const_generic_and_dynamic_agree_on_solution() {
    // The runtime `dynamic` API must dispatch to the same search and produce an
    // equivalent solution to the const-generic solver.
    let flat = [0usize, 0, 1, 1, 0, 0, 1, 1, 2, 2, 3, 3, 2, 2, 3, 3];

    let const_board = unsafe { QueensBoard::new_unchecked(quadrant_regions()) };
    let const_result =
        DepthFirstSearchSolver::<4, SmallestRegionFirstHeuristic>::solve(const_board);
    let const_queens: Vec<_> = const_result
        .solved_board
        .as_ref()
        .unwrap()
        .get_queen_positions()
        .into_iter()
        .map(|p| [p.row(), p.col()])
        .collect();

    let dynamic_solver = QueensSolverConstructor::build_flat(
        flat.to_vec(),
        SolverOption::DFSSolver,
        HeuristicOption::SmallestRegionFirstHeuristic,
    )
    .unwrap();
    let dynamic_result = dynamic_solver.solve();

    assert!(dynamic_result.solved_board.is_some());
    assert_eq!(dynamic_result.size, 4);

    let mut dynamic_queens: Vec<_> = dynamic_result
        .solved_board
        .as_ref()
        .unwrap()
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter().enumerate().filter_map(move |(c, &cell)| {
                (cell == queen_sweep_core::primitives::Cell::Queen).then_some([r, c])
            })
        })
        .collect();

    let mut const_queens = const_queens;
    const_queens.sort_unstable();
    dynamic_queens.sort_unstable();

    assert_eq!(const_queens, dynamic_queens);
}

/// A board whose colors are contiguous diagonal stripes; exercises a distinct
/// layout family beyond quadrants and column stripes.
fn diagonal_stripe_regions<const N: usize>() -> [[Color; N]; N] {
    std::array::from_fn(|row| std::array::from_fn(|col| Color::from((row + col) % N)))
}

#[test]
fn solves_diagonal_stripe_n5() {
    let board = unsafe { QueensBoard::new_unchecked(diagonal_stripe_regions::<5>()) };
    assert_valid_solution(
        &solve(board)
            .solved_board
            .expect("diagonal stripe should be solvable"),
    );
}

// ---------------------------------------------------------------------------
// Property-based: any random layout containing every color must yield either a
// valid solution or a clean `None`, and the solver must always terminate.
// ---------------------------------------------------------------------------

fn regions_from_flat<const N: usize>(flat: &[usize]) -> [[Color; N]; N] {
    std::array::from_fn(|row| std::array::from_fn(|col| Color::from(flat[row * N + col])))
}

proptest! {
    #[test]
    fn random_grid_yields_valid_solution_or_none_n4(
        mut flat in proptest::collection::vec(0usize..4, 16),
    ) {
        // Force every color to appear (on the diagonal), so the board is a
        // well-formed input regardless of the random fill.
        for i in 0..4 {
            flat[i * 4 + i] = i;
        }
        let regions = regions_from_flat::<4>(&flat);
        let board = unsafe { QueensBoard::new_unchecked(regions) };
        let result = solve(board);

        if let Some(solved) = &result.solved_board {
            assert_valid_solution(solved);
        }
    }

    #[test]
    fn random_grid_yields_valid_solution_or_none_n5(
        mut flat in proptest::collection::vec(0usize..5, 25),
    ) {
        for i in 0..5 {
            flat[i * 5 + i] = i;
        }
        let regions = regions_from_flat::<5>(&flat);
        let board = unsafe { QueensBoard::new_unchecked(regions) };
        let result = solve(board);

        if let Some(solved) = &result.solved_board {
            assert_valid_solution(solved);
        }
    }
}
