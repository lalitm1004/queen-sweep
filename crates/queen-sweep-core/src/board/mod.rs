use crate::primitives::{Color, ValidBoardSize};

mod hash;
mod immutable;

#[derive(Debug, Clone)]
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

/// Equality for `QueensBoard` is defined solely in terms of queen placement,
/// represented by `queen_col_per_row`.
///
/// Two boards are considered equal if and only if their queen positions are
/// identical for every row. The following are intentionally *not* considered:
///
/// - `blocked_mask`,
/// - `queen_present_by_color`,
/// - `hash`,
/// - `immutable_board_data`.
///
/// # Rationale
///
/// This implementation exists primarily to support use in `HashMap`/`HashSet`
/// during solution search.
///
/// Within a single search:
///
/// - All boards have the same `N`.
/// - All boards share the same color-region configuration
///   (`immutable_board_data`).
///
/// Given those invariants:
///
/// - Row/column/neighbor blocking is fully determined by queen positions.
/// - Color-region blocking is fixed by the shared immutable data.
/// - `queen_present_by_color` can be reconstructed from queen placement.
/// - `blocked_mask` is therefore a derived view of queen positions.
///
/// As a result, `queen_col_per_row` is a canonical representation of the
/// board state for equality and hashing purposes.
///
/// # Important
///
/// This definition assumes boards being compared originate from the same
/// search space (same `N` and same region layout). Comparing boards across
/// different region configurations would be semantically invalid.
impl<const N: usize> PartialEq for QueensBoard<N>
where
    [(); N]: ValidBoardSize,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.queen_col_per_row == other.queen_col_per_row
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.queen_col_per_row != other.queen_col_per_row
    }
}

/// Marker trait indicating that `QueensBoard` has total equality,
/// consistent with its `PartialEq` implementation.
///
/// Equality is defined exclusively by `queen_col_per_row`, meaning two
/// boards are equal if and only if their queen placements are identical.
///
/// This is sound because, within a given search:
///
/// - All boards share the same board size `N`.
/// - All boards share the same immutable color-region configuration.
/// - All blocked cells and derived state are fully determined by
///   queen placement and the shared immutable data.
///
/// Therefore, equality over queen positions implies full board-state
/// equivalence in the intended usage context.
///
/// This implementation exists primarily to allow `QueensBoard` to be
/// used as a key in `HashMap` and `HashSet`.
impl<const N: usize> Eq for QueensBoard<N> where [(); N]: ValidBoardSize {}

/// Hashes the board using its cached 64-bit state value.
///
/// The cached `u64` is deterministically derived from the per-row
/// queen positions. For small boards it is collision-free; for larger
/// boards it uses a 64-bit FNV-1a hash.
///
/// Since queen placement uniquely determines the logical board state,
/// hashing this value is sufficient and consistent with `PartialEq`.
impl<const N: usize> std::hash::Hash for QueensBoard<N>
where
    [(); N]: ValidBoardSize,
{
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}
