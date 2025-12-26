use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};
use queen_sweep_core::{GameState, depth_first_search};
use rayon::prelude::*;

use crate::heuristic::Heuristic;
use crate::levels::LevelData;

use super::result::BenchmarkResult;

const NUM_RUNS: u128 = 5;

pub fn benchmark_levels(
    levels: &[LevelData],
    category_name: &str,
    heuristic: Heuristic,
) -> Vec<BenchmarkResult> {
    let pb = ProgressBar::new(levels.len() as u64);

    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
            .unwrap()
            .progress_chars("█▓▒░ "),
    );

    pb.set_message(format!(
        "Benchmarking [ Category: {}, Heuristic: {} ]",
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
        "✔ Benchmark Complete [ Category: {}, Heuristic: {} ]",
        category_name,
        heuristic.name()
    ));
    results
}

fn benchmark_level(level: &LevelData, heuristic: Heuristic) -> BenchmarkResult {
    let color_regions: Vec<Vec<u8>> = level
        .regions
        .iter()
        .map(|row| row.iter().map(|&v| v).collect())
        .collect();

    let heuristic_fn = heuristic.to_fn();

    let mut total_nanos = 0_u128;
    let mut steps_taken = 0_usize;
    let mut solved = false;

    let game_state = GameState::from_color_regions(color_regions, heuristic_fn)
        .expect("error initializing gamestate");

    for run in 0..NUM_RUNS {
        let timer = Instant::now();
        let (solution, steps) = depth_first_search(game_state.clone());
        let duration_nanos = timer.elapsed().as_nanos();

        total_nanos += duration_nanos;

        if run == 0 {
            steps_taken = steps;
            solved = solution.is_some();
        }
    }

    BenchmarkResult {
        id: level.id,
        size: level.size,
        duration_ns: total_nanos / NUM_RUNS,
        steps_taken,
        solved,
    }
}
