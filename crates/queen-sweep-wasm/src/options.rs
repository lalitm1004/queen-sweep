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

#[cfg(test)]
mod tests {
    use super::{CellKind, HeuristicKind, SolverKind};
    use queen_sweep_core::dynamic::{HeuristicOption, SolverOption};

    #[test]
    fn solver_kind_converts_to_solver_option() {
        assert_eq!(SolverOption::from(SolverKind::Dfs), SolverOption::DFSSolver);
    }

    #[test]
    fn heuristic_kind_converts_to_heuristic_option() {
        assert_eq!(
            HeuristicOption::from(HeuristicKind::NoOp),
            HeuristicOption::NoOpHeuristic
        );
        assert_eq!(
            HeuristicOption::from(HeuristicKind::SmallestRegionFirst),
            HeuristicOption::SmallestRegionFirstHeuristic
        );
    }

    #[test]
    fn cell_kind_discriminants_match_solver_contract() {
        // The numeric values are part of the JS-facing contract (CellKind is
        // serialized into SolveResult.board); they must stay 0/1/2.
        assert_eq!(CellKind::Empty as u8, 0);
        assert_eq!(CellKind::Blocked as u8, 1);
        assert_eq!(CellKind::Queen as u8, 2);
    }
}
