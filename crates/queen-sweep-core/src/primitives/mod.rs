//! Fundamental types shared across the crate.
//!
//! These are the small, reusable building blocks used by the board and solver:
//! board cells ([`Cell`]), color regions ([`Color`]), board coordinates
//! ([`Position`]), and the compile-time board-size bound ([`ValidBoardSize`]).

mod cell;
mod color;
mod position;
mod size;

pub use cell::Cell;
pub use color::Color;
pub use position::Position;
pub use size::ValidBoardSize;
