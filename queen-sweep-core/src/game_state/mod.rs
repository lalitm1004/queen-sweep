mod errors;
pub use errors::GameStateError;

use std::{
    collections::{HashMap, HashSet, hash_map::DefaultHasher},
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
const MAX_BOARD_SIZE: usize = u8::MAX as usize;

#[derive(Debug, Clone)]
pub struct GameState {
    size: usize,
    states: Vec<CellState>,
    colors_with_queens: Vec<bool>,

    // immutable once initialized
    colors: Rc<[u8]>,
    color_masks: Rc<[Rc<[bool]>]>,

    heuristic: Option<HeuristicFn>,

    hash: u64,
}

// Accessors
impl GameState {
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn states(&self) -> &[CellState] {
        &self.states
    }

    #[inline]
    pub fn colors(&self) -> &[u8] {
        &self.colors
    }

    #[inline]
    pub fn hash(&self) -> u64 {
        self.hash
    }
}

// Helper functions
impl GameState {
    #[inline]
    pub fn pos_to_idx(&self, r: usize, c: usize) -> usize {
        r * self.size + c
    }

    #[inline]
    pub fn idx_to_pos(&self, idx: usize) -> (usize, usize) {
        (idx / self.size, idx % self.size)
    }

    #[inline]
    pub fn is_goal_state(&self) -> bool {
        self.colors_with_queens.iter().filter(|&&b| b).count() == self.size
    }

    #[inline]
    pub fn color_at_idx(&self, idx: usize) -> u8 {
        self.colors[idx] as u8
    }

    #[inline]
    fn get_color_mask(&self, color: u8) -> &[bool] {
        &self.color_masks[color as usize]
    }

    #[inline]
    fn in_bounds(&self, r: i32, c: i32) -> bool {
        r >= 0 && c >= 0 && r < self.size as i32 && c < self.size as i32
    }
}

impl GameState {
    pub fn from_color_regions(
        color_regions: Vec<Vec<u8>>,
        heuristic: Option<HeuristicFn>,
    ) -> Result<Self, GameStateError> {
        let mut base = GameState::try_from(color_regions)?;
        base.heuristic = heuristic;
        Ok(base)
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
        let color_mask = self.get_color_mask(color);
        for i in 0..new_states.len() {
            if color_mask[i] {
                new_states[i] = CellState::Blocked;
            }
        }

        // place queen
        new_states[idx] = CellState::Queen;
        new_colors_with_queens[color as usize] = true;

        // block all invalid moves
        for idx in 0..new_states.len() {
            if new_states[idx] == CellState::Empty {
                let (r, c) = self.idx_to_pos(idx);
                if !self.can_place_queen(&new_states, &new_colors_with_queens, r, c) {
                    new_states[idx] = CellState::Blocked;
                }
            }
        }

        let hash = compute_hash(&new_states);

        GameState {
            size: self.size,
            states: new_states,
            colors_with_queens: new_colors_with_queens,
            colors: Rc::clone(&self.colors),
            color_masks: Rc::clone(&self.color_masks),
            heuristic: self.heuristic,
            hash,
        }
    }

    pub fn valid_placements(&self) -> Vec<(usize, usize)> {
        let positions: Vec<(usize, usize)> = self
            .states
            .iter()
            .enumerate()
            .filter_map(|(idx, state)| {
                if *state == CellState::Empty {
                    Some(self.idx_to_pos(idx))
                } else {
                    None
                }
            })
            .collect();

        let heuristic_fn = match self.heuristic {
            Some(f) => f,
            None => return positions,
        };

        let ctx = HeuristicContext {
            positions: &positions,
            size: self.size,
            states: &self.states,
            colors_with_queens: &self.colors_with_queens,
            colors: &self.colors,
            color_masks: &self.color_masks,
        };

        let mut scored = heuristic_fn(&ctx);

        scored.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        scored.into_iter().map(|(pos, _)| pos).collect()
    }

    #[inline]
    pub fn queen_positions(&self) -> impl Iterator<Item = (usize, usize)> {
        self.states.iter().enumerate().filter_map(|(i, state)| {
            if *state == CellState::Queen {
                let (r, c) = self.idx_to_pos(i);
                Some((r, c))
            } else {
                None
            }
        })
    }

    fn can_place_queen(
        &self,
        states: &[CellState],
        colors_with_queens: &[bool],
        r: usize,
        c: usize,
    ) -> bool {
        let idx = self.pos_to_idx(r, c);

        if states[idx] != CellState::Empty {
            return false;
        }

        // check if color already has a queen
        let queen_color = self.color_at_idx(idx);
        if colors_with_queens[queen_color as usize] {
            return false;
        }

        // 1-step lookahead
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

            if self.in_bounds(nr, nc) {
                let neighbor_idx = self.pos_to_idx(nr as usize, nc as usize);
                will_be_blocked[neighbor_idx] = true;
            }
        }

        // block color region
        let color_mask = self.get_color_mask(queen_color);
        for idx in 0..will_be_blocked.len() {
            if color_mask[idx] {
                will_be_blocked[idx] = true;
            }
        }

        // all other regions must have at least one valid empty cell OR a queen placed already
        for color in 0..self.size {
            // skip own color and colors that already have queens
            if color == queen_color as usize || colors_with_queens[color] {
                continue;
            }

            let mut region_has_valid_empty = false;
            let color_mask = self.get_color_mask(color as u8);

            for idx in 0..states.len() {
                if states[idx] == CellState::Empty && color_mask[idx] && !will_be_blocked[idx] {
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
}

impl TryFrom<Vec<Vec<u8>>> for GameState {
    type Error = GameStateError;

    fn try_from(color_regions: Vec<Vec<u8>>) -> Result<Self, Self::Error> {
        let size = color_regions.len();

        if size == 0 {
            return Err(GameStateError::InexistentBoard);
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

        // normalize board to be continous from 0
        let color_regions = normalize_colors(color_regions);

        let total_cells = size * size;
        let states = vec![CellState::Empty; total_cells];
        let hash = compute_hash(&states);

        let colors: Vec<u8> = color_regions.into_iter().flatten().collect();
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

        // build color masks
        let colors_masks: Vec<Rc<[bool]>> = unique_colors
            .iter()
            .map(|&color| {
                let mask: Vec<bool> = colors.iter().map(|&c| c == color).collect();
                Rc::from(mask.into_boxed_slice())
            })
            .collect();

        let color_masks: Rc<[Rc<[bool]>]> = Rc::from(colors_masks.into_boxed_slice());
        let colors_with_queens = vec![false; size];
        let colors: Rc<[u8]> = Rc::from(colors);

        Ok(GameState {
            size,
            states,
            colors_with_queens,
            color_masks,
            colors,
            heuristic: None,
            hash,
        })
    }
}

fn normalize_colors(color_regions: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut unique_colors: Vec<u8> = color_regions
        .iter()
        .flat_map(|row| row.iter().copied())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    unique_colors.sort_unstable();

    let color_map: HashMap<u8, u8> = unique_colors
        .into_iter()
        .enumerate()
        .map(|(new_color, old_color)| (old_color, new_color as u8))
        .collect();

    color_regions
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|color| *color_map.get(&color).unwrap_or(&0))
                .collect()
        })
        .collect()
}

impl Hash for GameState {
    #[inline]
    fn hash<H: Hasher>(&self, hash_state: &mut H) {
        // use precomputed hash
        hash_state.write_u64(self.hash);
    }
}

fn compute_hash(states: &[CellState]) -> u64 {
    // only states change over the course of a dfs
    // all other attributes are either static or derived from states
    let mut hasher = DefaultHasher::new();
    states.hash(&mut hasher);
    hasher.finish()
}

impl PartialEq for GameState {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.states == other.states
    }
}

impl Eq for GameState {}

#[cfg(feature = "display")]
mod display;
