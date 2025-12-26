# queen-sweep-benchmark
This is a benchmark binary for the core solver engine.

### - `scrapeLevels.ts`
This script fetches and parses level data from an open source [repository](https://github.com/samimsu/queens-game-linkedin)

### - Rust Binary
The binary benchmarks the core solver engine measuring the following statistics per level:
```rs
pub struct BenchmarkResult {
    pub id: u32,
    pub size: u32,
    pub duration_ns: u128,
    pub steps_taken: usize,
    pub solved: bool,
}
```
The `duration_ns` field is averaged over 5 runs.

The per level statistics are then dumped into a `.csv` file in the `stats/` directory. It keeps track of which category and heuristic the statistic belongs to.

### - `process_benchmark.py`
A simple python script that loads up the previously generated csv files and calculates more statistics and then eventually visualizes them as graphs