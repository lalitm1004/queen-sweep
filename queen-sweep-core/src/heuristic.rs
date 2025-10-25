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
