use std::time::Instant;

use queen_sweep_core::{GameState, GameStateError, depth_first_search, heuristic::*};

mod display;
use display::pretty_print;

#[allow(dead_code)]
mod sample_levels;
use sample_levels::*;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), GameStateError> {
    let color_regions = puzzle_11x11();

    let state = GameState::from_color_regions(color_regions, Some(smallest_region_by_empty_cells))?;
    // let state = GameState::from_color_regions(color_regions, None)?;

    println!("Initial state:");
    pretty_print(&state);

    let start = Instant::now();
    let (solved, steps) = depth_first_search(state);
    let duration = start.elapsed();

    match solved {
        Some(solved) => {
            println!("✓ Solution found!");
            println!();
            pretty_print(&solved);
        }
        None => {
            println!("✗ No solution exists");
            println!();
            println!("{}", "=".repeat(50));
        }
    }

    print_statistics(steps, duration);

    Ok(())
}

fn print_statistics(steps: usize, duration: std::time::Duration) {
    let millis = duration.as_secs_f64() * 1000.0;

    println!("Statistics:");
    println!("  Steps explored: {}", steps);
    println!("  Time taken: {:.6} ms, {:.6} s", millis, millis / 1000.00);

    if millis > 0.0 {
        let steps_per_ms = steps as f64 / millis;
        println!("  Performance: {:.0} steps/ms", steps_per_ms);
    }
}
