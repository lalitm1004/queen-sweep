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
