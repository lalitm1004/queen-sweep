use std::rc::Rc;

use crate::CellState;

#[allow(dead_code)]
pub struct HeuristicContext<'a> {
    pub indexes: &'a [usize],
    pub size: usize,
    pub states: &'a [CellState],
    pub color_masks: &'a [Rc<[bool]>],
    pub colors_with_queens: &'a [bool],
}
pub type HeuristicFn = fn(&HeuristicContext) -> Vec<f32>;
