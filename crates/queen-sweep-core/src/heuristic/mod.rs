use crate::{
    board::QueensBoard,
    primitives::{Position, ValidBoardSize},
};

mod noop;
pub use noop::NoOpHeuristic;

pub trait QueensHeuristic<const N: usize>
where
    [(); N]: ValidBoardSize,
{
    fn score_placements(
        valid_placements: Vec<Position>,
        board: &QueensBoard<N>,
    ) -> Vec<(Position, f32)>;

    fn order_by_heuristic(placements: Vec<Position>, board: &QueensBoard<N>) -> Vec<Position> {
        let mut scored = Self::score_placements(placements, board);
        scored.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().map(|(pos, _)| pos).collect()
    }
}
