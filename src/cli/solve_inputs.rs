use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use anyhow::{anyhow, bail, Context, Result};
use clap::Args;
use walkdir::WalkDir;

use crate::{
    config::Config,
    highlight,
    solution::Solutions,
    success,
    util::{day_directory_name, sanitize_value_for_display},
};

/// Solve all the available inputs and store their solutions.
///
/// This will create a solutions.json file for each day's worth of inputs, and
/// assumes the specified inputs path to have paths like day_001, day_002, etc.
#[derive(Debug, Clone, Args)]
pub struct SolveInputs {
    /// The root directory where inputs are stored.
    ///
    /// This assumes a `<day>_<padded number>` directory structure containing
    /// the inputs.
    inputs: PathBuf,
}

impl SolveInputs {
    pub fn run(&self, config: &Config) -> Result<()> {
        if !self.inputs.is_dir() {
            bail!("Inputs must exist and be a directory");
        }

        let (_, solver) = config
            .participants()
            .iter()
            .find(|(_, p)| p.is_solver())
            .ok_or_else(|| anyhow!("Config does not specify at one participant as the solver"))?;

        'days: for day in 1..=25 {
            println!();

            let day_directory_name = day_directory_name(day);
            let day_directory = self.inputs.join(&day_directory_name);

            if !day_directory.is_dir() {
                println!("> No inputs for day {}", day);
                continue;
            }

            println!("> Day {}: solving inputs for day", day);

            // the BTreeMap should mean the generated json is stable instead of
            // being sensitive to changing key ordering with a HashMap
            let mut solutions = Solutions::default();

            for entry in WalkDir::new(&day_directory)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let filename = entry.file_name().to_string_lossy();

                // skip non-inputs
                if !(filename.starts_with("input-") || filename.starts_with("challenge-input")) {
                    continue;
                }

                let input = entry.path().canonicalize()?;
                if let Some(solution) = solver.solve(day, &input).with_context(|| {
                    format!("Failed to solve day {} for input {}", day, filename)
                })? {
                    solutions.insert(filename.to_string(), solution);
                } else {
                    println!(
                        "  {}",
                        highlight!(format!(
                            "Solver does not implement a solution for day {}",
                            day
                        ))
                    );
                    // don't bother with the rest of the input files or writing
                    // out solutions
                    continue 'days;
                }

                println!("  {}", success!(format!("Solved {}", filename)));
            }

            // create a file to store the json solutions and write the solutions
            let output = File::create(day_directory.join("solutions.json"))
                .context("Failed to create file")?;
            let mut writer = BufWriter::new(output);
            serde_json::to_writer(&mut writer, &solutions)
                .context("Failed to serialize to writer")?;
            writer.flush()?;

            // create a file to store the markdown solutions and write the
            // solutions
            let mut solutions_markdown = vec![
                "| Input | Part One | Part Two |".to_string(),
                "|:---|:---|:---|".to_string(),
            ];

            for (name, solution) in solutions.iter() {
                solutions_markdown.push(format!(
                    "|{}|<pre>{}</pre>|<pre>{}</pre>|",
                    name,
                    sanitize_value_for_display(solution.part_one()),
                    sanitize_value_for_display(solution.part_two()),
                ));
            }

            let markdown = solutions_markdown.join("\n");

            let mut output = File::create(day_directory.join("solutions.md"))
                .context("Failed to create file")?;
            output
                .write_all(markdown.as_bytes())
                .context("Failed to write file")?;
        }

        Ok(())
    }
}
