//! Runtime solver and heuristic selection.

/// The solver algorithm to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolverOption {
    /// Depth-first backtracking search.
    DFSSolver,
}

/// The move-ordering heuristic to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeuristicOption {
    /// Identity ordering: try placements in board order.
    NoOpHeuristic,

    /// Prefer placements in the most-constrained (smallest) color region.
    SmallestRegionFirstHeuristic,
}
