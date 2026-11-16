//! Color-region input types.

use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// A square grid of color regions, indexed `regions[row][col]`.
#[derive(Tsify, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridInput {
    /// `regions[row][col]` is the zero-based color index of that cell.
    pub regions: Vec<Vec<u32>>,
}
