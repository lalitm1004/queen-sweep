//! The type-erased solver runtime and construction logic.

use super::{
    error::DynamicSolverError,
    input::RegionInput,
    options::{HeuristicOption, SolverOption},
    result::DynamicSolveResult,
};
use crate::{
    board::QueensBoard,
    heuristic::{NoOpHeuristic, SmallestRegionFirstHeuristic},
    primitives::{Color, ValidBoardSize},
    solver::{DepthFirstSearchSolver, QueensSolveResult, QueensSolver},
};

/// Builds a [`DynamicSolver`] from runtime inputs.
pub struct QueensSolverConstructor;

/// A type-erased solver over a fixed board.
///
/// Created via [`QueensSolverConstructor`] and reusable: each call to
/// [`solve`](DynamicSolver::solve) runs a fresh search from the initial board.
pub struct DynamicSolver {
    inner: Box<dyn DynamicQueensSolver>,
}

impl DynamicSolver {
    /// Runs the solver, returning the solution (if any) and search statistics.
    pub fn solve(&self) -> DynamicSolveResult {
        self.inner.solve()
    }
}

impl std::fmt::Debug for DynamicSolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicSolver").finish_non_exhaustive()
    }
}

/// Object-safe view over a monomorphized solver.
trait DynamicQueensSolver {
    fn solve(&self) -> DynamicSolveResult;
}

/// A solver bound to a specific size `N`, holding the initial board and the
/// monomorphized solve function so it can be re-solved on demand.
struct BuiltSolver<const N: usize>
where
    [(); N]: ValidBoardSize,
{
    initial_board: QueensBoard<N>,
    solve_fn: fn(QueensBoard<N>) -> QueensSolveResult<N>,
}

impl<const N: usize> DynamicQueensSolver for BuiltSolver<N>
where
    [(); N]: ValidBoardSize,
{
    fn solve(&self) -> DynamicSolveResult {
        DynamicSolveResult::from_const((self.solve_fn)(self.initial_board.clone()))
    }
}

/// Selects the monomorphized solve function for a `(solver, heuristic)` pair.
///
/// This is the runtime→static dispatch table: each row maps a
/// `(solver, heuristic)` combination to the concrete [`QueensSolver::solve`]
/// function for size `N`. Adding a solver or heuristic is a one-line addition
/// here plus an enum variant. Because the table is a flat list (rather than a
/// hand-written nested match) and `BuiltSolver` stores only a function pointer,
/// the solver algorithm and heuristic stay fully decoupled.
impl QueensSolverConstructor {
    /// Returns the monomorphized solve function for a concrete `(solver,
    /// heuristic)` pair at size `N`.
    fn solve_fn<const N: usize>(
        solver: SolverOption,
        heuristic: HeuristicOption,
    ) -> fn(QueensBoard<N>) -> QueensSolveResult<N>
    where
        [(); N]: ValidBoardSize,
    {
        match (solver, heuristic) {
            (SolverOption::DFSSolver, HeuristicOption::NoOpHeuristic) => {
                DepthFirstSearchSolver::<N, NoOpHeuristic>::solve
            }
            (SolverOption::DFSSolver, HeuristicOption::SmallestRegionFirstHeuristic) => {
                DepthFirstSearchSolver::<N, SmallestRegionFirstHeuristic>::solve
            }
        }
    }

    /// Builds a [`DynamicSolver`] from color regions, a solver, and a heuristic.
    ///
    /// The regions are given as an explicit [`RegionInput`]; use
    /// [`build_grid`](Self::build_grid) or [`build_flat`](Self::build_flat) for
    /// the more ergonomic `Vec` forms.
    ///
    /// # Errors
    ///
    /// Returns a [`DynamicSolverError`] if the regions are empty, not square
    /// (or not a perfect square when flat), imply an unsupported board size, or
    /// fail board initialization (see [`BoardInitError`](crate::board::BoardInitError)).
    pub fn build<T>(
        regions: RegionInput<T>,
        solver: SolverOption,
        heuristic: HeuristicOption,
    ) -> Result<DynamicSolver, DynamicSolverError>
    where
        T: Into<Color>,
    {
        let (n, grid) = regions.normalize()?;
        Self::dispatch(n, grid, solver, heuristic)
    }

    /// Builds a [`DynamicSolver`] from a square grid of color regions.
    ///
    /// Each cell may be a [`Color`] or any value implementing [`Into<Color>`].
    pub fn build_grid<T>(
        regions: Vec<Vec<T>>,
        solver: SolverOption,
        heuristic: HeuristicOption,
    ) -> Result<DynamicSolver, DynamicSolverError>
    where
        T: Into<Color>,
    {
        Self::build(RegionInput::Grid(regions), solver, heuristic)
    }

    /// Builds a [`DynamicSolver`] from a flat, row-major list of colors.
    ///
    /// Each entry may be a [`Color`] or any value implementing [`Into<Color>`].
    pub fn build_flat<T>(
        regions: Vec<T>,
        solver: SolverOption,
        heuristic: HeuristicOption,
    ) -> Result<DynamicSolver, DynamicSolverError>
    where
        T: Into<Color>,
    {
        Self::build(RegionInput::Flat(regions), solver, heuristic)
    }

    /// Dispatches the runtime size `n` to a const-generic build.
    fn dispatch(
        n: usize,
        grid: Vec<Vec<usize>>,
        solver: SolverOption,
        heuristic: HeuristicOption,
    ) -> Result<DynamicSolver, DynamicSolverError> {
        match n {
            1 => Self::build_for::<1>(grid, solver, heuristic),

            4..=16 => {
                seq_macro::seq! {N in 4..=16 {
                    match n {
                        #(N => Self::build_for::<N>(grid, solver, heuristic),)*
                        _ => unreachable!()
                    }
                }}
            }

            _ => Err(DynamicSolverError::UnsupportedSize(n)),
        }
    }

    /// Builds a [`DynamicSolver`] for a concrete size `N`.
    fn build_for<const N: usize>(
        grid: Vec<Vec<usize>>,
        solver: SolverOption,
        heuristic: HeuristicOption,
    ) -> Result<DynamicSolver, DynamicSolverError>
    where
        [(); N]: ValidBoardSize,
    {
        let color_regions = Self::grid_to_color_array::<N>(&grid);

        // `QueensBoard::new` validates color range and distinct-color count, and
        // `?` maps its `BoardInitError` into `DynamicSolverError::BoardInit`.
        let board = QueensBoard::new(color_regions)?;

        let inner = Box::new(BuiltSolver::<N> {
            initial_board: board,
            solve_fn: Self::solve_fn::<N>(solver, heuristic),
        });

        Ok(DynamicSolver { inner })
    }

    /// Converts a validated `Vec<Vec<usize>>` into a fixed `[[Color; N]; N]`.
    fn grid_to_color_array<const N: usize>(grid: &[Vec<usize>]) -> [[Color; N]; N] {
        std::array::from_fn(|row| std::array::from_fn(|col| Color::from(grid[row][col])))
    }
}
