mod benchmark;
mod heuristic;
#[allow(dead_code)]
mod levels;

use std::{fs, path::Path};

use crate::{
    benchmark::{benchmark_levels, write_to_csv},
    heuristic::Heuristic,
    levels::{LevelData, load_base_levels, load_bonus_levels},
};

const STATS_DIR: &'static str = "stats";

fn main() {
    let stats_dir = Path::new(STATS_DIR);
    fs::create_dir_all(stats_dir).expect("Failed to create output directory");

    benchmark("base", load_base_levels());
    benchmark("bonus", load_bonus_levels());
}

fn benchmark(category: &'static str, levels: Vec<LevelData>) {
    println!("Category: {} ({} levels)", category, levels.len());

    let heuristics = Heuristic::all();

    for heuristic in heuristics {
        let output_file =
            Path::new(STATS_DIR).join(format!("{}_{}.csv", category, heuristic.name()));

        let result = benchmark_levels(&levels, category, heuristic);
        write_to_csv(output_file, &result);
    }
}
