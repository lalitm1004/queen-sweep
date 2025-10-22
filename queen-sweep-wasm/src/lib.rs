use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use queen_sweep_core::{GameState, depth_first_search};

#[wasm_bindgen]
pub struct GameStateWasm(GameState);

#[wasm_bindgen]
impl GameStateWasm {
    #[wasm_bindgen(constructor)]
    pub fn from_color_regions(color_regions: Vec<Uint8Array>) -> GameStateWasm {
        let regions: Vec<Vec<u8>> = color_regions.iter().map(|arr| arr.to_vec()).collect();
        let inner = GameState::from_color_regions(regions);
        GameStateWasm(inner)
    }

    #[wasm_bindgen]
    pub fn solve(&self) -> Option<GameStateWasm> {
        depth_first_search(self.0.clone()).map(|gs| GameStateWasm(gs))
    }

    #[wasm_bindgen]
    pub fn get_states(&self) -> Vec<u8> {
        self.0.states.iter().map(|&s| s as u8).collect()
    }

    #[wasm_bindgen]
    pub fn get_colors(&self) -> Vec<u8> {
        self.0.colors.clone()
    }

    #[wasm_bindgen]
    pub fn get_size(&self) -> usize {
        self.0.size
    }

    #[wasm_bindgen]
    pub fn get_states_2d(&self) -> Vec<Uint8Array> {
        let size = self.0.size;

        let mut rows = Vec::with_capacity(size);
        for r in 0..size {
            let start = r * size;
            let end = start + size;
            let row = self.0.states[start..end]
                .iter()
                .map(|&s| s as u8)
                .collect::<Vec<u8>>();

            rows.push(Uint8Array::from(row.as_slice()));
        }

        rows
    }

    #[wasm_bindgen]
    pub fn get_colors_2d(&self) -> Vec<Uint8Array> {
        let size = self.0.size;

        let mut rows = Vec::with_capacity(size);
        for r in 0..size {
            let start = r * size;
            let end = start + size;
            let row = self.0.colors[start..end]
                .iter()
                .map(|&s| s as u8)
                .collect::<Vec<u8>>();

            rows.push(Uint8Array::from(row.as_slice()));
        }

        rows
    }
}
