use std::path::PathBuf;

use anyhow::{Result, bail};
use clap::Args;

use crate::{
    attention,
    bench_data::{BenchCSVRow, load_benches},
    highlight,
    util::day_directory_name,
};

/// Combine the `benches_raw.csv` files for every day into a single CSV file.
#[derive(Debug, Clone, Args)]
pub struct UnifyBenches {
    /// The root directory where bench data is stored.
    ///
    /// This assumes a `<day>_<padded number>` directory structure containing
    /// the benches_raw.csv.
    inputs: PathBuf,

    /// The path to store the unified output file.
    #[clap(short, long)]
    output: PathBuf,
}

impl UnifyBenches {
    pub fn run(&self) -> Result<()> {
        if !self.inputs.is_dir() {
            bail!("Input directory does not exist: {}", self.inputs.display());
        }

        let mut unified: Vec<BenchCSVRow> = Vec::default();

        for day in 1..=25 {
            let raw_csv = self
                .inputs
                .join(day_directory_name(day))
                .join("benches_raw.csv");

            if !raw_csv.is_file() {
                println!("> No benches for day {day}");
                continue;
            } else {
                println!("> Reading data for day {day}");
            }

            load_benches(raw_csv, &mut unified)?;

            println!();
        }

        if unified.is_empty() {
            println!(
                "  {}",
                attention!("No benchmark data found. Not writing output.")
            );
            return Ok(());
        }

        println!(
            "> Writing unified data to {}",
            highlight!(self.output.display())
        );

        let mut writer = csv::Writer::from_path(&self.output)?;
        for record in unified {
            writer.serialize(record)?;
        }
        writer.flush()?;

        Ok(())
    }
}
