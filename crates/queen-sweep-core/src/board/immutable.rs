use crate::primitives::{Color, ValidBoardSize};

#[derive(Debug)]
pub struct ImmutableBoardData<const N: usize>
where
    [(); N]: ValidBoardSize,
{
    color_regions: [[Color; N]; N],

    color_masks: [[[bool; N]; N]; N],
}

impl<const N: usize> ImmutableBoardData<N>
where
    [(); N]: ValidBoardSize,
{
    pub(super) fn new(color_regions: [[Color; N]; N]) -> Self {
        let mut color_masks = [[[false; N]; N]; N];

        for color in 0..N {
            for row in 0..N {
                for col in 0..N {
                    let is_color_matching = color_regions[row][col] == color.into();
                    color_masks[color][row][col] = is_color_matching
                }
            }
        }

        Self {
            color_regions,
            color_masks,
        }
    }
}
