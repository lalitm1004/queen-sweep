mod benchmark;
mod csv_writer;
mod heuristic;
mod levels;
mod loader;

use std::fs;

use benchmark::benchmark_levels_with_progress;
use csv_writer::write_results_csv;
use heuristic::Heuristic;

fn main() {
    println!("Loading levels...");
    let base_levels = loader::load_base_levels();
    let bonus_levels = loader::load_bonus_levels();
    let community_levels = loader::load_community_levels();

    println!("  Base levels: {}", base_levels.len());
    println!("  Bonus levels: {}", bonus_levels.len());
    println!("  Community levels: {}", community_levels.len());
    println!();

    let heuristics = Heuristic::all();
    let categories = vec![
        ("base", &base_levels),
        ("bonus", &bonus_levels),
        ("community", &community_levels),
    ];

    fs::create_dir_all("stats/").expect("Failed to create output directory");

    for heuristic in &heuristics {
        println!(
            "\n----- Running benchmarks with {} -----\n",
            heuristic.name()
        );

        for (category_name, levels) in &categories {
            let results = benchmark_levels_with_progress(levels, category_name, *heuristic);

            let filename = format!("stats/{}_{}.csv", category_name, heuristic.name());
            write_results_csv(&results, &filename)
                .unwrap_or_else(|e| eprintln!("Failed to write {}: {}", filename, e));
        }
    }

    println!("✔ Benchmark complete!");
}
