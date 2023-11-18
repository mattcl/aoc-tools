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

#[derive(Debug, Default, Clone, Serialize)]
pub struct TransformedCSVRow {
    day: usize,
    participant: String,
    input: String,
    language: String,
    mean: f64,
    stddev: f64,
    median: f64,
    user: f64,
    system: f64,
    min: f64,
    max: f64,
}

impl TransformedCSVRow {
    pub fn from_original(
        original: OriginalCSVRow,
        day: usize,
        participant: String,
        language: String,
    ) -> Self {
        Self {
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
