use std::hash::{Hash, Hasher};

use crate::CellState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub size: usize,
    pub states: Vec<CellState>,
    pub colors: Vec<u8>,
    colors_with_queens: Vec<bool>,
    color_masks: Vec<Vec<bool>>,
}

impl GameState {
    #[inline]
    pub fn pos_to_index(&self, pos: (usize, usize)) -> usize {
        pos.0 * self.size + pos.1
    }

    #[inline]
    pub fn index_to_pos(&self, index: usize) -> (usize, usize) {
        let r = index / self.size;
        let c = index % self.size;

        (r, c)
    }

    pub fn get_queen_positions(&self) -> Vec<(usize, usize)> {
        self.states
            .iter()
            .enumerate()
            .filter_map(|(i, state)| {
                if *state == CellState::Queen {
                    Some(self.index_to_pos(i))
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

#[cfg(test)]
mod test;
