use crate::{
    board::QueensBoard,
    heuristic::QueensHeuristic,
    primitives::{Position, ValidBoardSize},
};

pub struct NoOpHeuristic;

impl<const N: usize> QueensHeuristic<N> for NoOpHeuristic
where
    [(); N]: ValidBoardSize,
{
    fn score_placements(
        valid_placements: Vec<Position>,
        #[allow(unused)] board: &QueensBoard<N>,
    ) -> Vec<(Position, f32)> {
        valid_placements.into_iter().map(|p| (p, 0.0)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::NoOpHeuristic;
    use crate::{
        board::QueensBoard,
        heuristic::QueensHeuristic,
        primitives::{Color, Position},
    };
    use std::collections::HashSet;

    fn board_4() -> QueensBoard<4> {
        let regions: [[Color; 4]; 4] =
            std::array::from_fn(|r| std::array::from_fn(|c| Color::from((r + c) % 4)));
        unsafe { QueensBoard::new_unchecked(regions) }
    }

    #[test]
    fn score_placements_is_all_zero() {
        let board = board_4();
        let placements = vec![Position::new(0, 0), Position::new(1, 2), Position::new(3, 3)];

        let scored = NoOpHeuristic::score_placements(placements.clone(), &board);

        assert_eq!(scored.len(), placements.len());
        assert!(scored.iter().all(|(_, score)| *score == 0.0));
    }

    #[test]
    fn order_by_heuristic_preserves_set() {
        let board = board_4();
        let placements = board.get_valid_placements();

        let ordered = NoOpHeuristic::order_by_heuristic(placements.clone(), &board);

        // All placements are retained; only their order may (in principle) vary.
        let ordered_set: HashSet<Position> = ordered.iter().copied().collect();
        assert_eq!(ordered_set.len(), placements.len());
        assert!(placements.iter().all(|p| ordered_set.contains(p)));
    }
}
