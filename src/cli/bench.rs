use std::{path::PathBuf, process::Command};

use anyhow::{Result, bail, Context};
use clap::Args;
use itertools::Itertools;
use which::which;

use crate::{config::Config, util::day_directory_name, solution::Solutions, highlight, success};

/// Run comparative benchmarks for a given day between the config participants.
///
/// This produces a file named benches_raw.json in the directory for the given
/// day.
///
/// This requires hyperfine to be installed.
///
/// This assumes that the configured projects have all passed the
/// check-solutions command for the available inputs, implying that only inputs
/// for which there is a solution will be used.
#[derive(Debug, Clone, Args)]
pub struct Bench {
    /// The day.
    day: usize,

    /// The root directory where inputs are stored.
    ///
    /// This assumes a `<day>_<padded number>` directory structure containing
    /// the inputs.
    inputs: PathBuf,
}

impl Bench {
    pub fn run(&self, config: &Config) -> Result<()> {
        if which("hyperfine").is_err() {
            bail!("hyperfine must be installed");
        }

        println!("> Day: {} benchmarking", self.day);

        let day_directory_name = day_directory_name(self.day);
        let day_directory = self.inputs.join(&day_directory_name);

        if !day_directory.is_dir() {
            println!("> No inputs for day {}", self.day);
            return Ok(());
        }

        let solution_file = day_directory.join("solutions.json");

        if !solution_file.is_file() {
            println!("> No solved inputs for day {}", self.day);
            return Ok(());
        }

        let solutions = Solutions::from_file(solution_file)?;

        // get the official inputs
        let inputs = solutions.keys().filter(|n| n.starts_with("input-")).join(",");

        if inputs.is_empty() {
            bail!("unexpected: no official inputs in solutions file");
        }

        // we need to filter out the projects that will not solve the current
        // day by attempting to get a solution for any of the inputs
        let canary = day_directory.join(
            solutions
                .keys()
                .filter(|n| n.starts_with("input-"))
                .next()
                .unwrap() // unwrap is safe because we actually checked already
        ).canonicalize()?;

        let mut candidates: Vec<_> = config.participants()
            .iter()
            .filter(|(_, p)| matches!(p.solve(self.day, &canary), Ok(Some(_))))
            .collect();

        candidates.sort_by(|a, b| a.0.cmp(&b.0));

        if candidates.is_empty() {
            println!("  {}", highlight!("No participants solve the specified day"));
            return Ok(());
        }

        let mut cmd = Command::new("hyperfine");
        cmd.current_dir(&day_directory);
        cmd.env("AOC_DAY", self.day.to_string());
        cmd.args([
            // warmup 3 times
            "-w", "3",
            // at least 10 runs
            "-m", "10",
            // at most 100 runs
            "-M", "200",
            // iterate for each input
            "-L", "input", &inputs,
            // output json
            "--export-json", "benches_raw.json",
        ]);

        // give names to the commands and indicate the projects we're benching
        println!("  Benchmarking the following projects:");
        for (name, _) in candidates.iter() {
            cmd.args(["-n", &format!("{} {{input}}", name)]);
            println!("  {}", success!(name));
        }

        for (_, project) in candidates.iter() {
            cmd.arg(format!("AOC_INPUT={{input}} {}", project.entrypoint()));
        }

        let status = cmd.status().context("Failed to execute hyperfine")?;
        if !status.success() {
            bail!("hyperfine did not exit successfully")
        }

        Ok(())
    }
}
