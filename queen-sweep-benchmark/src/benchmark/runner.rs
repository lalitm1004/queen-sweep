use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};
use queen_sweep_core::{GameState, depth_first_search};
use rayon::prelude::*;

use crate::heuristic::Heuristic;
use crate::levels::LevelData;

use super::result::BenchmarkResult;

fn benchmark_level(level: &LevelData, heuristic: Heuristic) -> BenchmarkResult {
    let color_regions: Vec<Vec<u8>> = level
        .regions
        .iter()
        .map(|row| row.iter().map(|&v| v as u8).collect())
        .collect();

    let heuristic_fn = heuristic.to_fn();

    let game_state = GameState::from_color_regions(color_regions, heuristic_fn)
        .expect("error initializing gamestate");

    let timer = Instant::now();
    let (solution, steps_taken) = depth_first_search(game_state);
    let duration_nanos = timer.elapsed().as_nanos();

    let solved = solution.is_some();

    BenchmarkResult::new(level.id, level.size, duration_nanos, steps_taken, solved)
}

pub fn benchmark_levels_with_progress(
    levels: &[LevelData],
    category_name: &str,
    heuristic: Heuristic,
) -> Vec<BenchmarkResult> {
    let pb = ProgressBar::new(levels.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {elapsed_precise}")
            .unwrap()
            .progress_chars("█▓▒░ "),
    );
    pb.set_message(format!(
        "Benchmarking {} ({})",
        category_name,
        heuristic.name()
    ));

    let results: Vec<BenchmarkResult> = levels
        .par_iter()
        .map(|level| {
            let result = benchmark_level(level, heuristic);
            pb.inc(1);
            result
        })
        .collect();

    pb.finish_with_message(format!(
        "✔ {} ({}) complete",
        category_name,
        heuristic.name()
    ));
    results
}
