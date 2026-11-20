use crate::{
    board::QueensBoard,
    heuristic::QueensHeuristic,
    primitives::{Position, ValidBoardSize},
};

pub struct SmallestRegionFirstHeuristic;

impl<const N: usize> QueensHeuristic<N> for SmallestRegionFirstHeuristic
where
    [(); N]: ValidBoardSize,
{
    fn score_placements(
        valid_placements: Vec<Position>,
        board: &QueensBoard<N>,
    ) -> Vec<(Position, f32)> {
        let mut color_empty_counts = [0_usize; N];

        let blocked_mask = board.get_blocked_mask();

        for row in 0..N {
            for col in 0..N {
                if !blocked_mask[row][col] {
                    let pos = Position::new(row, col);
                    let color = board.get_color_at_pos(pos);

                    color_empty_counts[usize::from(color)] += 1;
                }
            }
        }

        valid_placements
            .into_iter()
            .map(|pos| {
                let color = board.get_color_at_pos(pos);
                let score = color_empty_counts[usize::from(color)];

                (pos, score as f32)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::SmallestRegionFirstHeuristic;
    use crate::{
        board::QueensBoard,
        heuristic::QueensHeuristic,
        primitives::{Color, Position, ValidBoardSize},
    };

    /// A diagonal layout where `color = (row + col) % N`.
    fn diagonal_board<const N: usize>() -> QueensBoard<N>
    where
        [(); N]: ValidBoardSize,
    {
        let regions = std::array::from_fn(|r| std::array::from_fn(|c| Color::from((r + c) % N)));
        unsafe { QueensBoard::new_unchecked(regions) }
    }

    /// A layout with deliberately unequal region sizes so ordering is non-trivial:
    /// color 0 has 1 cell, color 1 has 2, color 2 has 3, color 3 has 10.
    fn unequal_region_board() -> QueensBoard<4> {
        let regions: [[Color; 4]; 4] = [
            [0.into(), 1.into(), 1.into(), 2.into()],
            [2.into(), 2.into(), 3.into(), 3.into()],
            [3.into(), 3.into(), 3.into(), 3.into()],
            [3.into(), 3.into(), 3.into(), 3.into()],
        ];
        unsafe { QueensBoard::new_unchecked(regions) }
    }

    /// Independently counts the unblocked cells in a color region — exactly the
    /// quantity the heuristic's score is documented to be.
    fn independent_empty_count<const N: usize>(board: &QueensBoard<N>, color: usize) -> usize
    where
        [(); N]: ValidBoardSize,
    {
        let blocked = board.get_blocked_mask();
        let mask = board.get_color_mask(color);
        (0..N)
            .flat_map(|r| (0..N).map(move |c| (r, c)))
            .filter(|&(r, c)| mask[r][c] && !blocked[r][c])
            .count()
    }

    #[test]
    fn scores_match_region_empty_cell_count() {
        // A queen is placed so the blocked mask is non-trivial and the score must
        // reflect blocked cells, not just the raw region size.
        let regions: [[Color; 4]; 4] = [
            [0.into(), 0.into(), 1.into(), 1.into()],
            [0.into(), 0.into(), 1.into(), 1.into()],
            [2.into(), 2.into(), 3.into(), 3.into()],
            [2.into(), 2.into(), 3.into(), 3.into()],
        ];
        let board = unsafe { QueensBoard::new_unchecked(regions) }
            .place_queen(Position::new(0, 1))
            .unwrap();

        let placements = board.get_valid_placements();
        assert!(!placements.is_empty(), "test board should still have placements");

        let scored = SmallestRegionFirstHeuristic::score_placements(placements.clone(), &board);

        assert_eq!(scored.len(), placements.len());

        for (pos, score) in scored {
            let color = usize::from(board.get_color_at_pos(pos));
            let expected = independent_empty_count(&board, color) as f32;
            assert_eq!(score, expected, "wrong score for ({}, {})", pos.row(), pos.col());
        }
    }

    #[test]
    fn order_prioritizes_smallest_region() {
        // On the empty unequal-region board the smallest region is color 0 with a
        // single cell (0, 0), so it must be explored first.
        let board = unequal_region_board();
        let placements = board.get_valid_placements();

        let ordered =
            SmallestRegionFirstHeuristic::order_by_heuristic(placements.clone(), &board);

        // Same set of placements is returned.
        let mut expected: Vec<_> = placements.iter().map(|p| (p.row(), p.col())).collect();
        let mut actual: Vec<_> = ordered.iter().map(|p| (p.row(), p.col())).collect();
        expected.sort_unstable();
        actual.sort_unstable();
        assert_eq!(expected, actual);

        // The first placement belongs to the smallest region (color 0, size 1).
        assert_eq!(ordered.first().copied(), Some(Position::new(0, 0)));

        // Ascending by independent region empty count.
        let counts: Vec<usize> = ordered
            .iter()
            .map(|p| independent_empty_count(&board, usize::from(board.get_color_at_pos(*p))))
            .collect();
        assert!(counts.windows(2).all(|w| w[0] <= w[1]), "not sorted by region size: {counts:?}");
    }

    #[test]
    fn placements_in_same_region_share_score() {
        let board = diagonal_board::<4>();
        let placements = board.get_valid_placements();

        let scored = SmallestRegionFirstHeuristic::score_placements(placements, &board);

        // Every placement in the same color region must receive the same score.
        for i in 0..scored.len() {
            for j in (i + 1)..scored.len() {
                let (pi, si) = scored[i];
                let (pj, sj) = scored[j];
                if board.get_color_at_pos(pi) == board.get_color_at_pos(pj) {
                    assert_eq!(si, sj);
                }
            }
        }
    }
}
