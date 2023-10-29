use std::path::PathBuf;

use anyhow::{anyhow, bail, Context, Result};
use clap::Args;

use crate::{
    config::Config,
    failure, highlight,
    solution::{Solution, Solutions},
    success,
    util::day_directory_name,
};

/// Checks the specified participant's solutions.
///
/// This runs with the official and challenge inputs (if available). Failing to
/// solve challenge inputs does not count as an overall failure when checking.
#[derive(Debug, Clone, Args)]
pub struct CheckSolutions {
    /// The particpatnt's solutions to check.
    ///
    /// This participant must exist in the config.
    participant: String,

    /// The root directory where inputs are stored.
    ///
    /// This assumes a `<day>_<padded number>` directory structure containing
    /// the inputs.
    inputs: PathBuf,
}

impl CheckSolutions {
    pub fn run(&self, config: &Config) -> Result<()> {
        if !self.inputs.is_dir() {
            bail!("Inputs must exist and be a directory");
        }

        let project = config
            .participants()
            .get(&self.participant)
            .ok_or_else(|| anyhow!("Particpant does not exist: {}", &self.participant))?;

        'days: for day in 1..=25 {
            println!();
            let day_directory_name = day_directory_name(day);
            let day_directory = self.inputs.join(&day_directory_name);

            if !day_directory.is_dir() {
                println!("> No inputs for day {}", day);
                continue;
            }

            let solution_file = day_directory.join("solutions.json");

            if !solution_file.is_file() {
                println!("> No solutions for day {}", day);
                continue;
            }

            let solutions = Solutions::from_file(solution_file)?;

            // We can determine the input file names from the solutions we've
            // parsed. This prevents us from attempting to check an input for
            // which we have no reference solution.

            // We're going to start with the official inputs, which have naming
            // format like `input-<name>`.

            println!("> Day {}:\n  Checking official inputs", day);

            for (input_name, solution) in solutions
                .iter()
                .filter(|(name, _)| name.starts_with("input-"))
            {
                let input_file = day_directory.join(input_name).canonicalize()?;

                // This would be unexpected but maybe not impossible. Let's skip
                // if this ends up being the case
                if !input_file.is_file() {
                    println!(
                        "  {}",
                        highlight!(format!("No file found for solution: {}", input_name))
                    );
                    continue;
                }

                if let Some(computed) = project
                    .solve(day, &input_file)
                    .context("Failed to produce solution")?
                {
                    if !self.check_solution(day, input_name, solution, &computed) {
                        bail!("Solution incorrect");
                    }
                } else {
                    println!(
                        "  {}",
                        highlight!("Project does not implement a solution for this day. Skipping.")
                    );
                    continue 'days;
                }
            }

            // For challenge inputs we do not fail on failures.
            println!("\n  Checking challenge inputs");

            for (input_name, solution) in solutions
                .iter()
                .filter(|(name, _)| name.starts_with("challenge-input-"))
            {
                let input_file = day_directory.join(input_name).canonicalize()?;

                // This would be unexpected but maybe not impossible. Let's skip
                // if this ends up being the case
                if !input_file.is_file() {
                    println!(
                        "  {}",
                        highlight!(format!("No file found for solution: {}", input_name))
                    );
                    continue;
                }

                match project.solve(day, &input_file) {
                    Ok(Some(computed)) => {
                        self.check_solution(day, input_name, solution, &computed);
                    }
                    Ok(None) => {
                        // it should not be possible for us to get here but just
                        // implement this anyway.
                        println!(
                            "  {}",
                            highlight!(
                                "Project does not implement a solution for this day. Skipping."
                            )
                        );
                        continue 'days;
                    }
                    Err(_) => {
                        println!(
                            "  {}",
                            failure!("Project did not successfully produce a solution.")
                        );
                    }
                }
            }
        }

        Ok(())
    }

    fn check_solution(
        &self,
        day: usize,
        input: &str,
        expected: &Solution,
        actual: &Solution,
    ) -> bool {
        // handle last day not having part two
        let cmp = if day == 25 {
            expected.last_day_compare(actual)
        } else {
            expected.string_compare(actual)
        };

        if cmp {
            // we've passed
            println!("  {} {}", input, success!("Ok"));
            true
        } else {
            println!("  {} {}", input, failure!("Failed"));
            println!("Expected:\n{}\n\n But got:\n{}", expected, actual);
            false
        }
    }
}
