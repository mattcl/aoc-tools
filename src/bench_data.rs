use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct OriginalCSVRow {
    command: String,
    mean: f64,
    stddev: f64,
    median: f64,
    user: f64,
    system: f64,
    min: f64,
    max: f64,
    parameter_input: String,
}

impl OriginalCSVRow {
    pub fn get_raw_command(&self) -> String {
        let needle = format!("AOC_INPUT={} ", &self.parameter_input);
        self.command.replace(&needle, "")
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BenchCSVRow {
    pub year: usize,
    pub day: usize,
    pub participant: String,
    pub input: String,
    pub language: String,
    pub mean: f64,
    pub stddev: f64,
    pub median: f64,
    pub user: f64,
    pub system: f64,
    pub min: f64,
    pub max: f64,
}

impl BenchCSVRow {
    pub fn from_original(
        original: OriginalCSVRow,
        year: usize,
        day: usize,
        participant: String,
        language: String,
    ) -> Self {
        Self {
            year,
            day,
            participant,
            input: original.parameter_input,
            language,
            mean: original.mean,
            stddev: original.stddev,
            median: original.median,
            user: original.user,
            system: original.system,
            min: original.min,
            max: original.max,
        }
    }
}

pub fn load_benches<P: AsRef<Path>>(path: P, out: &mut Vec<BenchCSVRow>) -> Result<()> {
    let mut reader = csv::Reader::from_path(path).context("Failed to parse csv")?;

    for result in reader.deserialize() {
        out.push(result?);
    }

    Ok(())
}
