#![cfg(target_arch = "wasm32")]

//! `wasm-bindgen-test` suite for the WebAssembly bindings.
//!
//! Run with `wasm-pack test --node` (or `--headless --chrome`).

use queen_sweep_wasm::{
    CellKind, GridInput, HeuristicKind, SolverKind, build_from_flat, build_from_grid,
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

    assert!(result.is_err());
}
