mod errors;
pub use errors::GameStateError;

use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    rc::Rc,
};

use crate::{
    CellState,
    heuristic::{HeuristicContext, HeuristicFn},
};

const NEIGHBOR_DISPLACEMENTS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
const MAX_BOARD_SIZE: usize = 255;

#[derive(Debug, Clone)]
pub struct GameState {
    pub size: usize,
    pub states: Vec<CellState>,
    colors_with_queens: Vec<bool>,

    // immutable once initialized
    pub colors: Rc<[u8]>,
    color_masks: Rc<[Rc<[bool]>]>,

    heuristic: Option<HeuristicFn>,
}

impl GameState {
    #[inline]
    pub fn pos_to_idx(&self, r: usize, c: usize) -> usize {
        r * self.size + c
    }

    #[inline]
    pub fn idx_to_pos(&self, idx: usize) -> (usize, usize) {
        let r = idx / self.size;
        let c = idx % self.size;

        (r, c)
    }

    #[inline]
    pub fn is_goal_state(&self) -> bool {
        self.colors_with_queens.iter().filter(|&&b| b).count() == self.size
    }

    #[inline]
    pub fn color_at_idx(&self, idx: usize) -> usize {
        self.colors[idx] as usize
    }

    #[inline]
    pub fn color_mask(&self, color: usize) -> &[bool] {
        &self.color_masks[color]
    }

