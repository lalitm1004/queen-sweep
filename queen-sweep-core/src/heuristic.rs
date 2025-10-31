use std::rc::Rc;

use crate::CellState;
use queen_sweep_heuristic_macros::heuristic;

pub struct HeuristicContext<'a> {
    pub positions: &'a [(usize, usize)],
    pub size: usize,
    pub states: &'a [CellState],
    pub colors_with_queens: &'a [bool],
    pub colors: &'a [u8],
    pub color_masks: &'a [Rc<[bool]>],
}
pub type HeuristicFn = fn(&HeuristicContext) -> Vec<((usize, usize), f32)>;

/// Prioritizes smaller color regions.
/// Counts all cells of a color regardless of cell state.
#[heuristic]
pub fn smallest_region_first(ctx: &HeuristicContext) -> Vec<((usize, usize), f32)> {
    let mut color_sizes = vec![0usize; ctx.size];

    for &color in ctx.colors.iter() {
        color_sizes[color as usize] += 1;
    }

    ctx.positions
        .iter()
        .map(|&(r, c)| {
            let idx = r * ctx.size + c;
            let color = ctx.colors[idx] as usize;
            ((r, c), color_sizes[color] as f32)
        })
        .collect()
}

/// Prioritizes regions with fewer empty cells.
/// Counts only empty cells in each color region
#[heuristic]
pub fn smallest_region_by_empty_cells(ctx: &HeuristicContext) -> Vec<((usize, usize), f32)> {
    let mut color_empty_counts = vec![0usize; ctx.size];

    for (idx, state) in ctx.states.iter().enumerate() {
        if *state == CellState::Empty {
            let color = ctx.colors[idx] as usize;
            color_empty_counts[color] += 1;
        }
    }

    ctx.positions
        .iter()
        .map(|&(r, c)| {
            let idx = r * ctx.size + c;
            let color = ctx.colors[idx] as usize;
            ((r, c), color_empty_counts[color] as f32)
        })
        .collect()
}
