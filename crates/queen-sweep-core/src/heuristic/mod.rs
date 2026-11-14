//! Move-ordering heuristics for the solver.
//!
//! A [`QueensHeuristic`] scores candidate placements so that a solver can try
//! the most promising ones first. The crate ships with the identity
//! [`NoOpHeuristic`]; custom heuristics implement the trait.

use crate::{
    board::QueensBoard,
    primitives::{Position, ValidBoardSize},
};

mod no_op;
pub use no_op::NoOpHeuristic;

mod smallest_region_first;
pub use smallest_region_first::SmallestRegionFirstHeuristic;

/// Orders candidate placements to guide a solver's search.
///
/// `score_placements` assigns a score to each candidate, and
/// `order_by_heuristic` returns them sorted ascending by score.
pub trait QueensHeuristic<const N: usize>
where
    [(); N]: ValidBoardSize,
    Self: Sized,
{
    /// Scores each valid placement, returning `(placement, score)` pairs.
    fn score_placements(
        valid_placements: Vec<Position>,
        board: &QueensBoard<N>,
    ) -> Vec<(Position, f32)>;

    /// Returns the placements sorted ascending by score.
    ///
    /// Ties (including non-finite scores) are broken by treating them as
    /// equal; the relative order of equal-scoring placements is unspecified.
    fn order_by_heuristic(placements: Vec<Position>, board: &QueensBoard<N>) -> Vec<Position> {
        let mut scored = Self::score_placements(placements, board);
        scored.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().map(|(pos, _)| pos).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::QueensHeuristic;
    use crate::{
        board::QueensBoard,
        primitives::{Color, Position, ValidBoardSize},
    };

    /// A test-only heuristic that scores each placement by its column index,
    /// so `order_by_heuristic` should return placements sorted by column.
    struct ColumnScoreHeuristic;

    impl<const N: usize> QueensHeuristic<N> for ColumnScoreHeuristic
    where
        [(); N]: ValidBoardSize,
    {
        fn score_placements(
            valid_placements: Vec<Position>,
            _board: &QueensBoard<N>,
        ) -> Vec<(Position, f32)> {
            valid_placements
                .into_iter()
                .map(|p| (p, p.col() as f32))
                .collect()
        }
    }

    fn board_4() -> QueensBoard<4> {
        let regions: [[Color; 4]; 4] =
            std::array::from_fn(|r| std::array::from_fn(|c| Color::from((r + c) % 4)));
        unsafe { QueensBoard::new_unchecked(regions) }
    }

    #[test]
    fn order_by_heuristic_sorts_ascending_by_score() {
        let board = board_4();
        let placements = board.get_valid_placements();

        let ordered = ColumnScoreHeuristic::order_by_heuristic(placements.clone(), &board);

        // Same set of placements is returned.
        let mut expected: Vec<_> = placements.iter().map(|p| (p.row(), p.col())).collect();
        let mut actual: Vec<_> = ordered.iter().map(|p| (p.row(), p.col())).collect();
        expected.sort_unstable();
        actual.sort_unstable();
        assert_eq!(expected, actual);

        // Positions are sorted ascending by column (their score).
        let columns: Vec<usize> = ordered.iter().map(|p| p.col()).collect();
        assert!(columns.windows(2).all(|w| w[0] <= w[1]));
    }
}
