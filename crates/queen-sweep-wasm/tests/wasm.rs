#![cfg(target_arch = "wasm32")]

//! `wasm-bindgen-test` suite for the WebAssembly bindings.
//!
//! Run with `wasm-pack test --node` (or `--headless --chrome`).

use queen_sweep_wasm::{
    BuildErrorCode, CellKind, GridInput, HeuristicKind, SolverKind, build_from_flat,
    build_from_grid,
};
use tsify::Tsify;
use wasm_bindgen_test::wasm_bindgen_test;

fn quadrant() -> GridInput {
    GridInput {
        regions: vec![
            vec![0, 0, 1, 1],
            vec![0, 0, 1, 1],
            vec![2, 2, 3, 3],
            vec![2, 2, 3, 3],
        ],
    }
}

fn grid_ts() -> tsify::Ts<GridInput> {
    quadrant().into_ts().expect("serialize grid input")
}

/// Extracts the `BuildError` from a `Result<WasmSolver, BuildError>` without
/// requiring `WasmSolver: Debug` (which `unwrap_err` would demand).
fn expect_build_error(
    result: Result<queen_sweep_wasm::WasmSolver, queen_sweep_wasm::BuildError>,
) -> queen_sweep_wasm::BuildError {
    match result {
        Ok(_) => panic!("expected a build error"),
        Err(e) => e,
    }
}

#[wasm_bindgen_test]
fn solves_quadrant_from_grid() {
    let solver =
        build_from_grid(grid_ts(), SolverKind::Dfs, HeuristicKind::NoOp).expect("build solver");
    let result = solver
        .solve()
        .expect("solve")
        .to_rust()
        .expect("deserialize");

    assert!(result.solved);
    assert_eq!(result.size, 4);
    assert!(result.num_states_visited > 0);

    let board = result.board.expect("solved board should be present");
    let queens = board
        .iter()
        .flatten()
        .filter(|&&cell| cell == CellKind::Queen as u8)
        .count();
    assert_eq!(queens, 4);
    assert_eq!(result.queens.len(), 4);
}

#[wasm_bindgen_test]
fn solves_quadrant_from_flat() {
    let flat = vec![0u32, 0, 1, 1, 0, 0, 1, 1, 2, 2, 3, 3, 2, 2, 3, 3];
    let solver = build_from_flat(flat, SolverKind::Dfs, HeuristicKind::SmallestRegionFirst)
        .expect("build solver");
    let result = solver
        .solve()
        .expect("solve")
        .to_rust()
        .expect("deserialize");

    assert!(result.solved);
}

#[wasm_bindgen_test]
fn unsolvable_board_reports_unsolved() {
    let regions = vec![
        vec![0, 0, 1, 1],
        vec![2, 2, 2, 2],
        vec![3, 3, 3, 3],
        vec![3, 3, 3, 3],
    ];
    let solver = build_from_grid(
        GridInput { regions }
            .into_ts()
            .expect("serialize grid input"),
        SolverKind::Dfs,
        HeuristicKind::NoOp,
    )
    .expect("build solver");
    let result = solver
        .solve()
        .expect("solve")
        .to_rust()
        .expect("deserialize");

    assert!(!result.solved);
    assert!(result.board.is_none());
    assert!(result.queens.is_empty());
}

#[wasm_bindgen_test]
fn rejects_non_square_grid() {
    let regions = vec![vec![0, 0, 1], vec![1, 1, 0]];
    let result = build_from_grid(
        GridInput { regions }
            .into_ts()
            .expect("serialize grid input"),
        SolverKind::Dfs,
        HeuristicKind::NoOp,
    );

    assert_eq!(expect_build_error(result).code, BuildErrorCode::NonSquare);
}

#[wasm_bindgen_test]
fn rejects_empty_grid() {
    let result = build_from_grid(
        GridInput { regions: vec![] }
            .into_ts()
            .expect("serialize grid input"),
        SolverKind::Dfs,
        HeuristicKind::NoOp,
    );

    assert_eq!(expect_build_error(result).code, BuildErrorCode::Empty);
}

#[wasm_bindgen_test]
fn rejects_empty_flat() {
    let result = build_from_flat(vec![], SolverKind::Dfs, HeuristicKind::NoOp);
    assert_eq!(expect_build_error(result).code, BuildErrorCode::Empty);
}

#[wasm_bindgen_test]
fn rejects_flat_not_perfect_square() {
    let result = build_from_flat(vec![0, 1, 2], SolverKind::Dfs, HeuristicKind::NoOp);
    assert_eq!(
        expect_build_error(result).code,
        BuildErrorCode::NotPerfectSquare
    );
}

#[wasm_bindgen_test]
fn rejects_unsupported_size_two() {
    let result = build_from_flat(vec![0, 1, 1, 0], SolverKind::Dfs, HeuristicKind::NoOp);
    assert_eq!(
        expect_build_error(result).code,
        BuildErrorCode::UnsupportedSize
    );
}

#[wasm_bindgen_test]
fn rejects_unsupported_size_three() {
    let result = build_from_flat(vec![0u32; 9], SolverKind::Dfs, HeuristicKind::NoOp);
    assert_eq!(
        expect_build_error(result).code,
        BuildErrorCode::UnsupportedSize
    );
}

#[wasm_bindgen_test]
fn rejects_color_out_of_range() {
    let regions = vec![
        vec![0, 0, 1, 1],
        vec![0, 0, 1, 1],
        vec![2, 2, 3, 4],
        vec![2, 2, 3, 3],
    ];
    let result = build_from_grid(
        GridInput { regions }
            .into_ts()
            .expect("serialize grid input"),
        SolverKind::Dfs,
        HeuristicKind::NoOp,
    );

    assert_eq!(
        expect_build_error(result).code,
        BuildErrorCode::BoardInitFailed
    );
}

#[wasm_bindgen_test]
fn rejects_missing_color() {
    let regions = vec![
        vec![0, 0, 1, 1],
        vec![0, 0, 1, 1],
        vec![2, 2, 2, 2],
        vec![2, 2, 2, 2],
    ];
    let result = build_from_grid(
        GridInput { regions }
            .into_ts()
            .expect("serialize grid input"),
        SolverKind::Dfs,
        HeuristicKind::NoOp,
    );

    assert_eq!(
        expect_build_error(result).code,
        BuildErrorCode::BoardInitFailed
    );
}

#[wasm_bindgen_test]
fn solve_result_board_values_match_cell_kind() {
    let solver =
        build_from_grid(grid_ts(), SolverKind::Dfs, HeuristicKind::NoOp).expect("build solver");
    let result = solver
        .solve()
        .expect("solve")
        .to_rust()
        .expect("deserialize");

    let board = result.board.expect("solved board should be present");

    // Every queen coordinate must point at a cell with the Queen discriminant.
    for &[row, col] in &result.queens {
        assert_eq!(board[row as usize][col as usize], CellKind::Queen as u8);
    }

    // The board only contains valid CellKind values.
    for cell in board.iter().flatten() {
        assert!(
            *cell <= CellKind::Queen as u8,
            "unexpected cell value {cell}"
        );
    }
}

#[wasm_bindgen_test]
fn solver_is_reusable() {
    let solver =
        build_from_grid(grid_ts(), SolverKind::Dfs, HeuristicKind::NoOp).expect("build solver");

    let first = solver.solve().expect("solve");
    let second = solver.solve().expect("solve");

    let first = first.to_rust().expect("deserialize");
    let second = second.to_rust().expect("deserialize");

    assert_eq!(first.queens, second.queens);
    assert_eq!(first.num_states_visited, second.num_states_visited);
}
