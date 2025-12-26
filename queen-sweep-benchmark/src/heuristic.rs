#[derive(Debug, Clone, Copy)]
pub enum Heuristic {
    None,
    SmallestRegionFirst,
    SmallestRegionByEmptyCells,
}

impl Heuristic {
    pub fn name(&self) -> &'static str {
        match self {
            Heuristic::None => "no-heuristic",
            Heuristic::SmallestRegionFirst => "smallest-region-first",
            Heuristic::SmallestRegionByEmptyCells => "smallest-region-by-empty-cells",
        }
    }

    pub fn all() -> Vec<Heuristic> {
        vec![
            Heuristic::None,
            Heuristic::SmallestRegionFirst,
            Heuristic::SmallestRegionByEmptyCells,
        ]
    }

    pub fn to_fn(&self) -> Option<queen_sweep_core::heuristic::HeuristicFn> {
        match self {
            Heuristic::None => None,
            Heuristic::SmallestRegionFirst => {
                Some(queen_sweep_core::heuristic::smallest_region_first)
            }
            Heuristic::SmallestRegionByEmptyCells => {
                Some(queen_sweep_core::heuristic::smallest_region_by_empty_cells)
            }
        }
    }
}
