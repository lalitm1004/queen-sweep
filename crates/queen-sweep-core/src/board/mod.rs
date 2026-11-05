use crate::primitives::{Color, ValidBoardSize};

mod hash;
mod immutable;

pub struct QueensBoard<const N: usize>
where
    [(); N]: ValidBoardSize,
{
    blocked_mask: [[bool; N]; N],

    queen_col_per_row: [Option<usize>; N],

    queen_present_by_color: [bool; N],

    hash: u64,

    immutable_board_data: std::rc::Rc<immutable::ImmutableBoardData<N>>,
}

impl<const N: usize> QueensBoard<N>
where
    [(); N]: ValidBoardSize,
{
    pub unsafe fn new_unchecked(color_regions: [[Color; N]; N]) -> Self {
        let blocked_mask = [[false; N]; N];
        let queen_col_per_row = [None; N];
        let queen_present_by_color = [false; N];

        let hash = hash::compute_board_hash(&queen_col_per_row);

        let immutable_board_data = immutable::ImmutableBoardData::new(color_regions);
        let immutable_board_data = std::rc::Rc::new(immutable_board_data);

        Self {
            blocked_mask,
            queen_col_per_row,
            queen_present_by_color,
            hash,
            immutable_board_data,
        }
    }
}
