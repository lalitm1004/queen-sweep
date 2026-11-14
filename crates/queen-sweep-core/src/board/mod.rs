//! The [`QueensBoard`] state type and its supporting logic.
//!
//! A [`QueensBoard`] tracks the queens placed so far and derives the set of
//! blocked cells from them, combined with an immutable, shared color-region
//! layout. It is the central state type consumed and produced by the
//! [`crate::solver`].

use crate::primitives::{Cell, Color, Position, ValidBoardSize};

mod error;
mod hash;
mod immutable;

pub use error::{BoardInitError, QueensPlacementError};

/// A partially or fully solved Queens board of size `N`.
///
/// The board is immutable in spirit: mutating operations (placing a queen)
/// return a new [`QueensBoard`] rather than modifying the receiver in place.
///
/// # Representation
///
/// - `blocked_mask`: which cells may no longer receive a queen.
/// - `queen_col_per_row`: the column of the queen in each row, if any.
/// - `queen_present_by_color`: whether each color region already has a queen.
/// - `hash`: a cached hash of the queen placement (deterministic; equal boards
///   have equal hashes).
/// - `immutable_board_data`: the shared color-region layout.
///
/// The `blocked_mask` is always a derived view: it is fully determined by the
/// queen placements and the (shared) color-region layout, and is maintained so
/// that every unblocked cell is a currently valid placement.
///
/// # Equality
///
/// Two boards compare equal when their queen placements are identical, ignoring
/// the derived `blocked_mask`, `queen_present_by_color`, `hash`, and
/// `immutable_board_data`. See the [`PartialEq`] implementation for details.
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
    /// Creates an empty `QueensBoard` with the given color regions.
    ///
    /// Validates that all color indices are within `0..N` and that exactly `N`
    /// distinct colors are present before initializing the board.
    ///
    /// # Errors
    ///
    /// Returns [`BoardInitError::ColorOutOfRange`] if any color index is greater
    /// than or equal to `N`.
    ///
    /// Returns [`BoardInitError::InvalidColorCount`] if the color regions do not
    /// contain exactly `N` distinct colors.
    ///
    /// [`QueensBoard::new_unchecked`].
    pub fn new(color_regions: [[Color; N]; N]) -> Result<Self, BoardInitError> {
        let mut color_present = [false; N];
        let mut distinct_color_count = 0;

        for row in &color_regions {
            for &color in row {
                let color: usize = color.into();

                if color >= N {
                    return Err(BoardInitError::ColorOutOfRange {
                        color,
                        max_colors: N,
                    });
                }

                if !color_present[color] {
                    color_present[color] = true;
                    distinct_color_count += 1;
                }
            }
        }

        if distinct_color_count != N {
            return Err(BoardInitError::InvalidColorCount {
                expected: N,
                found: distinct_color_count,
            });
        }

        // SAFETY: Validation is done before calling unchecked builder
        Ok(unsafe { Self::new_unchecked(color_regions) })
    }

    /// Creates an empty `QueensBoard` with the given color regions, without
    /// validating the region data.
    ///
    /// The board assumes exactly `N` distinct color indices (`0..N`) and stores
    /// per-color state in fixed-size arrays indexed by those values (for
    /// example `queen_present_by_color` and the precomputed color masks).
    ///
    /// # Safety
    ///
    /// The caller must guarantee that every `Color` in `color_regions` has a
    /// value strictly less than `N`. A color value of `N` or greater will later
    /// cause out-of-bounds indexing when placing queens or querying color data.
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

    /// Returns the board size `N`.
    #[inline]
    pub const fn get_size(&self) -> usize {
        N
    }

    /// Returns the current blocked-cell mask.
    ///
    /// A `true` entry means the cell may not receive a queen.
    #[inline]
    pub const fn get_blocked_mask(&self) -> &[[bool; N]; N] {
        &self.blocked_mask
    }

    /// Returns the per-row queen columns.
    ///
    /// Entry `r` is `Some(col)` if row `r` contains a queen in column `col`,
    /// or `None` if the row has no queen yet.
    #[inline]
    pub const fn get_queen_col_per_row(&self) -> &[Option<usize>; N] {
        &self.queen_col_per_row
    }

    /// Returns the cached hash of the queen placement.
    ///
    /// The hash is deterministic and consistent with [`PartialEq`]: equal
    /// boards have equal hashes.
    #[inline]
    pub const fn get_hash(&self) -> u64 {
        self.hash
    }

    /// Returns the color of the region containing `pos`.
    #[inline]
    pub fn get_color_at_pos(&self, pos: Position) -> Color {
        self.immutable_board_data.get_color_at_pos(pos)
    }

    /// Returns the boolean mask of the given color region.
    ///
    /// The returned `N×N` array has a `true` entry at every cell belonging to
    /// the color.
    #[inline]
    pub fn get_color_mask(&self, color: impl Into<usize>) -> &[[bool; N]; N] {
        &self.immutable_board_data.get_color_mask(color)
    }

    /// Returns the full color-region layout: `color_regions[row][col]` is the
    /// color of the cell at that position.
    #[inline]
    pub fn get_color_regions(&self) -> &[[Color; N]; N] {
        &self.immutable_board_data.get_color_regions()
    }

    /// Returns the board as a grid of [`Cell`] values.
    ///
    /// Each cell is [`Cell::Empty`], [`Cell::Blocked`], or [`Cell::Queen`]
    /// according to the current board state.
    pub fn get_cells(&self) -> [[Cell; N]; N] {
        let mut cells = [[Cell::Blocked; N]; N];

        for row in 0..N {
            let queen_col = self.queen_col_per_row[row];

            for col in 0..N {
                // cell is of type empty
                if !self.blocked_mask[row][col] {
                    cells[row][col] = Cell::Empty;
                    continue;
                }

                // cell is of type queen
                if queen_col == Some(col) {
                    cells[row][col] = Cell::Queen;
                }
            }
        }

        cells
    }

    pub fn get_queen_positions(&self) -> Vec<Position> {
        let mut positions = Vec::with_capacity(N);

        for row in 0..N {
            if let Some(queen_col) = self.queen_col_per_row[row] {
                positions.push(Position::new(row, queen_col));
            }
        }

        positions
    }

    /// Returns `true` if the given color region already contains a queen.
    pub fn queen_present_by_color(&self, color: impl Into<usize>) -> bool {
        self.queen_present_by_color[color.into()]
    }

    /// Returns `true` if every row contains a queen.
    ///
    /// A solved board has exactly one queen per row; because queens also
    /// enforce one-per-column and one-per-color during placement, this implies
    /// the board is a complete, valid solution.
    pub fn is_solved(&self) -> bool {
        self.queen_col_per_row.iter().all(|c| c.is_some())
    }

    /// Checks whether a queen may be placed at `pos` given the provided
    /// `blocked_mask` and `queen_present_by_color`.
    ///
    /// This simulates the blocking a queen at `pos` would cause and rejects the
    /// placement if it would strand any not-yet-filled color region (leaving it
    /// with no remaining valid cell). It is used by
    /// [`perform_exhaustive_blocking`](Self::perform_exhaustive_blocking) to
    /// prune cells that cannot be part of any solution.
    fn placement_is_valid(
        &self,
        blocked_mask: &[[bool; N]; N],
        queen_present_by_color: &[bool; N],
        pos: Position,
    ) -> bool {
        let (r, c) = pos.to_tuple();

        // cell must be empty
        if blocked_mask[r][c] {
            return false;
        }

        // color must not have a queen placed already
        let color = self.get_color_at_pos(pos);
        if queen_present_by_color[usize::from(color)] {
            return false;
        }

        // simulate blocking
        let mut blocked_mask = blocked_mask.clone();

        // block row & col
        for i in 0..N {
            blocked_mask[r][i] = true;
            blocked_mask[i][c] = true;
        }

        // block neighbors
        for (dr, dc) in NEIGHBOR_DELTAS {
            if let Some(pos) = pos.displace::<N>(dr, dc) {
                blocked_mask[pos.row()][pos.col()] = true;
            }
        }

        // block color region
        let color_mask = self.get_color_mask(color);
        for r in 0..N {
            for c in 0..N {
                if color_mask[r][c] {
                    blocked_mask[r][c] = true;
                }
            }
        }

        // check if placement leaves any region unsolvable
        for other_color in 0..N {
            if other_color == color.into() || queen_present_by_color[other_color] {
                continue;
            }

            let other_color_mask = self.get_color_mask(other_color);
            let mut has_valid_cell = false;

            'outer: for row in 0..N {
                for col in 0..N {
                    if other_color_mask[row][col] && !blocked_mask[row][col] {
                        has_valid_cell = true;
                        break 'outer;
                    }
                }
            }

            if !has_valid_cell {
                return false;
            }
        }

        true
    }

    /// Repeatedly blocks every cell that cannot be part of any solution, until
    /// a fixpoint is reached.
    ///
    /// On each pass it marks any unblocked cell for which
    /// [`placement_is_valid`](Self::placement_is_valid) returns `false` as
    /// blocked, and stops once a full pass changes nothing. This is applied
    /// automatically by [`place_queen`](Self::place_queen) and
    /// [`place_queen_unchecked`](Self::place_queen_unchecked), so the returned
    /// board's `blocked_mask` is always a fixpoint.
    fn perform_exhaustive_blocking(self) -> Self {
        let mut new_blocked_mask = self.blocked_mask.clone();

        loop {
            let mut blocked_any = false;

            for row in 0..N {
                for col in 0..N {
                    let pos = Position::new(row, col);

                    if !new_blocked_mask[row][col]
                        && !self.placement_is_valid(
                            &new_blocked_mask,
                            &self.queen_present_by_color,
                            pos,
                        )
                    {
                        new_blocked_mask[row][col] = true;
                        blocked_any = true;
                    }
                }
            }

            if !blocked_any {
                break;
            }
        }

        Self {
            blocked_mask: new_blocked_mask,
            queen_col_per_row: self.queen_col_per_row,
            queen_present_by_color: self.queen_present_by_color,
            hash: self.hash,
            immutable_board_data: std::rc::Rc::clone(&self.immutable_board_data),
        }
    }

    /// Places a queen at `pos`, returning the new board.
    ///
    /// # Errors
    ///
    /// Returns [`QueensPlacementError::OutOfBounds`] if `pos` lies outside the
    /// board, or [`QueensPlacementError::NonEmptyPlacementTarget`] if the cell
    /// is already blocked.
    ///
    /// On success the returned board has recorded the queen, applied all
    /// blocking, recomputed its hash, and run exhaustive blocking.
    pub fn place_queen(&self, pos: Position) -> Result<Self, QueensPlacementError> {
        let (r, c) = (pos.row(), pos.col());

        if r >= N || c >= N {
            return Err(QueensPlacementError::OutOfBounds { r, c });
        }

        if self.blocked_mask[r][c] {
            return Err(QueensPlacementError::NonEmptyPlacementTarget { r, c });
        }

        // SAFETY: `pos` was verified to be in bounds and unblocked by the
        // checks above, satisfying the preconditions of `place_queen_unchecked`.
        let new_board = unsafe { self.place_queen_unchecked(pos) };
        Ok(new_board)
    }

    /// Places a queen at `pos` without validating the placement.
    ///
    /// Unlike [`place_queen`](Self::place_queen), this performs no bounds or
    /// blocked-cell checks. It records the queen, blocks its row, column,
    /// eight neighbors, and color region, recomputes the hash, and then runs
    /// exhaustive blocking.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that:
    ///
    /// - `pos` is in bounds (`pos.row() < N` and `pos.col() < N`), so that the
    ///   fixed-size arrays are indexed safely.
    /// - The target cell is currently unblocked and its color region does not
    ///   already contain a queen (i.e. `pos` is a valid placement). Otherwise
    ///   the resulting board violates the one-queen-per-row/column/color
    ///   invariants.
    pub unsafe fn place_queen_unchecked(&self, pos: Position) -> Self {
        let mut new_blocked_mask = self.blocked_mask.clone();
        let mut new_queen_col_per_row = self.queen_col_per_row.clone();
        let mut new_queen_present_by_color = self.queen_present_by_color.clone();

        let (row, col) = pos.to_tuple();

        // place queen
        new_queen_col_per_row[row] = Some(col);

        // mark color region as having a queen
        let color = self.get_color_at_pos(pos);
        new_queen_present_by_color[usize::from(color)] = true;

        // block pos and row + col
        for i in 0..N {
            new_blocked_mask[row][i] = true;
            new_blocked_mask[i][col] = true;
        }

        // block 8 neighbors
        for (dr, dc) in NEIGHBOR_DELTAS {
            if let Some(new_pos) = pos.displace::<N>(dr, dc) {
                new_blocked_mask[new_pos.row()][new_pos.col()] = true;
            }
        }

        // block color region
        let color_mask = self.get_color_mask(color);
        for r in 0..N {
            for c in 0..N {
                if color_mask[r][c] {
                    new_blocked_mask[r][c] = true;
                }
            }
        }

        Self {
            blocked_mask: new_blocked_mask,
            queen_col_per_row: new_queen_col_per_row,
            queen_present_by_color: new_queen_present_by_color,
            hash: hash::compute_board_hash(&new_queen_col_per_row),
            immutable_board_data: std::rc::Rc::clone(&self.immutable_board_data),
        }
        .perform_exhaustive_blocking()
    }

    /// Returns every cell that is currently a valid placement, in row-major
    /// order.
    ///
    /// A cell is a valid placement if it is not blocked. Because blocking is
    /// maintained to a fixpoint, this is exactly the set of cells from which a
    /// solution can still be reached.
    pub fn get_valid_placements(&self) -> Vec<Position> {
        let mut positions = Vec::with_capacity(N * N);

        for row in 0..N {
            for col in 0..N {
                if !self.blocked_mask[row][col] {
                    positions.push(Position::new(row, col));
                }
            }
        }

        positions
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

const NEIGHBOR_DELTAS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[cfg(test)]
mod tests {
    use super::{QueensBoard, QueensPlacementError};
    use crate::primitives::{Cell, Color, Position};
    use proptest::prelude::*;
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

    /// A solvable `N = 4` color layout: each color is a 2x2 quadrant.
    ///
    /// One known solution is queens at `(0,1)`, `(1,3)`, `(2,0)`, `(3,2)`.
    fn quadrant_regions() -> [[Color; 4]; 4] {
        [
            [0.into(), 0.into(), 1.into(), 1.into()],
            [0.into(), 0.into(), 1.into(), 1.into()],
            [2.into(), 2.into(), 3.into(), 3.into()],
            [2.into(), 2.into(), 3.into(), 3.into()],
        ]
    }

    fn quadrant_board() -> QueensBoard<4> {
        unsafe { QueensBoard::new_unchecked(quadrant_regions()) }
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

    // ------------------------------------------------------------------
    // place_queen / place_queen_unchecked
    // ------------------------------------------------------------------

    #[test]
    fn test_place_queen_rejects_out_of_bounds() {
        let board = create_board::<4>();

        let err = board.place_queen(Position::new(4, 0)).unwrap_err();
        assert!(matches!(
            err,
            QueensPlacementError::OutOfBounds { r: 4, c: 0 }
        ));

        let err = board.place_queen(Position::new(0, 4)).unwrap_err();
        assert!(matches!(
            err,
            QueensPlacementError::OutOfBounds { r: 0, c: 4 }
        ));
    }

    #[test]
    fn test_place_queen_rejects_blocked_target() {
        let board = create_board::<4>();
        let placed = board.place_queen(Position::new(0, 0)).unwrap();

        // Same row is blocked by the queen.
        let err = placed.place_queen(Position::new(0, 1)).unwrap_err();
        assert!(matches!(
            err,
            QueensPlacementError::NonEmptyPlacementTarget { r: 0, c: 1 }
        ));

        // The queen's own cell is also blocked.
        let err = placed.place_queen(Position::new(0, 0)).unwrap_err();
        assert!(matches!(
            err,
            QueensPlacementError::NonEmptyPlacementTarget { r: 0, c: 0 }
        ));
    }

    #[test]
    fn test_place_queen_records_queen_and_color() {
        let board = create_board::<4>();

        // (1, 2) has color (1 + 2) % 4 == 3 on the diagonal layout.
        let placed = board.place_queen(Position::new(1, 2)).unwrap();

        assert_eq!(placed.queen_col_per_row[1], Some(2));
        assert!(placed.queen_present_by_color(3usize));
        assert!(!placed.queen_present_by_color(0usize));
        assert!(!placed.queen_present_by_color(1usize));
        assert!(!placed.queen_present_by_color(2usize));
    }

    #[test]
    fn test_place_queen_blocks_row_column_and_neighbors() {
        let board = create_board::<4>();
        let placed = board.place_queen(Position::new(1, 1)).unwrap();

        // Row and column are fully blocked.
        for i in 0..4 {
            assert!(placed.blocked_mask[1][i], "row 1 col {i} should be blocked");
            assert!(placed.blocked_mask[i][1], "row {i} col 1 should be blocked");
        }

        // All eight neighbors are blocked.
        for (dr, dc) in super::NEIGHBOR_DELTAS {
            if let Some(pos) = Position::new(1, 1).displace::<4>(dr, dc) {
                assert!(
                    placed.blocked_mask[pos.row()][pos.col()],
                    "neighbor ({}, {}) should be blocked",
                    pos.row(),
                    pos.col()
                );
            }
        }
    }

    #[test]
    fn test_place_queen_blocks_color_region() {
        let board = create_board::<4>();

        // (1, 1) has color 2 on the diagonal layout.
        let placed = board.place_queen(Position::new(1, 1)).unwrap();

        let mask = board.get_color_mask(2usize);
        for row in 0..4 {
            for col in 0..4 {
                if mask[row][col] {
                    assert!(
                        placed.blocked_mask[row][col],
                        "color-region cell ({row}, {col}) should be blocked"
                    );
                }
            }
        }
    }

    #[test]
    fn test_place_queen_recomputes_hash() {
        let board = create_board::<4>();
        let placed = board.place_queen(Position::new(1, 2)).unwrap();

        assert_eq!(
            placed.hash,
            super::hash::compute_board_hash(&placed.queen_col_per_row)
        );
        assert_ne!(placed.hash, board.hash);
    }

    #[test]
    fn test_place_queen_shares_immutable_data() {
        let board = create_board::<4>();
        let placed = board.place_queen(Position::new(0, 0)).unwrap();

        assert!(std::rc::Rc::ptr_eq(
            &board.immutable_board_data,
            &placed.immutable_board_data
        ));
    }

    // ------------------------------------------------------------------
    // placement_is_valid (core validation logic)
    // ------------------------------------------------------------------

    #[test]
    fn test_placement_is_valid_accepts_all_on_empty_board() {
        // On the quadrant layout every cell is a valid first move.
        let board = quadrant_board();
        let blocked_mask = [[false; 4]; 4];
        let queen_present_by_color = [false; 4];

        for row in 0..4 {
            for col in 0..4 {
                assert!(
                    board.placement_is_valid(
                        &blocked_mask,
                        &queen_present_by_color,
                        Position::new(row, col)
                    ),
                    "({row}, {col}) should be valid on an empty board"
                );
            }
        }
    }

    #[test]
    fn test_placement_is_valid_rejects_blocked_cell() {
        let board = create_board::<4>();
        let mut blocked_mask = [[false; 4]; 4];
        blocked_mask[0][0] = true;
        let queen_present_by_color = [false; 4];

        assert!(!board.placement_is_valid(
            &blocked_mask,
            &queen_present_by_color,
            Position::new(0, 0)
        ));
    }

    #[test]
    fn test_placement_is_valid_rejects_occupied_color() {
        let board = create_board::<4>();
        let blocked_mask = [[false; 4]; 4];
        let mut queen_present_by_color = [false; 4];
        queen_present_by_color[0] = true;

        // (0, 0) has color 0, which already has a queen.
        assert!(!board.placement_is_valid(
            &blocked_mask,
            &queen_present_by_color,
            Position::new(0, 0)
        ));
    }

    #[test]
    fn test_placement_is_valid_rejects_stranded_region() {
        let board = quadrant_board();

        // Block all of color 1 except (1, 2). Color 1 = {(0,2), (0,3), (1,2), (1,3)}.
        let mut blocked_mask = [[false; 4]; 4];
        blocked_mask[0][2] = true;
        blocked_mask[0][3] = true;
        blocked_mask[1][3] = true;
        let queen_present_by_color = [false; 4];

        // (2, 2) is color 3; placing it blocks column 2, stranding color 1's
        // last remaining cell (1, 2).
        assert!(!board.placement_is_valid(
            &blocked_mask,
            &queen_present_by_color,
            Position::new(2, 2)
        ));

        // (3, 3) is also color 3 but leaves (1, 2) untouched, so it stays valid.
        assert!(board.placement_is_valid(
            &blocked_mask,
            &queen_present_by_color,
            Position::new(3, 3)
        ));
    }

    // ------------------------------------------------------------------
    // perform_exhaustive_blocking
    // ------------------------------------------------------------------

    #[test]
    fn test_exhaustive_blocking_reaches_fixpoint() {
        let board = create_board::<4>();
        let placed = board.place_queen(Position::new(0, 0)).unwrap();

        // A second pass of exhaustive blocking must change nothing.
        let once_more = placed.clone().perform_exhaustive_blocking();
        assert_eq!(once_more.blocked_mask, placed.blocked_mask);
    }

    #[test]
    fn test_exhaustive_blocking_eliminates_dead_end() {
        // On the quadrant board a queen at (0, 0) cannot lead to a solution:
        // exhaustive blocking must eliminate every remaining placement.
        let board = quadrant_board();
        let placed = board.place_queen(Position::new(0, 0)).unwrap();

        assert!(placed.get_valid_placements().is_empty());
        assert!(placed.blocked_mask.iter().flatten().all(|&blocked| blocked));
    }

    // ------------------------------------------------------------------
    // get_valid_placements / get_cells / is_solved
    // ------------------------------------------------------------------

    #[test]
    fn test_get_valid_placements_matches_blocked_mask() {
        let board = quadrant_board();
        let placed = board.place_queen(Position::new(0, 1)).unwrap();

        let placements = placed.get_valid_placements();

        let mut expected = Vec::new();
        for row in 0..4 {
            for col in 0..4 {
                if !placed.blocked_mask[row][col] {
                    expected.push(Position::new(row, col));
                }
            }
        }

        assert_eq!(placements, expected);
        assert!(
            placements
                .iter()
                .all(|p| !placed.blocked_mask[p.row()][p.col()])
        );
    }

    #[test]
    fn test_get_cells_matches_internal_state() {
        let board = quadrant_board();
        let placed = board.place_queen(Position::new(0, 1)).unwrap();

        let cells = placed.get_cells();

        for row in 0..4 {
            for col in 0..4 {
                let expected = if placed.queen_col_per_row[row] == Some(col) {
                    Cell::Queen
                } else if placed.blocked_mask[row][col] {
                    Cell::Blocked
                } else {
                    Cell::Empty
                };

                assert_eq!(cells[row][col], expected, "cell ({row}, {col})");
            }
        }
    }

    #[test]
    fn test_is_solved_tracks_completion() {
        let board = create_board::<4>();
        assert!(!board.is_solved());

        let mut partial = board.clone();
        partial.queen_col_per_row = [Some(0), Some(1), None, None];
        assert!(!partial.is_solved());

        let mut solved = board.clone();
        solved.queen_col_per_row = [Some(0), Some(1), Some(2), Some(3)];
        assert!(solved.is_solved());
    }

    // ------------------------------------------------------------------
    // End-to-end solve
    // ------------------------------------------------------------------

    #[test]
    fn test_solve_quadrant_board_end_to_end() {
        let board = quadrant_board();
        assert!(!board.is_solved());

        let after_first = board.place_queen(Position::new(0, 1)).unwrap();
        assert_eq!(
            after_first.get_valid_placements(),
            vec![
                Position::new(1, 3),
                Position::new(2, 0),
                Position::new(3, 2)
            ]
        );

        let after_second = after_first.place_queen(Position::new(1, 3)).unwrap();
        assert_eq!(
            after_second.get_valid_placements(),
            vec![Position::new(2, 0), Position::new(3, 2)]
        );

        let after_third = after_second.place_queen(Position::new(2, 0)).unwrap();
        assert_eq!(
            after_third.get_valid_placements(),
            vec![Position::new(3, 2)]
        );

        let solved = after_third.place_queen(Position::new(3, 2)).unwrap();
        assert!(solved.is_solved());
        assert!(solved.get_valid_placements().is_empty());
        assert_eq!(
            solved.queen_col_per_row,
            [Some(1), Some(3), Some(0), Some(2)]
        );

        // Every color ends up with exactly one queen.
        for color in 0usize..4 {
            assert!(solved.queen_present_by_color(color));
        }
    }

    // ------------------------------------------------------------------
    // Property-based tests
    // ------------------------------------------------------------------

    /// Builds an `N×N` color-region grid from a flat `N*N` slice.
    fn regions_from_flat<const N: usize>(flat: &[usize]) -> [[Color; N]; N] {
        std::array::from_fn(|row| std::array::from_fn(|col| Color::from(flat[row * N + col])))
    }

    /// Places the first `n` valid placements (row-major order) via `place_queen`,
    /// producing an arbitrarily-developed board state.
    fn place_first_n<const N: usize>(mut board: QueensBoard<N>, n: usize) -> QueensBoard<N>
    where
        [(); N]: crate::primitives::ValidBoardSize,
    {
        for _ in 0..n {
            let Some(&pos) = board.get_valid_placements().first() else {
                break;
            };
            board = board.place_queen(pos).unwrap();
        }
        board
    }

    proptest! {
        /// `place_queen` succeeds exactly for in-bounds, unblocked cells, and
        /// reports the correct error variant otherwise — for arbitrary layouts
        /// and arbitrary prior placement sequences.
        #[test]
        fn test_place_queen_ok_iff_unblocked(
            flat in proptest::collection::vec(0usize..4, 16),
            num_placements in 0usize..8,
            row in 0usize..=4,
            col in 0usize..=4,
        ) {
            let regions = regions_from_flat::<4>(&flat);
            let board = unsafe { QueensBoard::new_unchecked(regions) };
            let board = place_first_n(board, num_placements);

            let result = board.place_queen(Position::new(row, col));

            if row >= 4 || col >= 4 {
                assert!(matches!(result, Err(QueensPlacementError::OutOfBounds { .. })));
            } else if board.blocked_mask[row][col] {
                assert!(matches!(
                    result,
                    Err(QueensPlacementError::NonEmptyPlacementTarget { .. })
                ));
            } else {
                assert!(result.is_ok());
            }
        }

        /// After any valid placement, the queen is recorded, its row/column/
        /// neighbors/color-region are blocked, the hash is recomputed, and
        /// exhaustive blocking is at a fixpoint.
        #[test]
        fn test_place_queen_invariants(
            flat in proptest::collection::vec(0usize..5, 25),
            num_placements in 0usize..6,
        ) {
            let regions = regions_from_flat::<5>(&flat);
            let mut board = unsafe { QueensBoard::new_unchecked(regions) };

            for _ in 0..num_placements {
                let Some(&pos) = board.get_valid_placements().first() else {
                    break;
                };

                let color = board.get_color_at_pos(pos);
                let placed = board.place_queen(pos).unwrap();
                let (r, c) = pos.to_tuple();

                // Queen and its color are recorded.
                assert_eq!(placed.queen_col_per_row[r], Some(c));
                assert!(placed.queen_present_by_color(usize::from(color)));

                // Row and column are fully blocked.
                for i in 0..5 {
                    assert!(placed.blocked_mask[r][i], "({}, {}) should be blocked", r, i);
                    assert!(placed.blocked_mask[i][c], "({}, {}) should be blocked", i, c);
                }

                // All eight neighbors are blocked.
                for (dr, dc) in super::NEIGHBOR_DELTAS {
                    if let Some(n) = pos.displace::<5>(dr, dc) {
                        assert!(
                            placed.blocked_mask[n.row()][n.col()],
                            "neighbor ({}, {}) should be blocked",
                            n.row(),
                            n.col()
                        );
                    }
                }

                // The entire color region is blocked.
                let mask = board.get_color_mask(usize::from(color));
                for rr in 0..5 {
                    for cc in 0..5 {
                        if mask[rr][cc] {
                            assert!(
                                placed.blocked_mask[rr][cc],
                                "color-region cell ({}, {}) should be blocked",
                                rr,
                                cc
                            );
                        }
                    }
                }

                // Hash is recomputed from queen placements.
                assert_eq!(
                    placed.get_hash(),
                    super::hash::compute_board_hash(&placed.queen_col_per_row)
                );

                // Exhaustive blocking is at a fixpoint.
                let once_more = placed.clone().perform_exhaustive_blocking();
                assert_eq!(once_more.blocked_mask, placed.blocked_mask);

                board = placed;
            }
        }
    }
}
