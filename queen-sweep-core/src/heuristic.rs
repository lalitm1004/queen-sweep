use std::rc::Rc;

use crate::CellState;

pub struct HeuristicContext<'a> {
    pub positions: &'a [(usize, usize)],
    pub size: usize,
    pub states: &'a [CellState],
    pub colors_with_queens: &'a [bool],
    pub colors: &'a [u8],
    pub color_masks: &'a [Rc<[bool]>],
}
pub type HeuristicFn = fn(&HeuristicContext) -> Vec<f32>;

/// Prioritizes smaller color regions.
/// Counts all cells of a color regardless of cell state.
pub fn smallest_region_first(ctx: &HeuristicContext) -> Vec<f32> {
    let mut color_sizes = vec![0usize; ctx.size];

    for &color in ctx.colors.iter() {
        color_sizes[color as usize] += 1
    }

    ctx.positions
        .iter()
        .map(|&(r, c)| {
            let idx = r * ctx.size + c;
            let color = ctx.colors[idx] as usize;
            color_sizes[color] as f32
        })
        .collect()
}

/// Prioritizes regions with fewer empty cells.
/// Counts only empty cells in each color region
pub fn smallest_region_by_empty_cells(ctx: &HeuristicContext) -> Vec<f32> {
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
            color_empty_counts[color] as f32
        })
        .collect()
}

/// Prioritizes positions in colors with very few empty cells.
/// The heuristic value is inversely proportional to the number of empty cells in the region,
/// making cells in almost-full regions higher priority.
pub fn critical_empty_cell_first(ctx: &HeuristicContext) -> Vec<f32> {
    let mut region_empty_counts = vec![0usize; ctx.size];

    for (idx, state) in ctx.states.iter().enumerate() {
        if *state == CellState::Empty {
            let color = ctx.colors[idx] as usize;
            region_empty_counts[color] += 1;
        }
    }

    ctx.positions
        .iter()
        .map(|&(r, c)| {
            let idx = r * ctx.size + c;
            let color = ctx.colors[idx] as usize;
            1.0 / (region_empty_counts[color].max(1) as f32)
        })
        .collect()
}
