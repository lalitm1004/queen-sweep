use std::{fs::File, path::Path};

use csv::Writer;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BenchmarkResult {
    pub id: u32,
    pub size: u32,
    pub duration_ns: u128,
    pub steps_taken: usize,
    pub solved: bool,
}

pub fn write_to_csv<P: AsRef<Path>>(path: P, results: &[BenchmarkResult]) {
    let path = path.as_ref();
    let mut writer = create_csv_writer(path);

    for res in results {
        if let Err(err) = writer.serialize(res) {
            eprintln!("Failed to write CSV record to {}: {}", path.display(), err);
            std::process::exit(1);
        }
    }

    if let Err(err) = writer.flush() {
        eprintln!("Failed to flush CSV writer for {}: {}", path.display(), err);
        std::process::exit(1);
    }
}

#[inline]
fn create_csv_writer(path: &Path) -> Writer<File> {
    match File::create(path) {
        Ok(file) => Writer::from_writer(file),
        Err(err) => {
            eprintln!("Failed to create CSV file {}: {}", path.display(), err);
            std::process::exit(1);
        }
    }
}
