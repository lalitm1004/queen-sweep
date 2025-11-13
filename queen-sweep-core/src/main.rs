use std::time::{Duration, Instant};

use queen_sweep_core::{GameState, GameStateError, depth_first_search, heuristic::*};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn initialize_state() -> Result<GameState, GameStateError> {
    let color_regions = puzzle_11x11();

    let heuristic_fn: Option<HeuristicFn> = Some(smallest_region_by_empty_cells);

    GameState::from_color_regions(color_regions, heuristic_fn)
}

fn run() -> Result<(), GameStateError> {
    let state = initialize_state()?;

    print_board(&state, "Initial state");

    let (solved, steps, duration) = solve_puzzle(state);

    match solved {
        Some(solved) => {
            println!("✓ Solution found!\n");
            print_board(&solved, "Solution");
        }
        None => println!("✗ No solution exists\n"),
    }

    print_statistics(steps, duration);
    Ok(())
}

fn solve_puzzle(state: GameState) -> (Option<GameState>, usize, Duration) {
    let start = Instant::now();
    let (solved, steps) = depth_first_search(state);
    let duration = start.elapsed();
    (solved, steps, duration)
}

fn print_board(state: &GameState, label: &str) {
    println!("{label}:");

    #[cfg(feature = "display")]
    state.print_board();

    #[cfg(not(feature = "display"))]
    println!(
        "(display feature disabled) GameState size = {:?}",
        state.states()
    );

    println!();
}

fn print_statistics(steps: usize, duration: Duration) {
    let millis = duration.as_secs_f64() * 1000.0;

    println!("Statistics:");
    println!("  Steps explored: {}", steps);
    println!(
        "  Time taken: {:.6} ms, {:.6} µs",
        millis,
        duration.as_micros()
    );

    if millis > 0.0 {
        println!("  Performance: {:.0} steps/ms", steps as f64 / millis);
    }
}

#[allow(dead_code)]
mod sample_levels {
    pub fn puzzle_11x11() -> Vec<Vec<u8>> {
        vec![
            vec![0, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2],
            vec![0, 3, 3, 1, 1, 1, 2, 2, 2, 2, 2],
            vec![0, 3, 3, 1, 1, 2, 2, 2, 2, 2, 2],
            vec![0, 0, 0, 0, 2, 2, 2, 4, 4, 4, 4],
            vec![0, 5, 5, 6, 7, 7, 2, 2, 2, 4, 4],
            vec![0, 5, 5, 6, 7, 7, 7, 7, 7, 7, 4],
            vec![0, 5, 5, 6, 7, 7, 7, 7, 7, 7, 4],
            vec![0, 5, 5, 6, 7, 7, 8, 9, 7, 7, 4],
            vec![0, 5, 5, 6, 7, 7, 8, 9, 7, 7, 8],
            vec![10, 5, 5, 6, 7, 7, 8, 9, 7, 7, 8],
            vec![10, 10, 10, 6, 6, 8, 8, 8, 8, 8, 8],
        ]
    }

    pub fn puzzle_8x8() -> Vec<Vec<u8>> {
        vec![
            vec![0, 0, 1, 1, 1, 2, 2, 2],
            vec![0, 3, 1, 3, 1, 4, 2, 2],
            vec![0, 3, 1, 3, 1, 2, 2, 2],
            vec![0, 3, 3, 3, 1, 5, 6, 2],
            vec![0, 3, 3, 3, 1, 5, 6, 6],
            vec![0, 3, 7, 3, 1, 5, 6, 6],
            vec![7, 3, 7, 3, 1, 5, 5, 6],
            vec![7, 7, 7, 7, 6, 6, 6, 6],
        ]
    }

    pub fn puzzle_13x13() -> Vec<Vec<u8>> {
        vec![
            vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 4, 4, 3],
            vec![0, 0, 0, 0, 0, 0, 5, 5, 1, 1, 4, 3, 3],
            vec![0, 0, 0, 0, 0, 0, 5, 5, 5, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 5, 6, 1, 6, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 5, 6, 6, 6, 6, 6],
            vec![0, 0, 7, 7, 8, 8, 8, 5, 5, 5, 6, 6, 6],
            vec![9, 7, 7, 7, 8, 8, 8, 5, 5, 6, 6, 5, 6],
            vec![9, 7, 9, 7, 7, 8, 8, 8, 5, 5, 5, 5, 5],
            vec![9, 9, 9, 7, 7, 7, 7, 8, 10, 10, 5, 5, 5],
            vec![9, 9, 11, 11, 7, 7, 7, 8, 10, 10, 10, 10, 10],
            vec![12, 9, 11, 11, 11, 7, 11, 11, 10, 10, 10, 10, 10],
            vec![12, 12, 12, 12, 11, 11, 11, 10, 10, 10, 10, 10, 10],
        ]
    }

    pub fn puzzle_11x11_multiple_solns() -> Vec<Vec<u8>> {
        vec![
            vec![0, 1, 1, 0, 2, 2, 3, 3, 4, 1, 1],
            vec![1, 0, 1, 4, 0, 2, 5, 6, 4, 4, 1],
            vec![1, 1, 0, 4, 4, 0, 7, 8, 6, 4, 4],
            vec![0, 4, 4, 0, 9, 10, 0, 7, 8, 6, 3],
            vec![2, 0, 4, 9, 0, 9, 10, 0, 7, 5, 3],
            vec![2, 2, 0, 10, 9, 0, 9, 10, 0, 2, 2],
            vec![3, 5, 7, 0, 10, 9, 0, 9, 4, 0, 2],
            vec![3, 6, 8, 7, 0, 10, 9, 0, 4, 4, 0],
            vec![4, 4, 6, 8, 7, 0, 4, 4, 0, 1, 1],
            vec![1, 4, 4, 6, 5, 2, 0, 4, 1, 0, 1],
            vec![1, 1, 4, 3, 3, 2, 2, 0, 1, 1, 0],
        ]
    }
}
use sample_levels::*;
