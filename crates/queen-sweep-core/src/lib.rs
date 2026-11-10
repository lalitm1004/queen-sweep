//! A solver for the "Queens" puzzle.
//!
//! The Queens puzzle asks for a placement of exactly one queen in every row,
//! column, and colored region of an `N×N` board, such that no two queens touch
//! (not even diagonally). This crate models the board, enforces the placement
//! constraints, and searches for a solution via backtracking.
//!
//! # Modules
//!
//! - [`primitives`]: the fundamental types — [`primitives::Cell`],
//!   [`primitives::Color`], [`primitives::Position`], and
//!   [`primitives::ValidBoardSize`].
//! - [`board`]: the [`board::QueensBoard`] state type, which tracks placements
//!   and derives the blocked cells from them.
//! - [`heuristic`]: move-ordering heuristics via the [`heuristic::QueensHeuristic`]
//!   trait.
//! - [`solver`]: backtracking search via [`solver::DepthFirstSearchSolver`].

pub mod board;
pub mod heuristic;
pub mod primitives;
pub mod solver;
