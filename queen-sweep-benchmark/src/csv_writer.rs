use csv::Writer;
use std::{fs::File, io};

use crate::benchmark::BenchmarkResult;

pub fn write_results_csv(results: &[BenchmarkResult], output_path: &str) -> io::Result<()> {
    let file = File::create(output_path)?;

    let mut writer = Writer::from_writer(file);

    for result in results {
        writer.serialize(result)?;
    }

    writer.flush()?;
    Ok(())
}
