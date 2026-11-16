# queen-sweep-core

The core solver engine for QueenSweep, implementing a high-performance, const-generic depth-first search algorithm with constraint propagation for solving Queens puzzles.

## Overview

This crate provides the fundamental solving logic for Queens puzzles - a constraint satisfaction problem where N queens must be placed on an NxN board such that:

- Exactly one queen appears in each colored region
- Exactly one queen appears in each row and column
- No two queens are adjacent to each other (including diagonally)

The solver combines depth-first search with aggressive pruning through 1-step lookahead constraint checking and configurable heuristics. It exposes both a **const-generic API** (board size known at compile time) and a runtime **`dynamic` API** (size inferred from `Vec` input).

## Architecture

### Core Components

#### `QueensBoard<N>`

The central data structure representing a puzzle configuration, generic over the board size `N` with the bound `[(); N]: ValidBoardSize`. It maintains:

- **Blocked mask**: a `[[bool; N]; N]` grid of cells that may no longer receive a queen
- **Queen placement**: `[Option<usize>; N]`, the queen column per row
- **Color tracking**: `[bool; N]`, whether each region already has a queen
- **State hash**: a pre-computed hash of the queen placement for deduplication
- **Immutable data**: `Rc<ImmutableBoardData<N>>` holding the color regions and pre-computed color masks

Key design decisions:

- Uses `Rc` for the immutable color data to avoid cloning it during search
- Caches a deterministic hash consistent with `PartialEq`, so boards work as `HashMap`/`HashSet` keys
- Maintains invariants through `place_queen` rather than exposing mutable state

#### `Cell` and `ValidBoardSize`

`Cell` represents the three possible states of a board cell:

```rust
pub enum Cell {
    Empty = 0,    // Valid placement location
    Blocked = 1,  // Invalid due to constraints
    Queen = 2,    // Queen placed
}
```

`ValidBoardSize` is a marker trait implemented for `[(); N]` when `N` is a supported board size. It appears throughout the crate as `where [(); N]: ValidBoardSize`, rejecting unsupported sizes at compile time.

#### Depth-First Search

`DepthFirstSearchSolver<N, H>` implements backtracking search with memoization:

```rust
pub trait QueensSolver<const N: usize, H>
where
    [(); N]: ValidBoardSize,
    H: QueensHeuristic<N>,
{
    fn solve(board: QueensBoard<N>) -> QueensSolveResult<N>;
}
```

The algorithm:

1. Maintains a `HashSet` of visited board states to prevent redundant exploration
2. Returns early on goal state detection (`is_solved`)
3. Explores valid placements in heuristic-determined order
4. Backtracks when no valid placements remain

### Constraint Propagation

The solver performs aggressive constraint propagation after each queen placement:

1. **Direct blocking**: blocks the row, column, and all 8-adjacent cells
2. **Region blocking**: blocks every cell in the same color region
3. **Lookahead blocking**: exhaustively blocks any cell whose placement would leave another region with no remaining valid cell

The board is always kept at a **fixpoint** of this blocking, so `get_valid_placements` returns only cells from which a solution can still be reached. This 1-step lookahead significantly prunes the search space early.

### Heuristics

Heuristics determine the order in which valid placements are explored via the `QueensHeuristic` trait:

```rust
pub trait QueensHeuristic<const N: usize>
where
    [(); N]: ValidBoardSize,
{
    fn score_placements(
        valid_placements: Vec<Position>,
        board: &QueensBoard<N>,
    ) -> Vec<(Position, f32)>;

    fn order_by_heuristic(placements: Vec<Position>, board: &QueensBoard<N>) -> Vec<Position>;
}
```

Built-in heuristics:

#### `NoOpHeuristic`
Assigns every placement the same score, leaving the order produced by the board unchanged. Useful as an identity baseline.

#### `SmallestRegionFirstHeuristic`
Prioritizes placements in the color region with the **fewest remaining empty cells**.

