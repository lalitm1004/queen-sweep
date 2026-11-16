//! Solve output types.

use queen_sweep_core::{dynamic::DynamicSolveResult, primitives::Cell};
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::options::CellKind;

/// The result of a solve.
#[derive(Tsify, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolveResult {
    /// The board size `N`.
    pub size: usize,
    /// The number of board states visited during the search.
    pub num_states_visited: usize,
    /// Whether a solution was found.
    pub solved: bool,
    /// The solved board as numeric `CellKind` values, or `undefined` if no
    /// solution was found.
    pub board: Option<Vec<Vec<u8>>>,
}

impl From<DynamicSolveResult> for SolveResult {
    fn from(result: DynamicSolveResult) -> Self {
        Self {
            size: result.size,
            num_states_visited: result.num_states_visited,
            solved: result.solved_board.is_some(),
            board: result.solved_board.map(|grid| {
                grid.into_iter()
                    .map(|row| row.into_iter().map(cell_to_u8).collect())
                    .collect()
            }),
        }
    }
}

/// Maps a board [`Cell`] to its numeric [`CellKind`] value.
fn cell_to_u8(cell: Cell) -> u8 {
    match cell {
        Cell::Empty => CellKind::Empty as u8,
        Cell::Blocked => CellKind::Blocked as u8,
        Cell::Queen => CellKind::Queen as u8,
    }
}
