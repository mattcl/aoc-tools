use std::{collections::BTreeMap, path::PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use clap::Args;
use comfy_table::{presets::ASCII_BORDERS_ONLY_CONDENSED, CellAlignment, Table};
use serde::Deserialize;
use walkdir::WalkDir;

/// Given a project conforming to the rust template, generate a benchmark summary.
///
/// This will probably not work for anything else.
#[derive(Debug, Clone, Args)]
pub struct CriterionSummary {
    /// The path to the criterion target
    criterion_path: PathBuf,
}

impl CriterionSummary {
    pub fn run(&self) -> Result<()> {
        // we're looking for target/criterion/<day>...

        if !self.criterion_path.is_dir() {
            bail!("Supplied criterion path does not exist or is not a directory");
        }

        let mut records: BTreeMap<String, f64> = BTreeMap::default();
        let mut total = 0.0;

        for entry in WalkDir::new(&self.criterion_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let dir_name = entry.file_name().to_string_lossy();

            // we know our combined benches start with "Combined"
            if !dir_name.starts_with("Combined") {
                continue;
            }

            // just sanity check to make sure we don't have some other combined
            // that isn't for a particular day
            let parent_name = entry
                .path()
                .parent()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy())
                .unwrap_or_default();

            // we know all of our day entries are padded up to 3 digits with
            // zeroes, meaning that any path starting with a zero must be a day
            // entry.
            if !parent_name.starts_with('0') {
                continue;
            }

            // now that we know we have a valid entry, we're going to find the
            // new/raw.csv if we can, and, if not, fall back to base/raw.csv
            let mut data_csv = entry.path().join("new/raw.csv");
            if !data_csv.is_file() {
                data_csv = entry.path().join("base/raw.csv");
            }

            // we now need the last line of that CSV
            let mut reader = csv::Reader::from_path(&data_csv)?;

            let last: Record = reader
                .deserialize()
                .last()
                .ok_or_else(|| anyhow!("unable to process row from {}", data_csv.display()))?
                .context("Failed to deserialize record")?;

            let per_iter = last.per_iter_ms();
            records.insert(parent_name.to_string(), per_iter);
            total += per_iter;
        }

        let mut table = Table::new();
        table.load_preset(ASCII_BORDERS_ONLY_CONDENSED);
        table.set_header(vec!["Problem", "Time (ms)", "% Total Time"]);

        let mut percent_sum = 0.0;
        for (problem, per_iter) in records.iter() {
            let percent = per_iter / total * 100.0;
            percent_sum += percent;
            table.add_row(vec![
                problem,
                &format!("{:.5}", per_iter),
                &format!("{:.3}", percent),
            ]);
        }

        table.add_row(vec![
            "Total",
            &format!("{:.5}", total),
            &format!("{:.3}", percent_sum),
        ]);
        table
            .column_mut(1)
            .unwrap()
            .set_cell_alignment(CellAlignment::Right);
        table
            .column_mut(2)
            .unwrap()
            .set_cell_alignment(CellAlignment::Right);

        println!("{}", table);

        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy, Deserialize)]
struct Record {
    sample_measured_value: f64,
    iteration_count: usize,
}

impl Record {
    pub fn per_iter_ms(&self) -> f64 {
        self.sample_measured_value / self.iteration_count as f64 / 1_000_000.0
    }
}
