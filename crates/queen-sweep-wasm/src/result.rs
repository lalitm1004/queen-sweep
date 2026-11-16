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
    /// The queen positions as `[row, column]` pairs (empty if unsolved).
    pub queens: Vec<[u32; 2]>,
}

impl From<DynamicSolveResult> for SolveResult {
    fn from(result: DynamicSolveResult) -> Self {
        let solved_board = result.solved_board;
        let solved = solved_board.is_some();

        let board = solved_board.as_ref().map(|grid| {
            grid.iter()
                .map(|row| row.iter().map(|&cell| cell_to_u8(cell)).collect())
                .collect()
        });

        let queens = solved_board
            .as_ref()
            .map(|grid| {
                grid.iter()
                    .enumerate()
                    .flat_map(|(row, cells)| {
                        cells.iter().enumerate().filter_map(move |(col, &cell)| {
                            (cell == Cell::Queen).then_some([row as u32, col as u32])
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        Self {
            size: result.size,
            num_states_visited: result.num_states_visited,
            solved,
            board,
            queens,
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