- **Rationale**: Regions with fewer valid options are the most constraining and most likely to become unsolvable
- **Metric**: Current empty cell count per region
- **Performance**: Generally the most effective built-in heuristic

### Runtime `dynamic` API

The `dynamic` module bridges runtime `Vec` input to the const-generic solver. `QueensSolverConstructor` accepts color regions whose cells are `Color` or any `Into<Color>` (for example `usize`), validates them, infers the size, and dispatches to the correct monomorphization:

```rust
pub struct QueensSolverConstructor;

impl QueensSolverConstructor {
    pub fn build_grid<T: Into<Color>>(
        regions: Vec<Vec<T>>,
        solver: SolverOption,
        heuristic: HeuristicOption,
    ) -> Result<DynamicSolver, DynamicSolverError>;

    pub fn build_flat<T: Into<Color>>(
        regions: Vec<T>,
        solver: SolverOption,
        heuristic: HeuristicOption,
    ) -> Result<DynamicSolver, DynamicSolverError>;
}
```

`DynamicSolver::solve()` returns a `DynamicSolveResult`:

```rust
pub struct DynamicSolveResult {
    pub size: usize,
    pub solved_board: Option<Vec<Vec<Cell>>>,
    pub num_states_visited: usize,
}
```

## Board Sizes and Hashing

Supported board sizes are `1` and `4..=18`. Hashing of the queen placement is:

- **Collision-free base-16 encoding** for `N <= 15` (fits in a `u64`)
- **64-bit FNV-1a** for `N >= 16` (collisions are theoretically possible, but `HashSet` handles them correctly via `Eq`)

## Usage Example

### Runtime API

```rust
use queen_sweep_core::{
    dynamic::{HeuristicOption, QueensSolverConstructor, SolverOption},
    primitives::Cell,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let color_regions = vec![
        vec![0, 1, 1, 2],
        vec![0, 1, 2, 2],
        vec![0, 0, 2, 3],
        vec![0, 3, 3, 3],
    ];

    let solver = QueensSolverConstructor::build_grid(
        color_regions,
        SolverOption::DFSSolver,
        HeuristicOption::SmallestRegionFirstHeuristic,
    )?;

    let result = solver.solve();

    match result.solved_board {
        Some(board) => {
            println!(
                "Solved {}x{} in {} states",
                result.size, result.size, result.num_states_visited
            );
            for (r, row) in board.iter().enumerate() {
                for (c, cell) in row.iter().enumerate() {
                    if *cell == Cell::Queen {
                        println!("Queen at ({}, {})", r, c);
                    }
                }
            }
        }
        None => println!("No solution exists"),
    }

    Ok(())
}
```

### Const-Generic API

```rust
use queen_sweep_core::{
    board::QueensBoard,
    heuristic::SmallestRegionFirstHeuristic,
    primitives::Color,
    solver::{DepthFirstSearchSolver, QueensSolver},
};

let color_regions: [[Color; 4]; 4] = [
    [0.into(), 1.into(), 1.into(), 2.into()],
    [0.into(), 1.into(), 2.into(), 2.into()],
    [0.into(), 0.into(), 2.into(), 3.into()],
    [0.into(), 3.into(), 3.into(), 3.into()],
];

let board = QueensBoard::new(color_regions)?;
let result = DepthFirstSearchSolver::<4, SmallestRegionFirstHeuristic>::solve(board);
```

## Validation and Error Handling

- `BoardInitError` - a color index is out of range, or the board does not contain exactly `N` distinct colors
- `QueensPlacementError` - out-of-bounds coordinates, or a blocked placement target
- `DynamicSolverError` - empty input, non-square grid, non-perfect-square flat list, unsupported size, or board-init failure

## Safety

The crate contains two `unsafe` functions - `QueensBoard::new_unchecked` and `QueensBoard::place_queen_unchecked` - used internally to skip re-validation. Both document their invariants in `# Safety` sections, and all `unsafe` call sites carry `// SAFETY:` justifications.

## Testing

```bash
cargo test -p queen-sweep-core
```
