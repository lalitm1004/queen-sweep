use queen_sweep_core::GameState;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmGameState {
    inner: GameState,
}

#[wasm_bindgen]
impl WasmGameState {
    #[wasm_bindgen(constructor)]
    pub fn from_color_regions(color_regions: JsValue) -> WasmGameState {
        let regions: Vec<Vec<u8>> = from_value(color_regions).unwrap();
        let inner = GameState::from_color_regions(regions);
        WasmGameState { inner }
    }

    #[wasm_bindgen]
    pub fn solve(&self) -> Option<WasmGameState> {
        GameState::depth_first_search(self.inner.clone()).map(|gs| WasmGameState { inner: gs })
    }

    #[wasm_bindgen]
    pub fn get_states(&self) -> Vec<u8> {
        self.inner.states.iter().map(|&s| s as u8).collect()
    }

    #[wasm_bindgen]
    pub fn get_colors(&self) -> Vec<u8> {
        self.inner.colors.clone()
    }

    #[wasm_bindgen]
    pub fn get_size(&self) -> usize {
        self.inner.size
    }

    #[wasm_bindgen]
    pub fn get_states_2d(&self) -> JsValue {
        let size = self.inner.size;
        let mut grid = Vec::with_capacity(size);
        for r in 0..size {
            let row: Vec<u8> = (0..size)
                .map(|c| self.inner.states[r * size + c] as u8)
                .collect();
            grid.push(row);
        }
        to_value(&grid).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_colors_2d(&self) -> JsValue {
        let size = self.inner.size;
        let mut grid = Vec::with_capacity(size);
        for r in 0..size {
            let row: Vec<u8> = (0..size).map(|c| self.inner.colors[r * size + c]).collect();
            grid.push(row);
        }
        to_value(&grid).unwrap()
    }
}
