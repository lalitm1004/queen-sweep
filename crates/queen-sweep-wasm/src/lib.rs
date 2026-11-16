//! WebAssembly bindings for the `queen-sweep-core` dynamic solver.
//!
//! This crate is a thin, type-safe wrapper around
//! [`queen_sweep_core::dynamic`]. It exposes:
//!
//! - [`SolverKind`], [`HeuristicKind`], and [`CellKind`] as JavaScript enums.
//! - [`build_from_grid`] / [`build_from_flat`] to construct a [`WasmSolver`].
//! - [`WasmSolver::solve`] to run the solver.
//!
//! The generated TypeScript declarations (via `wasm-pack` + `tsify`) describe
//! [`GridInput`] and [`SolveResult`] structurally, so the bindings are
//! seamless to consume from TypeScript.
//!
//! The crate is environment-agnostic (no DOM / `web-sys`): it runs equally well
//! in a Chrome extension service worker, an offscreen document, or a content
//! script's isolated world.

mod error;
mod input;
mod options;
mod result;
mod solver;

pub use error::{BuildError, BuildErrorCode};
pub use input::GridInput;
pub use options::{CellKind, HeuristicKind, SolverKind};
pub use result::SolveResult;
pub use solver::{WasmSolver, build_from_flat, build_from_grid};

use wasm_bindgen::prelude::*;

/// Installs a panic hook so Rust panics surface as readable JS errors in dev.
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}
