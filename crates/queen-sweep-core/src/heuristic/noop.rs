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
