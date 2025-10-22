use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

use crate::cell_state::CellState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub size: usize,
    pub states: Vec<CellState>,
    pub colors: Vec<u8>,
    unique_colors: Vec<u8>,
    colors_with_queens: HashSet<u8>,
    color_masks: HashMap<u8, Vec<bool>>,
}

impl GameState {
    #[inline]
    pub fn index(&self, r: usize, c: usize) -> usize {
        r * self.size + c
    }

    fn new(states: Vec<CellState>, colors: Vec<u8>, size: usize) -> Self {
        let unique_colors: Vec<u8> = {
            let mut set = HashSet::with_capacity(size);
            for &color in &colors {
                set.insert(color);
            }

            set.into_iter().collect()
        };

        let mut color_masks = HashMap::with_capacity(unique_colors.len());
        let mut color_region_sizes = HashMap::with_capacity(unique_colors.len());

        for &color in &unique_colors {
            let mask = colors.iter().map(|&c| c == color).collect::<Vec<bool>>();
            let size = mask.iter().filter(|&&b| b).count();
            color_region_sizes.insert(color, size);
            color_masks.insert(color, mask);
        }

        let colors_with_queens = HashSet::new();

        GameState {
            size,
            states,
            colors,
            unique_colors,
            colors_with_queens,
            color_masks,
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

        // color already has a queen
        if self.colors_with_queens.contains(&queen_color) {
            return false;
        }

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
        let color_mask = &self.color_masks[&queen_color];
        for idx in 0..will_be_blocked.len() {
            if color_mask[idx] {
                will_be_blocked[idx] = true;
            }
        }

        // all other color regions must have at least one valid empty cell
        for &color in &self.unique_colors {
            if color == queen_color || self.colors_with_queens.contains(&color) {
                continue;
            }

            let mut region_has_valid_empty = false;
            let color_mask = &self.color_masks[&color];

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

    pub fn get_valid_queen_placements(&self) -> Vec<(usize, usize)> {
        let colors_needing_queens: Vec<u8> = self
            .unique_colors
            .iter()
            .filter(|&&c| !self.colors_with_queens.contains(&c))
            .cloned()
            .collect();

        // count empty cells for each region
        let mut color_sizes: Vec<(usize, u8)> = colors_needing_queens
            .iter()
            .filter_map(|&color| {
                let color_mask = &self.color_masks[&color];
                let count = (0..self.states.len())
                    .filter(|&idx| self.states[idx] == CellState::Empty && color_mask[idx])
                    .count();

                if count > 0 {
                    Some((count, color))
                } else {
                    None
                }
            })
            .collect();

        color_sizes.sort_by_key(|&(count, _)| count);

        let mut valid_placements = Vec::with_capacity(self.size * self.size);

        // process colors with fewest empty cells first
        for (_, color) in color_sizes {
            let color_mask = &self.color_masks[&color];

            for r in 0..self.size {
                for c in 0..self.size {
                    let idx = self.index(r, c);
                    if self.states[idx] == CellState::Empty
                        && color_mask[idx]
                        && self.can_place_queen(r, c)
                    {
                        valid_placements.push((r, c));
                    }
                }
            }
        }

        valid_placements
    }

    pub fn is_goal_state(&self) -> bool {
        self.unique_colors.len() == self.colors_with_queens.len()
    }

    pub fn place_queen(&self, r: usize, c: usize) -> Self {
        let mut new_state = self.clone();
        let idx = self.index(r, c);
        let queen_color = self.colors[idx];

        // place the queen
        new_state.states[idx] = CellState::Queen;

        // block row and column
        for i in 0..self.size {
            let row_idx = self.index(r, i);
            let col_idx = self.index(i, c);

            if new_state.states[row_idx] == CellState::Empty {
                new_state.states[row_idx] = CellState::Blocked;
            }

            if new_state.states[col_idx] == CellState::Empty {
                new_state.states[col_idx] = CellState::Blocked;
            }
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
                    let neighbor_idx = self.index(nr as usize, nc as usize);
                    if new_state.states[neighbor_idx] == CellState::Empty {
                        new_state.states[neighbor_idx] = CellState::Blocked;
                    }
                }
            }
        }

        // block entire color region
        let color_mask = &self.color_masks[&queen_color];
        for idx in 0..new_state.states.len() {
            if color_mask[idx] && new_state.states[idx] == CellState::Empty {
                new_state.states[idx] = CellState::Blocked;
            }
        }

        // mark this color as having a queen
        new_state.colors_with_queens.insert(queen_color);

        new_state
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

#[cfg(test)]
mod test;
