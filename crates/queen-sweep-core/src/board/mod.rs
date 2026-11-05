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

#[cfg(test)]
mod tests {
    use super::QueensBoard;
    use crate::primitives::Color;
    use std::collections::{HashMap, HashSet};
    use std::hash::{DefaultHasher, Hash, Hasher};

    fn create_color_regions<const N: usize>() -> [[Color; N]; N] {
        std::array::from_fn(|row| std::array::from_fn(|col| Color::from((row + col) % N)))
    }

    fn create_board<const N: usize>() -> QueensBoard<N>
    where
        [(); N]: crate::primitives::ValidBoardSize,
    {
        unsafe { QueensBoard::new_unchecked(create_color_regions::<N>()) }
    }

    fn calculate_hash<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn test_new_unchecked_initializes_board() {
        let board = create_board::<4>();

        assert_eq!(board.blocked_mask, [[false; 4]; 4]);
        assert_eq!(board.queen_col_per_row, [None; 4]);
        assert_eq!(board.queen_present_by_color, [false; 4]);
    }

    #[test]
    fn test_new_unchecked_initializes_hash() {
        let board = create_board::<4>();

        let expected_hash = super::hash::compute_board_hash(&[None; 4]);

        assert_eq!(board.hash, expected_hash);
    }

    #[test]
    fn test_new_unchecked_initializes_immutable_data() {
        let regions = create_color_regions::<4>();

        let board = unsafe { QueensBoard::new_unchecked(regions) };

        assert_eq!(board.immutable_board_data.get_color_regions(), &regions);
    }

    #[test]
    fn test_new_unchecked_initializes_color_masks() {
        let board = create_board::<4>();

        for color in 0usize..4 {
            let mask = board.immutable_board_data.get_color_mask(color);

            for row in 0..4 {
                for col in 0..4 {
                    assert_eq!(
                        mask[row][col],
                        usize::from(
                            board
                                .immutable_board_data
                                .get_color_at_pos(crate::primitives::Position::new(row, col))
                        ) == color
                    );
                }
            }
        }
    }

    #[test]
    fn test_clone_board() {
        let board = create_board::<4>();
        let cloned = board.clone();

        assert_eq!(board, cloned);
        assert_eq!(board.blocked_mask, cloned.blocked_mask);
        assert_eq!(board.queen_col_per_row, cloned.queen_col_per_row);
        assert_eq!(board.queen_present_by_color, cloned.queen_present_by_color);
        assert_eq!(board.hash, cloned.hash);
    }

    #[test]
    fn test_clone_shares_immutable_data() {
        let board = create_board::<4>();
        let cloned = board.clone();

        assert!(std::rc::Rc::ptr_eq(
            &board.immutable_board_data,
            &cloned.immutable_board_data
        ));
    }

    #[test]
    fn test_board_equality() {
        let board1 = create_board::<4>();
        let board2 = create_board::<4>();

        assert_eq!(board1, board2);
        assert!(!(board1 != board2));
    }

    #[test]
    fn test_board_inequality() {
        let mut board1 = create_board::<4>();
        let mut board2 = create_board::<4>();

        board1.queen_col_per_row[0] = Some(1);
        board2.queen_col_per_row[0] = Some(2);

        assert_ne!(board1, board2);
    }

    #[test]
    fn test_board_equality_depends_only_on_queen_positions() {
        let mut board1 = create_board::<4>();
        let mut board2 = create_board::<4>();

        board1.blocked_mask[0][0] = true;
        board2.blocked_mask[1][1] = true;

        board1.queen_present_by_color[0] = true;
        board2.queen_present_by_color[1] = true;

        board1.hash = 100;
        board2.hash = 200;

        assert_eq!(board1, board2);
    }

    #[test]
    fn test_board_equality_with_different_queen_rows() {
        let mut board1 = create_board::<4>();
        let mut board2 = create_board::<4>();

        board1.queen_col_per_row[0] = Some(1);
        board2.queen_col_per_row[1] = Some(1);

        assert_ne!(board1, board2);
    }

    #[test]
    fn test_board_equality_with_partial_queen_placement() {
        let mut board1 = create_board::<4>();
        let mut board2 = create_board::<4>();

        board1.queen_col_per_row[0] = Some(1);
        board1.queen_col_per_row[2] = Some(3);

        board2.queen_col_per_row[0] = Some(1);
        board2.queen_col_per_row[2] = Some(3);

        assert_eq!(board1, board2);
    }

    #[test]
    fn test_board_hash() {
        let board1 = create_board::<4>();
        let board2 = create_board::<4>();

        assert_eq!(calculate_hash(&board1), calculate_hash(&board2));
    }

    #[test]
    fn test_equal_boards_have_equal_hashes() {
        let mut board1 = create_board::<4>();
        let mut board2 = create_board::<4>();

        board1.queen_col_per_row[0] = Some(1);
        board2.queen_col_per_row[0] = Some(1);

        assert_eq!(board1, board2);
        assert_eq!(calculate_hash(&board1), calculate_hash(&board2));
    }

    #[test]
    fn test_board_hash_changes_with_queen_placement() {
        let mut board1 = create_board::<4>();
        let mut board2 = create_board::<4>();

        board1.queen_col_per_row[0] = Some(1);
        board2.queen_col_per_row[0] = Some(2);

        board1.hash = super::hash::compute_board_hash(&board1.queen_col_per_row);

        board2.hash = super::hash::compute_board_hash(&board2.queen_col_per_row);

        assert_ne!(board1.hash, board2.hash);
    }

    #[test]
    fn test_hashset_deduplicates_equal_boards() {
        let board1 = create_board::<4>();
        let board2 = create_board::<4>();

        let mut boards = HashSet::new();

        boards.insert(board1);
        boards.insert(board2);

        assert_eq!(boards.len(), 1);
    }

    #[test]
    fn test_hashset_stores_different_boards() {
        let mut board1 = create_board::<4>();
        let mut board2 = create_board::<4>();

        board1.queen_col_per_row[0] = Some(1);
        board2.queen_col_per_row[0] = Some(2);

        board1.hash = super::hash::compute_board_hash(&board1.queen_col_per_row);

        board2.hash = super::hash::compute_board_hash(&board2.queen_col_per_row);

        let mut boards = HashSet::new();

        boards.insert(board1);
        boards.insert(board2);

        assert_eq!(boards.len(), 2);
    }

    #[test]
    fn test_hashmap_board_keys() {
        let board1 = create_board::<4>();
        let board2 = create_board::<4>();

        let mut map = HashMap::new();

        map.insert(board1, "initial");

        assert_eq!(map.get(&board2), Some(&"initial"));
    }

    #[test]
    fn test_larger_board() {
        let board = create_board::<8>();

        assert_eq!(board.blocked_mask, [[false; 8]; 8]);
        assert_eq!(board.queen_col_per_row, [None; 8]);
        assert_eq!(board.queen_present_by_color, [false; 8]);
    }
}
