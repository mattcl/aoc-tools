use std::{collections::HashSet, fs::File, io::Write, path::PathBuf};

use anyhow::{Context, Result};
use clap::Args;
use minijinja::{context, Environment};

use crate::{config::Config, highlight, success, util::day_directory_name};

/// Generates a benchmark report for a given day.
#[derive(Debug, Clone, Args)]
pub struct Report {
    /// The day
    day: usize,

    /// The root directory where inputs, solutions, and benches are stored.
    ///
    /// This assumes a `<day>_<padded number>` directory structure containing
    /// the inputs, solutions, participants, and benchmark results.
    inputs: PathBuf,

    /// The report template
    ///
    /// This is a minijinja-compliant template.
    #[arg(short, long)]
    template: PathBuf,

    /// Output to the specified file instead of stdout.
    ///
    /// If a relative path is specified, this path is _relative to the day
    /// directory inside of the input dir_.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

impl Report {
    pub fn run(&self, config: &Config) -> Result<()> {
        // load the template engine
        let mut env = Environment::new();
        let report_raw =
            std::fs::read_to_string(&self.template).context("Failed to read template file")?;
        env.add_template("report", &report_raw)
            .context("Failed to add template to engine")?;
        let report_template = env
            .get_template("report")
            .context("Failed to get template")?;

        let day_directory_name = day_directory_name(self.day);
        let day_directory = self.inputs.join(day_directory_name);

        if !day_directory.is_dir() {
            eprintln!(
                "{}",
                highlight!(format!(
                    "The computed input directory does not exist. Doing nothing. ({})",
                    day_directory.display()
                ))
            );
            return Ok(());
        }

        // we need to determine the list of participants that actually solve
        // this day's problem
        let participants_raw = std::fs::read_to_string(day_directory.join("participants.json"))
            .context("Could not open participants file")?;
        let participant_names: HashSet<String> = serde_json::from_str(&participants_raw)
            .context("Failed to deserialize participants")?;

        let participants: Vec<_> = config
            .participants()
            .iter()
            .filter(|(n, _)| participant_names.contains(*n))
            .map(|(_, p)| p)
            .collect();

        // fetch the benchmarks table
        let official_benchmarks = std::fs::read_to_string(day_directory.join("benches.md"))
            .context("Could not open benchmark file")?;

        // fetch the solutions table
        let solutions = std::fs::read_to_string(day_directory.join("solutions.md"))
            .context("Could not open solution file")?;

        let rendered = report_template
            .render(context! {
                year => config.year(),
                day => self.day,
                pipeline_url => config.pipeline_url(),
                participants,
                official_benchmarks,
                solutions
            })
            .context("Failed to render template")?;

        if let Some(ref output_dest) = self.output {
            println!(
                "  {}",
                success!(format!("Writing output to '{}'", output_dest.display()))
            );
            let outpath = if output_dest.is_relative() {
                day_directory.join(output_dest)
            } else {
                output_dest.clone()
            };

            let mut outfile = File::create(outpath).context("Failed to create output file")?;
            outfile
                .write_all(rendered.as_bytes())
                .context("Failed to write output file")?;
        } else {
            println!("{}", rendered);
        }

        Ok(())
    }
}
