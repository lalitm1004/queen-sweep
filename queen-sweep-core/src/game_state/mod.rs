use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    rc::Rc,
};

use crate::{CellState, heuristic::HeuristicFn};

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

#[derive(Debug, Clone)]
pub struct GameState {
    pub size: usize,
    pub states: Vec<CellState>,
    colors_with_queens: Vec<bool>,

    // immutable once initialized
    colors: Rc<[u8]>,
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
        self.colors_with_queens.len() == self.size
    }

    #[inline]
    pub fn color_at_idx(&self, idx: usize) -> usize {
        self.colors[idx] as usize
    }

    #[inline]
    pub fn color_mask(&self, color: usize) -> &[bool] {
        &self.color_masks[color]
    }

    pub fn from_color_regions(color_regions: Vec<Vec<u8>>, heuristic: Option<HeuristicFn>) -> Self {
        let size = color_regions.len();
        let total_cells = size * size;

        let states = vec![CellState::Empty; total_cells];

        let colors: Vec<u8> = color_regions.into_iter().flatten().collect();

        let mut unique_colors = HashSet::with_capacity(size);
        for &color in &colors {
            unique_colors.insert(color);
        }
        let unique_colors: Vec<u8> = unique_colors.into_iter().collect();

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

        GameState {
            size,
            states,
            colors_with_queens,
            color_masks,
            colors,
            heuristic,
        }
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
