# queen-sweep-core

The core solver engine for QueenSweep, implementing a high-performance depth-first search algorithm with constraint propagation for solving Queens puzzles.

## Overview

This crate provides the fundamental solving logic for Queens puzzles - a constraint satisfaction problem where N queens must be placed on an NxN board such that:
- Exactly one queen appears in each colored region
- Exactly one queen appears in each row and column
- No two queens are adjacent to each other

The solver combines depth-first search with aggressive pruning through 1-step lookahead constraint checking and configurable heuristics.

## Architecture

### Core Components

#### `GameState`
The central data structure representing a puzzle configuration. Maintains:
- **Board state**: A flat vector of `CellState` values
- **Color regions**: Immutable reference-counted color assignments for each cell
- **Color masks**: Pre-computed boolean masks for efficient region lookups
- **Queen tracking**: Per-region boolean flags indicating queen placement
- **State hash**: Pre-computed hash for efficient deduplication in search

Key design decisions:
- Uses `Rc<[T]>` for immutable shared data to avoid cloning overhead during search
- Employs hash-based equality checking with pre-computed hashes for O(1) duplicate detection
- Maintains invariants through the `place_queen` method rather than exposing mutable state

#### `CellState`
Represents the three possible states of a board cell:
```rust
pub enum CellState {
    Empty = 0,    // Valid placement location
    Blocked = 1,  // Invalid due to constraints
    Queen = 2,    // Queen placed
}
```

#### Depth-First Search (`dfs.rs`)
Implements backtracking search with memoization:
```rust
pub fn depth_first_search(game_state: GameState) -> (Option<GameState>, usize)
```

The algorithm:
1. Maintains a `HashSet` of visited states to prevent redundant exploration
2. Returns early on goal state detection
3. Explores valid placements in heuristic-determined order
4. Backtracks when no valid placements remain

### Constraint Propagation

The solver performs aggressive constraint propagation after each queen placement:

1. **Direct blocking**: Blocks row, column, and 8-adjacent cells
2. **Region blocking**: Blocks all cells in the same color region
3. **Lookahead blocking**: Blocks cells whose placement would make any other region unsolvable

The lookahead check (`can_place_queen`) ensures:
- After placing a queen at position (r,c), every other region still has at least one valid empty cell
- This 1-step lookahead significantly prunes the search space early

### Heuristics

Heuristics determine the order in which valid placements are explored. The system supports pluggable heuristic functions via the `HeuristicFn` type:

```rust
pub type HeuristicFn = fn(&HeuristicContext) -> Vec<((usize, usize), f32)>;
```

Built-in heuristics (in `heuristic.rs`):

#### `smallest_region_first`
Prioritizes placing queens in regions with fewer total cells.
- **Rationale**: Smaller regions constrain the search space more
- **Metric**: Total cell count per region

#### `smallest_region_by_empty_cells`
Prioritizes placing queens in regions with fewer remaining empty cells.
- **Rationale**: Regions with fewer valid options are more likely to become unsolvable
- **Metric**: Current empty cell count per region
- **Performance**: Generally the most effective heuristic

The `#[heuristic]` procedural macro enforces type safety at compile time.

## Usage Example

```rust
use queen_sweep_core::{GameState, GameStateError, depth_first_search, heuristic::*};

fn solve_puzzle() -> Result<(), GameStateError> {
    let color_regions = vec![
        vec![0, 1, 1, 2],
        vec![0, 1, 2, 2],
        vec![0, 0, 2, 3],
        vec![0, 3, 3, 3],
    ];

    let heuristic = Some(smallest_region_by_empty_cells);
    let state = GameState::from_color_regions(color_regions, heuristic)?;

    let (solution, states_visited) = depth_first_search(state);

    match solution {
        Some(solved) => {
            println!("Solution found in {} steps", states_visited);
            for (row, col) in solved.queen_positions() {
                println!("Queen at ({}, {})", row, col);
            }
        }
        None => println!("No solution exists"),
    }

    Ok(())
}
```

## Validation and Error Handling

The `GameState::try_from` implementation validates:
- Board existence (size > 0)
- Square board dimensions
- Correct cell count
- Size constraints (≤ 255 for u8 color indexing)

Color normalization ensures color indices are contiguous starting from 0, regardless of input values.

## Testing

Run the solver directly via the included `main.rs`:

```bash
cargo run --release --features display
```

Modify the `intialize_state()` function in `main.rs` to test different puzzles (samples in `sample_levels` module in `main.rs`) or heuristics (in `heuristic.rs`).

Performance metrics are displayed after each solve:
- States explored
- Time elapsed (milliseconds and microseconds)
- Exploration rate (states/ms)