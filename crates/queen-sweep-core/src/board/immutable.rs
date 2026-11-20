//! Immutable, shared board configuration.
//!
//! The color-region layout never changes during a solve, so it is stored once
//! in an [`ImmutableBoardData`] and shared between boards via
//! [`std::rc::Rc`].

use crate::primitives::{Color, Position, ValidBoardSize};

/// The immutable color-region layout shared by every board in a search.
///
/// Besides the raw `color_regions` grid, it caches a boolean `color_masks`
/// table so the cells of any color can be looked up in `O(1)`.
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
    /// Builds the immutable data, precomputing the per-color masks.
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

    /// Returns the color of the cell at `pos`.
    #[inline(always)]
    pub(super) const fn get_color_at_pos(&self, pos: Position) -> Color {
        self.color_regions[pos.row()][pos.col()]
    }

    /// Returns the boolean mask of the given color region.
    #[inline(always)]
    pub(super) fn get_color_mask(&self, color: impl Into<usize>) -> &[[bool; N]; N] {
        &self.color_masks[color.into()]
    }

    /// Returns a reference to the `N×N` color regions.
    #[inline(always)]
    pub(super) const fn get_color_regions(&self) -> &[[Color; N]; N] {
        &self.color_regions
    }
}

#[cfg(test)]
mod tests {
    use super::ImmutableBoardData;
    use crate::primitives::{Color, Position};

    fn create_color_regions<const N: usize>() -> [[Color; N]; N] {
        std::array::from_fn(|row| std::array::from_fn(|col| Color::from((row + col) % N)))
    }

    #[test]
    fn test_get_color_at_pos() {
        let regions = create_color_regions::<4>();
        let board = ImmutableBoardData::<4>::new(regions);

        assert_eq!(usize::from(board.get_color_at_pos(Position::new(0, 0))), 0);

        assert_eq!(usize::from(board.get_color_at_pos(Position::new(1, 2))), 3);

        assert_eq!(usize::from(board.get_color_at_pos(Position::new(3, 3))), 2);
    }

    #[test]
    fn test_get_color_mask() {
        let regions = [
            [
                Color::from(0usize),
                Color::from(1usize),
                Color::from(2usize),
                Color::from(3usize),
            ],
            [
                Color::from(1usize),
                Color::from(2usize),
                Color::from(3usize),
                Color::from(0usize),
            ],
            [
                Color::from(2usize),
                Color::from(3usize),
                Color::from(0usize),
                Color::from(1usize),
            ],
            [
                Color::from(3usize),
                Color::from(0usize),
                Color::from(1usize),
                Color::from(2usize),
            ],
        ];

        let board = ImmutableBoardData::<4>::new(regions);

        let expected_mask = [
            [true, false, false, false],
            [false, false, false, true],
            [false, false, true, false],
            [false, true, false, false],
        ];

        assert_eq!(board.get_color_mask(0usize), &expected_mask);
    }

    #[test]
    fn test_get_all_color_masks() {
        let regions = create_color_regions::<4>();
        let board = ImmutableBoardData::<4>::new(regions);

        for color in 0usize..4 {
            let mask = board.get_color_mask(color);

            for row in 0..4 {
                for col in 0..4 {
                    assert_eq!(
                        mask[row][col],
                        usize::from(regions[row][col]) == color,
                        "Incorrect mask for color {color} at ({row}, {col})"
                    );
                }
            }
        }
    }

    #[test]
    fn test_color_masks_are_mutually_exclusive() {
        let regions = create_color_regions::<4>();
        let board = ImmutableBoardData::<4>::new(regions);

        for row in 0..4 {
            for col in 0..4 {
                let matching_masks = (0usize..4)
                    .filter(|&color| board.get_color_mask(color)[row][col])
                    .count();

                assert_eq!(matching_masks, 1);
            }
        }
    }

    #[test]
    fn test_color_masks_cover_entire_board() {
        let regions = create_color_regions::<4>();
        let board = ImmutableBoardData::<4>::new(regions);

        for color in 0usize..4 {
            let matching_cells = board
                .get_color_mask(color)
                .iter()
                .flatten()
                .filter(|&&matches| matches)
                .count();

            assert_eq!(matching_cells, 4);
        }
    }

    #[test]
    fn test_larger_board() {
        let regions = create_color_regions::<8>();
        let board = ImmutableBoardData::<8>::new(regions);

        assert_eq!(board.get_color_regions(), &regions);

        for color in 0usize..8 {
            let matching_cells = board
                .get_color_mask(color)
                .iter()
                .flatten()
                .filter(|&&matches| matches)
                .count();

            assert_eq!(matching_cells, 8);
        }
    }
}
