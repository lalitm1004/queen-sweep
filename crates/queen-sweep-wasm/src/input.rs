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

#[cfg(test)]
mod tests {
    use super::GridInput;

    #[test]
    fn grid_input_roundtrips_through_serde() {
        let input = GridInput {
            regions: vec![vec![0, 0, 1], vec![0, 1, 1], vec![2, 2, 2]],
        };

        let json = serde_json::to_string(&input).unwrap();
        let back: GridInput = serde_json::from_str(&json).unwrap();

        assert_eq!(back.regions, input.regions);
    }

    #[test]
    fn grid_input_deserializes_from_json() {
        // serde field is `regions`; the JSON key must match.
        let json = r#"{"regions":[[0,0],[1,1]]}"#;
        let input: GridInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.regions, vec![vec![0, 0], vec![1, 1]]);
    }
}
