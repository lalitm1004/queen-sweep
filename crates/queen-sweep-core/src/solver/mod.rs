use crate::{board::QueensBoard, heuristic::QueensHeuristic, primitives::ValidBoardSize};

mod dfs;
pub use dfs::DepthFirstSearchSolver;

pub struct QueensSolveResult<const N: usize>
where
    [(); N]: ValidBoardSize,
{
    pub solved_board: Option<QueensBoard<N>>,
    pub num_states_visited: usize,
}

pub trait QueensSolver<const N: usize, H>
where
    [(); N]: ValidBoardSize,
    H: QueensHeuristic<N>,
{
    fn solve(board: QueensBoard<N>) -> QueensSolveResult<N>;
}
