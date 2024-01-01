use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Context, Result};
use clap::Args;
use itertools::Itertools;
use which::which;

use crate::{
    aoc_project::AocProject,
    attention,
    bench_data::{BenchCSVRow, OriginalCSVRow},
    config::Config,
    highlight,
    solution::Solutions,
    success,
    util::day_directory_name,
};

/// Run comparative benchmarks for a given day between the config participants.
///
/// This produces a files named benches.md and benches_raw.csv in the directory
/// for the given day.
///
/// This requires hyperfine to be installed.
///
/// This assumes that the configured projects have all passed the
/// check-solutions command for the available inputs, implying that only inputs
/// for which there is a solution will be used. A project will not be benched on
/// a given day if its runtime exceeds the configured timeout.
#[derive(Debug, Clone, Args)]
pub struct Bench {
    /// The year.
    year: usize,

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
        let day_directory = self.inputs.join(day_directory_name);

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

        // get the first N official inputs
        let inputs_raw: Vec<_> = solutions
            .keys()
            .filter(|n| n.starts_with("input-"))
            .take(config.max_inputs())
            .collect();

        println!("> Using the selected inputs (max {}):", config.max_inputs());
        for input in inputs_raw.iter() {
            println!("    {}", input);
        }
        println!();

        if inputs_raw.is_empty() {
            bail!("unexpected: no official inputs in solutions file");
        }

        let inputs = inputs_raw.iter().join(",");

        // we need to filter out the projects that will not solve the current
        // day by attempting to get a solution for any of the inputs
        let canary = day_directory
            .join(
                solutions.keys().find(|n| n.starts_with("input-")).unwrap(), // unwrap is safe because we actually checked already
            )
            .canonicalize()?;

        // we attempt to solve all the participants' solutions for the given day
        // and the canary file, further requiring that they complete in under
        // the specified timeout
        let mut candidates: Vec<_> = config
            .participants()
            .iter()
            .filter(|(_, p)| {
                matches!(
                    p.solve(self.year, self.day, &canary, Some(config.timeout())),
                    Ok(Some(_))
                )
            })
            .collect();

        candidates.sort_by(|a, b| a.0.cmp(b.0));

        if candidates.is_empty() {
            println!(
                "  {}",
                highlight!("No participants solve the specified day")
            );
            return Ok(());
        }

        let mut cmd = Command::new("hyperfine");
        cmd.current_dir(&day_directory);
        cmd.env("AOC_YEAR", self.year.to_string());
        cmd.env("AOC_DAY", self.day.to_string());
        cmd.env("AOC_CI", "true");
        cmd.args([
            // warmup 2 times
            "-w",
            "2",
            // at least 3 runs
            "-m",
            "3",
            // at most 100 runs
            "-M",
            "200",
            // iterate for each input
            "-L",
            "input",
            &inputs,
            // sort by the execution time instead of order of specification
            "--sort",
            "mean-time",
            // export in various formats
            "--export-markdown",
            "benches.md",
            "--export-csv",
            "benches_raw.csv",
        ]);

        println!("  Benchmarking the following projects:");
        for (name, _) in candidates.iter() {
            println!("  {}", success!(name));
        }

        for (_, project) in candidates.iter() {
            cmd.arg(format!(
                "AOC_INPUT={{input}} {}",
                project.bench_entrypoint()
            ));
        }

        let status = cmd.status().context("Failed to execute hyperfine")?;
        if !status.success() {
            bail!("hyperfine did not exit successfully");
        }

        // write the list of participants for this day's benchmarks
        let mut participants_record: Vec<_> = candidates.iter().map(|(name, _)| name).collect();
        participants_record.sort();
        let output = File::create(day_directory.join("participants.json"))
            .context("Failed to create participants file")?;
        let mut writer = BufWriter::new(output);
        serde_json::to_writer(&mut writer, &participants_record)
            .context("Failed to write participants")?;
        writer.flush()?;

        // Hyperfine's combinations of command names and inputs don't allow for
        // what we want to do, so we're going to rewrite the contents of the
        // generated bench markdown to be what we want. This is memory
        // inefficient with all the string replacements, but it probably won't
        // be too bad. If at some point hyperfine allows for us to do this in a
        // better way, we can get rid of this code.
        let mut bench_contents = std::fs::read_to_string(day_directory.join("benches.md"))?;

        // we're going to change the header of the markdown table
        bench_contents = bench_contents.replacen("Command", "Participant | Input", 1);
        // and the alignment spec
        bench_contents = bench_contents.replacen(":---", ":---|:---", 1);

        // now we're going to replace the command name and add the inputs in a
        // separate column.
        for (name, project) in candidates.iter() {
            for input_name in inputs_raw.iter() {
                let needle = format!("`AOC_INPUT={} {}`", input_name, project.bench_entrypoint());
                let replacement = format!("{} | {}", name, input_name);
                bench_contents = bench_contents.replacen(&needle, &replacement, 1);
            }
        }

        // finally, write that file back to disk
        let mut bench_out = File::create(day_directory.join("benches.md"))?;
        bench_out.write_all(bench_contents.as_bytes())?;

        // We want to do something similar for the raw csv
        let bench_csv = day_directory.join("benches_raw.csv");
        transform_bench_csv(bench_csv, self.year, self.day, &candidates)
            .context("Could not transform bench csv")?;

        Ok(())
    }
}

fn transform_bench_csv<P: AsRef<Path>>(
    path: P,
    year: usize,
    day: usize,
    candidates: &[(&String, &AocProject)],
) -> Result<()> {
    let path = path.as_ref();
    let raw_bench_data: Vec<OriginalCSVRow> = {
        let mut out = Vec::default();
        let mut reader = csv::Reader::from_path(path)?;
        for result in reader.deserialize() {
            out.push(result?);
        }
        out
    };

    let lookup_map: HashMap<&str, (&str, &str)> = candidates
        .iter()
        .map(|(name, project)| {
            (
                project.bench_entrypoint(),
                (name.as_str(), project.language()),
            )
        })
        .collect();

    let mut transformed = Vec::with_capacity(raw_bench_data.len());

    for record in raw_bench_data {
        let entrypoint = record.get_raw_command();
        if let Some((name, language)) = lookup_map.get(&entrypoint.as_str()) {
            transformed.push(BenchCSVRow::from_original(
                record,
                year,
                day,
                name.to_string(),
                language.to_string(),
            ));
        } else {
            println!(
                "  {}",
                attention!(format!("Could not transform record for `{}`", &entrypoint))
            );
        }
    }

    let mut writer = csv::Writer::from_path(path)?;
    for record in transformed {
        writer.serialize(record)?;
    }
    writer.flush()?;
    Ok(())
}
