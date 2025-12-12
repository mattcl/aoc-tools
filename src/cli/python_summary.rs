use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use anyhow::{Context, Result, bail};
use clap::Args;
use comfy_table::{CellAlignment, Table, presets::ASCII_BORDERS_ONLY_CONDENSED};
use serde::{Deserialize, Serialize};

/// Given a python output from a conforming project's pytest benchmark run,
/// generate a summary table.
#[derive(Debug, Clone, Args)]
pub struct PythonSummary {
    benchmark_json: PathBuf,

    /// An optional path to a file containing additional label suffixes.
    ///
    /// This is expected as a json object in the format of "01" -> "some suffix".
    #[clap(short, long)]
    labels: Option<PathBuf>,

    /// Output a json representation to the specified path.
    #[clap(short, long)]
    output: Option<PathBuf>,
}

impl PythonSummary {
    pub fn run(&self) -> Result<()> {
        if !self.benchmark_json.is_file() {
            bail!("Supplied benchmark json path does not exist or is not a file");
        }

        let reader = File::open(&self.benchmark_json)?;
        let PytestOutput { benchmarks } = serde_json::from_reader(&reader)?;

        let label_mapping: HashMap<String, String> = if let Some(ref p) = self.labels {
            let reader = File::open(p)?;
            serde_json::from_reader(&reader)?
        } else {
            HashMap::default()
        };

        let mut rows = Vec::default();

        let mut total = 0.0_f64;

        for result in benchmarks.iter() {
            if !result.name.starts_with("test_day") {
                continue;
            }

            let day = result.name.replace("test_day", "");
            let time = result.stats.mean * 1000.0;

            total += time;

            let label = if let Some(suffix) = label_mapping.get(&day) {
                format!("{} {}", day, suffix)
            } else {
                day.clone()
            };

            rows.push(ReportRow {
                label,
                time_ms: time,
                percent_total_time: 0.0,
            });
        }

        for row in rows.iter_mut() {
            row.percent_total_time = row.time_ms / total * 100.0;
        }

        rows.sort_by(|a, b| a.label.cmp(&b.label));
        rows.push(ReportRow {
            label: "Total".into(),
            time_ms: total,
            percent_total_time: 100.0,
        });

        let mut table = Table::new();

        table.load_preset(ASCII_BORDERS_ONLY_CONDENSED);
        table.set_header(vec!["Problem", "Time (ms)", "% Total Time"]);

        for row in rows.iter() {
            table.add_row(vec![
                &row.label,
                &format!("{:.5}", row.time_ms),
                &format!("{:.3}", row.percent_total_time),
            ]);
        }

        table
            .column_mut(1)
            .unwrap()
            .set_cell_alignment(CellAlignment::Right);
        table
            .column_mut(2)
            .unwrap()
            .set_cell_alignment(CellAlignment::Right);

        println!("{}", table);

        if let Some(ref output) = self.output {
            println!("  Writing json to '{}'", output.display());
            let output = File::create(output).context("Failed to create output file")?;
            let mut writer = BufWriter::new(output);
            serde_json::to_writer(&mut writer, &rows).context("Failed to write output")?;
            writer.flush()?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PytestOutput {
    benchmarks: Vec<BenchmarkOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchmarkOutput {
    name: String,
    stats: Stats,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct Stats {
    mean: f64,
}

#[derive(Debug, Default, Clone, Serialize)]
struct ReportRow {
    label: String,
    time_ms: f64,
    percent_total_time: f64,
}
