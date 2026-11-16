# queen-sweep-core

The core solver engine for QueenSweep: a high-performance, const-generic depth-first-search solver for the Queens puzzle with aggressive constraint propagation and pluggable heuristics.

## Overview

Queens is a constraint-satisfaction puzzle: place `N` queens on an `N×N` board such that:

- exactly one queen appears in each **colored region**,
- exactly one queen appears in each **row** and **column**, and
- no two queens are **adjacent** (including diagonally).

This crate models the board, derives the blocked cells from every placement, and searches for a solution with backtracking. It exposes both a **const-generic API** (board size known at compile time) and a runtime **`dynamic` API** (size inferred from `Vec` input).

## Architecture

### `QueensBoard<N>`

The central state type, generic over the board size `N` with the compile-time bound `[(); N]: ValidBoardSize`. It stores:

| Field | Purpose |
| --- | --- |
| `blocked_mask: [[bool; N]; N]` | Cells that may no longer receive a queen (derived from placements). |
| `queen_col_per_row: [Option<usize>; N]` | The queen column in each row. |
| `queen_present_by_color: [bool; N]` | Whether each color region already has a queen. |
| `hash: u64` | Cached hash of the queen placement. |
| `immutable_board_data: Rc<ImmutableBoardData<N>>` | Shared, immutable color-region layout and precomputed color masks. |

The board is immutable in spirit: `place_queen` returns a new board rather than mutating in place. Two boards compare equal when their queen placements are identical (see the `PartialEq`/`Eq`/`Hash` impls), which makes them usable as `HashMap`/`HashSet` keys during search.

### `Cell` / `ValidBoardSize`

`Cell` is `Empty` (0), `Blocked` (1), or `Queen` (2). `ValidBoardSize` is a marker trait implemented for `[(); N]` when `N` is a supported board size; it appears throughout the crate as `where [(); N]: ValidBoardSize`.

### Constraint propagation

After a queen is placed, the board blocks:

1. its entire **row** and **column**,
2. all **eight neighbors**, and
3. its entire **color region**.

Then **exhaustive blocking** runs to a fixpoint: any cell whose placement would strand another color region (leave it with no valid cells) is blocked as well. This 1-step lookahead prunes the search space early.

### Depth-first search

`DepthFirstSearchSolver<N, H>` explores valid placements depth-first, deduplicating visited boards with a `HashSet` and ordering moves with heuristic `H`:

```rust
pub trait QueensSolver<const N: usize, H>
where
    [(); N]: ValidBoardSize,
    H: QueensHeuristic<N>,
{
    fn solve(board: QueensBoard<N>) -> QueensSolveResult<N>;
}
```

### Heuristics

`QueensHeuristic<N>` scores candidate placements; `order_by_heuristic` sorts them ascending.

- `NoOpHeuristic` — assigns every placement the same score (identity ordering).
- `SmallestRegionFirstHeuristic` — scores each placement by the number of **remaining empty cells** in its color region, prioritizing the most-constrained regions. In practice this is the most effective built-in heuristic.

### Runtime `dynamic` API

The `dynamic` module bridges runtime `Vec` input to the const-generic solver:

- `QueensSolverConstructor::build_grid` / `build_flat` / `build` accept color regions whose cells are `Color` or any `Into<Color>` (e.g. `usize`), validate them, infer the size, and dispatch to the correct monomorphization.
- `DynamicSolver::solve()` returns a `DynamicSolveResult { size, solved_board: Option<Vec<Vec<Cell>>>, num_states_visited }`.

## Board sizes and hashing

Supported board sizes are `1` and `4..=18`. Hashing of the queen placement is:

- **collision-free base-16 encoding** for `N <= 15` (fits in `u64`),
- **64-bit FNV-1a** for `N >= 16` (collisions theoretically possible, handled correctly by `Eq`).

## Usage

### Runtime API

```rust
use queen_sweep_core::{
    dynamic::{HeuristicOption, QueensSolverConstructor, SolverOption},
    primitives::Cell,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grid = vec![
        vec![0, 1, 1, 2],
        vec![0, 1, 2, 2],
        vec![0, 0, 2, 3],
        vec![0, 3, 3, 3],
    ];

    let solver = QueensSolverConstructor::build_grid(
        grid,
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
                        println!("queen at ({r}, {c})");
                    }
                }
            }
        }
        None => println!("No solution"),
    }

    Ok(())
}
```

### Const-generic API

```rust
use queen_sweep_core::{
    board::QueensBoard,
    heuristic::SmallestRegionFirstHeuristic,
    primitives::Color,
    solver::{DepthFirstSearchSolver, QueensSolver},
};

let regions: [[Color; 4]; 4] = [
    [0.into(), 1.into(), 1.into(), 2.into()],
    [0.into(), 1.into(), 2.into(), 2.into()],
    [0.into(), 0.into(), 2.into(), 3.into()],
    [0.into(), 3.into(), 3.into(), 3.into()],
];

let board = QueensBoard::new(regions)?;
let result = DepthFirstSearchSolver::<4, SmallestRegionFirstHeuristic>::solve(board);
```

## Errors

- `BoardInitError` — color index out of range, or not exactly `N` distinct colors.
- `QueensPlacementError` — out-of-bounds coordinates, or a blocked placement target.
- `DynamicSolverError` — empty input, non-square grid, non-perfect-square flat list, unsupported size, or board-init failure.

## Safety

The crate contains two `unsafe` functions — `QueensBoard::new_unchecked` and `QueensBoard::place_queen_unchecked` — used internally to skip re-validation. Both document their invariants in `# Safety` sections, and all `unsafe` call sites carry `// SAFETY:` justifications.

## Testing

```bash
cargo test -p queen-sweep-core
```
