//! Runtime option and cell enums exposed to JavaScript.

use queen_sweep_core::dynamic::{HeuristicOption, SolverOption};
use wasm_bindgen::prelude::*;

/// The solver algorithm to use.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolverKind {
    /// Depth-first backtracking search.
    Dfs = 0,
}

/// The move-ordering heuristic to use.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeuristicKind {
    /// Identity ordering: try placements in board order.
    NoOp = 0,
    /// Prefer placements in the most-constrained (smallest) color region.
    SmallestRegionFirst = 1,
}

/// The state of a board cell.
///
/// The numeric values match the entries of `SolveResult.board`.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellKind {
    /// A cell with no queen and no blocking applied.
    Empty = 0,
    /// A cell that may not receive a queen.
    Blocked = 1,
    /// A cell that contains a placed queen.
    Queen = 2,
}

impl From<SolverKind> for SolverOption {
    fn from(kind: SolverKind) -> Self {
        match kind {
            SolverKind::Dfs => SolverOption::DFSSolver,
        }
    }
}

impl From<HeuristicKind> for HeuristicOption {
    fn from(kind: HeuristicKind) -> Self {
        match kind {
            HeuristicKind::NoOp => HeuristicOption::NoOpHeuristic,
            HeuristicKind::SmallestRegionFirst => HeuristicOption::SmallestRegionFirstHeuristic,
        }
    }
}
