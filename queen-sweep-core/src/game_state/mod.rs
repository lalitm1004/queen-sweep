use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    rc::Rc,
};

use crate::{CellState, heuristic::HeuristicFn};

#[derive(Debug, Clone)]
pub struct GameState {
    pub size: usize,
    pub states: Vec<CellState>,
    colors_with_queens: Vec<bool>,

    // immutable once initialized
    color_masks: Rc<[Rc<[bool]>]>,

    heuristic: Option<HeuristicFn>,
}

impl GameState {
    #[inline]
    pub fn pos_to_idx(&self, pos: (usize, usize)) -> usize {
        pos.0 * self.size + pos.1
    }

    #[inline]
    pub fn idx_to_pos(&self, idx: usize) -> (usize, usize) {
        let r = idx / self.size;
        let c = idx % self.size;

        (r, c)
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

        GameState {
            size,
            states,
            colors_with_queens,
            color_masks,
            heuristic,
        }
    }

    #[inline]
    pub fn is_goal_state(&self) -> bool {
        self.colors_with_queens.len() == self.size
    }
}

impl Hash for GameState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.states.hash(state);
    }
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.states == other.states
    }
}

impl Eq for GameState {}

#[cfg(test)]
mod test;
