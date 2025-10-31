use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use queen_sweep_core::{GameState, depth_first_search, heuristic::*};

#[wasm_bindgen]
pub struct QueensGame(GameState);

#[wasm_bindgen]
impl QueensGame {
    #[wasm_bindgen(constructor)]
    pub fn from_color_regions(color_regions: Vec<Uint8Array>) -> Result<QueensGame, JsValue> {
        let regions: Vec<Vec<u8>> = color_regions.iter().map(|arr| arr.to_vec()).collect();

        let inner = GameState::from_color_regions(regions, Some(smallest_region_by_empty_cells))
            .map_err(|e| JsError::new(&e.to_string()))?;

        Ok(QueensGame(inner))
    }

    #[wasm_bindgen]
    pub fn solve(&self) -> Option<QueensGame> {
        let (solution_opt, _steps) = depth_first_search(self.0.clone());
        solution_opt.map(QueensGame)
    }

    #[wasm_bindgen]
    pub fn get_queen_positions(&self) -> Vec<Uint8Array> {
        let positions = self.0.queen_positions();

        positions
            .into_iter()
            .map(|(r, c)| {
                let arr = vec![r as u8, c as u8];
                Uint8Array::from(arr.as_slice())
            })
            .collect()
    }

    #[wasm_bindgen]
    pub fn get_states(&self) -> Vec<u8> {
        self.0.states.iter().map(|&s| s as u8).collect()
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
}
