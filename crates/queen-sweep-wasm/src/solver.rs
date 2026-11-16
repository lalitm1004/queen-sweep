//! The `WasmSolver` handle and its constructors.

use queen_sweep_core::dynamic::{DynamicSolver, QueensSolverConstructor};
use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

use crate::{
    error::{BuildError, BuildErrorCode},
    input::GridInput,
    options::{HeuristicKind, SolverKind},
    result::SolveResult,
};

/// A reusable solver handle.
///
/// Built via `buildFromGrid` or `buildFromFlat`; call `solve` as many times as
/// needed.
#[wasm_bindgen]
pub struct WasmSolver {
    inner: DynamicSolver,
}

#[wasm_bindgen]
impl WasmSolver {
    /// Runs the solver, returning the solution (if any) and search statistics.
    pub fn solve(&self) -> Result<Ts<SolveResult>, JsError> {
        Ok(SolveResult::from(self.inner.solve()).into_ts()?)
    }
}

/// Builds a solver from a square grid of color regions.
///
/// Throws a JavaScript error if the input is malformed or the board size is
/// unsupported.
#[wasm_bindgen(js_name = buildFromGrid)]
pub fn build_from_grid(
    grid: Ts<GridInput>,
    solver: SolverKind,
    heuristic: HeuristicKind,
) -> Result<WasmSolver, BuildError> {
    let grid = grid.to_rust().map_err(|err| {
        BuildError::new(
            BuildErrorCode::InvalidInput,
            format!("invalid grid input: {err}"),
        )
    })?;

    let regions = grid
        .regions
        .into_iter()
        .map(|row| row.into_iter().map(|color| color as usize).collect())
        .collect();

    QueensSolverConstructor::build_grid(regions, solver.into(), heuristic.into())
        .map(|inner| WasmSolver { inner })
        .map_err(BuildError::from)
}

/// Builds a solver from a flat, row-major list of color regions.
///
/// Throws a JavaScript error if the input is malformed or the board size is
/// unsupported.
#[wasm_bindgen(js_name = buildFromFlat)]
pub fn build_from_flat(
    flat: Vec<u32>,
    solver: SolverKind,
    heuristic: HeuristicKind,
) -> Result<WasmSolver, BuildError> {
    let regions: Vec<usize> = flat.into_iter().map(|color| color as usize).collect();

    QueensSolverConstructor::build_flat(regions, solver.into(), heuristic.into())
        .map(|inner| WasmSolver { inner })
        .map_err(BuildError::from)
}