    pub fn from_color_regions(
        color_regions: Vec<Vec<u8>>,
        heuristic: Option<HeuristicFn>,
    ) -> Result<Self, GameStateError> {
        let size = color_regions.len();

        if size == 0 {
            return Err(GameStateError::InvalidBoardSize(size));
        }

        if size > MAX_BOARD_SIZE {
            return Err(GameStateError::BoardTooLarge {
                size,
                max_size: MAX_BOARD_SIZE,
            });
        }

        // validate square board
        for row in color_regions.iter() {
            if row.len() != size {
                return Err(GameStateError::NonSquareBoard {
                    rows: size,
                    cols: row.len(),
                });
            }
        }

        let total_cells = size * size;
        let states = vec![CellState::Empty; total_cells];

        let colors: Vec<u8> = color_regions.into_iter().flatten().collect();

        // verify cell count
        if colors.len() != total_cells {
            return Err(GameStateError::InvalidCellCount {
                expected: total_cells,
                found: colors.len(),
            });
        }

        // collect unique colors
        let mut unique_colors = HashSet::with_capacity(size);
        for &color in &colors {
            unique_colors.insert(color);
        }
        let mut unique_colors: Vec<u8> = unique_colors.into_iter().collect();
        unique_colors.sort_unstable();

        // ensure colors start from 0
        if let Some(&first_color) = unique_colors.first() {
            if first_color != 0 {
                return Err(GameStateError::ColorsNotStartingFromZero { first_color });
            }
        }

        // check for continuous values
        for i in 1..unique_colors.len() {
            if unique_colors[i] != unique_colors[i - 1] + 1 {
                return Err(GameStateError::NonContinuousColors {
                    expected: unique_colors[i - 1] + 1,
                    found: unique_colors[i],
                });
            }
        }

        // build color masks
        let colors_masks: Vec<Rc<[bool]>> = unique_colors
            .iter()
            .map(|&color| {
                let mask: Vec<bool> = colors.iter().map(|&c| c == color).collect();

                Ok(Rc::from(mask.into_boxed_slice()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let color_masks: Rc<[Rc<[bool]>]> = Rc::from(colors_masks.into_boxed_slice());

        let colors_with_queens = vec![false; size];
        let colors: Rc<[u8]> = Rc::from(colors);

        Ok(GameState {
            size,
            states,
            colors_with_queens,
            color_masks,
            colors,
            heuristic,
        })
    }

    pub fn place_queen(&self, r: usize, c: usize) -> Self {
        let mut new_states = self.states.clone();
        let mut new_colors_with_queens = self.colors_with_queens.clone();

        let idx = self.pos_to_idx(r, c);

        // block row and col
        for i in 0..self.size {
            let row_idx = self.pos_to_idx(r, i);
            let col_idx = self.pos_to_idx(i, c);

            new_states[row_idx] = CellState::Blocked;
            new_states[col_idx] = CellState::Blocked;
        }

        // block neighbors
        for (dr, dc) in NEIGHBOR_DISPLACEMENTS {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;

            if nr >= 0 && nr < self.size as i32 && nc >= 0 && nc < self.size as i32 {
                let neighbor_idx = self.pos_to_idx(nr as usize, nc as usize);
                new_states[neighbor_idx] = CellState::Blocked;
            }
        }

        // block color region
        let color = self.color_at_idx(idx);
        let color_mask = self.color_mask(color);
        for i in 0..new_states.len() {
            if color_mask[i] {
                new_states[i] = CellState::Blocked;
            }
        }

        // place queen
        new_states[idx] = CellState::Queen;
        new_colors_with_queens[color] = true;

        GameState {
            size: self.size,
            states: new_states,
            colors_with_queens: new_colors_with_queens,
            colors: Rc::clone(&self.colors),
            color_masks: Rc::clone(&self.color_masks),
            heuristic: self.heuristic,
        }
    }

    fn can_place_queen(&self, r: usize, c: usize) -> bool {
        let idx = self.pos_to_idx(r, c);

        if self.states[idx] != CellState::Empty {
            return false;
        }

        // color already has a queen
        let queen_color = self.color_at_idx(idx);
        if self.colors_with_queens[queen_color] {
            return false;
        }

        // 1-step lookahead: make sure this move doesn't block entirety of another region
        let mut will_be_blocked = vec![false; self.size * self.size];

        // block row and col
        for i in 0..self.size {
            will_be_blocked[self.pos_to_idx(r, i)] = true;
            will_be_blocked[self.pos_to_idx(i, c)] = true;
        }

        // block neighbors
        for (dr, dc) in NEIGHBOR_DISPLACEMENTS {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;

            if nr >= 0 && nr < self.size as i32 && nc >= 0 && nc < self.size as i32 {
                let neighbor_idx = self.pos_to_idx(nr as usize, nc as usize);
                will_be_blocked[neighbor_idx] = true;
            }
        }

        // block color region
        let color_mask = self.color_mask(queen_color);
        for idx in 0..will_be_blocked.len() {
            if color_mask[idx] {
                will_be_blocked[idx] = true;
            }
        }

        // all other regions must have at least one valid empty cell OR a queen placed already
        for color in 0..self.size {
            // checking own color / color already has a queen
            if color == queen_color || self.colors_with_queens[color] {
                continue;
            }

            let mut region_has_valid_empty = false;
            let color_mask = self.color_mask(color);

            for idx in 0..self.states.len() {
                if self.states[idx] == CellState::Empty && color_mask[idx] && !will_be_blocked[idx]
                {
                    region_has_valid_empty = true;
                    break;
                }
            }

            if !region_has_valid_empty {
                return false;
            }
        }

        true
    }

    pub fn get_valid_placements(&self) -> Vec<(usize, usize)> {
        let valid_placements: Vec<(usize, usize)> = (0..self.size)
            .flat_map(|r| (0..self.size).map(move |c| (r, c)))
            .filter(|&(r, c)| self.can_place_queen(r, c))
            .collect();

        if let Some(heuristic_fn) = self.heuristic {
            let ctx = HeuristicContext {
                positions: &valid_placements,
                size: self.size,
                states: &self.states,
                colors_with_queens: &self.colors_with_queens,
                colors: &self.colors,
                color_masks: &self.color_masks,
            };
            let heuristic_values = heuristic_fn(&ctx);

            let mut paired: Vec<((usize, usize), f32)> = valid_placements
                .into_iter()
                .zip(heuristic_values.into_iter())
                .collect();

            // sort by heuristic in ascending order
            paired.sort_unstable_by(|a, b| {
                a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)
            });

            return paired.into_iter().map(|(pos, _)| pos).collect();
        }

        valid_placements
    }

    pub fn get_queen_positions(&self) -> Vec<(usize, usize)> {
        self.states
            .iter()
            .enumerate()
            .filter_map(|(i, state)| {
                if *state == CellState::Queen {
                    let (r, c) = self.idx_to_pos(i);
                    Some((r, c))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Hash for GameState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.states.hash(state);
    }
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.states == other.states
    }
}

impl Eq for GameState {}

#[cfg(test)]
mod test;
