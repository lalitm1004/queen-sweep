use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BenchmarkResult {
    pub id: u32,
    pub size: u32,
    pub duration_nanos: u128,
    pub steps_taken: usize,
    pub solved: bool,
}

impl BenchmarkResult {
    pub fn new(id: u32, size: u32, duration_nanos: u128, steps_taken: usize, solved: bool) -> Self {
        Self {
            id,
            size,
            duration_nanos,
            steps_taken,
            solved,
        }
    }
}
