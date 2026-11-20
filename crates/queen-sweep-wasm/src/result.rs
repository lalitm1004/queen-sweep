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

#[cfg(test)]
mod tests {
    use super::{cell_to_u8, SolveResult};
    use crate::options::CellKind;
    use queen_sweep_core::{dynamic::DynamicSolveResult, primitives::Cell};

    #[test]
    fn cell_to_u8_maps_every_variant() {
        assert_eq!(cell_to_u8(Cell::Empty), 0);
        assert_eq!(cell_to_u8(Cell::Blocked), 1);
        assert_eq!(cell_to_u8(Cell::Queen), 2);
        assert_eq!(CellKind::Empty as u8, 0);
        assert_eq!(CellKind::Blocked as u8, 1);
        assert_eq!(CellKind::Queen as u8, 2);
    }

    fn dyn_result(
        solved: Option<Vec<Vec<Cell>>>,
    ) -> DynamicSolveResult {
        DynamicSolveResult {
            size: 4,
            solved_board: solved,
            num_states_visited: 7,
        }
    }

    #[test]
    fn solve_result_from_solved_board() {
        let board = vec![
            vec![Cell::Blocked, Cell::Queen, Cell::Blocked, Cell::Blocked],
            vec![Cell::Blocked, Cell::Blocked, Cell::Blocked, Cell::Queen],
            vec![Cell::Queen, Cell::Blocked, Cell::Blocked, Cell::Blocked],
            vec![Cell::Blocked, Cell::Blocked, Cell::Queen, Cell::Blocked],
        ];

        let result = SolveResult::from(dyn_result(Some(board)));

        assert!(result.solved);
        assert_eq!(result.size, 4);
        assert_eq!(result.num_states_visited, 7);

        let board = result.board.expect("board present when solved");
        // Cells are mapped to their numeric CellKind values.
        assert_eq!(board[0][1], 2);
        assert_eq!(board[0][0], 1);

        // Queens are extracted as [row, col] pairs.
        assert_eq!(result.queens.len(), 4);
        assert!(result.queens.contains(&[0, 1]));
        assert!(result.queens.contains(&[1, 3]));
        assert!(result.queens.contains(&[2, 0]));
        assert!(result.queens.contains(&[3, 2]));
    }

    #[test]
    fn solve_result_from_unsolved_board() {
        let result = SolveResult::from(dyn_result(None));

        assert!(!result.solved);
        assert!(result.board.is_none());
        assert!(result.queens.is_empty());
        assert_eq!(result.size, 4);
        assert_eq!(result.num_states_visited, 7);
    }
}
