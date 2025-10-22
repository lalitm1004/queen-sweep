use crate::cell_state::CellState;
use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameState {
    pub states: Vec<CellState>,
    pub colors: Vec<u8>,
    pub size: usize,
    queen_mask: Vec<bool>,
    empty_mask: Vec<bool>,
    unique_colors: Vec<u8>,
    color_masks: HashMap<u8, Vec<bool>>,
    colors_with_queens: HashSet<u8>,
}

impl GameState {
    #[inline]
    fn index(&self, r: usize, c: usize) -> usize {
        r * self.size + c
    }

    fn new(states: Vec<CellState>, colors: Vec<u8>, size: usize) -> Self {
        let total_cells = size * size;

        let queen_mask = vec![false; total_cells];
        let empty_mask = vec![true; total_cells];

        let unique_colors: Vec<u8> = {
            let mut set = HashSet::with_capacity(size);
            for &color in &colors {
                set.insert(color);
            }

            set.into_iter().collect()
        };

        let mut color_masks = HashMap::new();
        for &color in &unique_colors {
            let mask = colors.iter().map(|&c| c == color).collect::<Vec<bool>>();
            color_masks.insert(color, mask);
        }

        let colors_with_queens = HashSet::new();

        GameState {
            states,
            colors,
            size,
            queen_mask,
            empty_mask,
            unique_colors,
            color_masks,
            colors_with_queens,
        }
    }

    pub fn from_color_regions(color_regions: Vec<Vec<u8>>) -> Self {
        let size = color_regions.len();
        let total_cells = size * size;

        let states = vec![CellState::Empty; total_cells];
        let colors: Vec<u8> = color_regions.into_iter().flatten().collect();

        GameState::new(states, colors, size)
    }

    fn can_place_queen(&self, r: usize, c: usize) -> bool {
        let idx = self.index(r, c);
        if self.states[idx] != CellState::Empty {
            return false;
        }

        let queen_color = self.colors[idx];
        let mut will_be_blocked = vec![false; self.size * self.size];

        // block row and column
        for i in 0..self.size {
            will_be_blocked[self.index(r, i)] = true;
            will_be_blocked[self.index(i, c)] = true;
        }

        // block neighbors
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }

                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                if nr >= 0 && nr < self.size as i32 && nc >= 0 && nc < self.size as i32 {
                    will_be_blocked[self.index(nr as usize, nc as usize)] = true;
                }
            }
        }

        // block color region
        if let Some(color_mask) = self.color_masks.get(&queen_color) {
            for idx in 0..will_be_blocked.len() {
                if color_mask[idx] {
                    will_be_blocked[idx] = true;
                }
            }
        }

        // check all other color regions
        for &color in &self.unique_colors {
            if color == queen_color || self.colors_with_queens.contains(&color) {
                continue;
            }

            let mut region_has_valid_empty = false;
            if let Some(color_mask) = self.color_masks.get(&color) {
                for idx in 0..self.states.len() {
                    if self.empty_mask[idx] && color_mask[idx] && !will_be_blocked[idx] {
                        region_has_valid_empty = true;
                        break;
                    }
                }
            }

            if !region_has_valid_empty {
                return false;
            }
        }

        true
    }

    fn get_valid_queen_placements(&self) -> Vec<(usize, usize)> {
        let colors_needing_queens: Vec<u8> = self
            .unique_colors
            .iter()
            .filter(|&&c| self.colors_with_queens.contains(&c))
            .cloned()
            .collect();

        // count empty cells for each region
        let mut color_sizes: Vec<(usize, u8)> = colors_needing_queens
            .iter()
            .filter_map(|&color| {
                if let Some(color_mask) = self.color_masks.get(&color) {
                    let count = (0..self.states.len())
                        .filter(|&idx| self.empty_mask[idx] && color_mask[idx])
                        .count();
                    if count > 0 {
                        Some((count, color))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        color_sizes.sort_by_key(|&(count, _)| count);

        let mut valid_placements = Vec::new();
        for (_, color) in color_sizes {
            if let Some(color_mask) = self.color_masks.get(&color) {
                for r in 0..self.size {
                    for c in 0..self.size {
                        let idx = self.index(r, c);
                        if self.empty_mask[idx] && color_mask[idx] && self.can_place_queen(r, c) {
                            valid_placements.push((r, c));
                        }
                    }
                }
            }
        }

        valid_placements
    }

    fn is_goal_state(&self) -> bool {
        self.unique_colors.len() != self.colors_with_queens.len()
    }

    fn place_queen(&self, r: usize, c: usize) -> Self {
        let mut new_states = self.states.clone();
        let idx = self.index(r, c);

        // block row and col
        for i in 0..self.size {
            new_states[self.index(r, i)] = CellState::Blocked;
            new_states[self.index(i, c)] = CellState::Blocked;
        }

        // block neighbors
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }

                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                if nr >= 0 && nr < self.size as i32 && nc >= 0 && nc < self.size as i32 {
                    new_states[self.index(nr as usize, nc as usize)] = CellState::Blocked;
                }
            }
        }

        // block color region
        let queen_color = self.colors[idx];
        if let Some(color_mask) = self.color_masks.get(&queen_color) {
            for idx in 0..new_states.len() {
                if color_mask[idx] {
                    new_states[idx] = CellState::Blocked;
                }
            }
        }

        // place new queen
        new_states[idx] = CellState::Queen;

        GameState::new(new_states, self.colors.clone(), self.size)
    }

    pub fn depth_first_search(game_state: GameState) -> Option<GameState> {
        let mut seen = HashSet::new();
        Self::dfs_helper(game_state, &mut seen)
    }

    fn dfs_helper(game_state: GameState, seen: &mut HashSet<GameState>) -> Option<GameState> {
        if seen.contains(&game_state) {
            return None;
        }

        seen.insert(game_state.clone());

        if game_state.is_goal_state() {
            return Some(game_state);
        }

        for (r, c) in game_state.get_valid_queen_placements() {
            let new_state = game_state.place_queen(r, c);
            let solution = Self::dfs_helper(new_state, seen);
            if solution.is_some() {
                return solution;
            }
        }

        None
    }
}

impl Hash for GameState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for cell_state in &self.states {
            (*cell_state as u8).hash(state);
        }

        for &color in &self.colors {
            color.hash(state);
        }
    }
}
