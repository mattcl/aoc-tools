use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::Args;
use reqwest::header::CONTENT_TYPE;

use crate::bench_data::BenchCSVRow;

use super::ApiCommon;

/// Publish a unified benches CSV to an aoc-web service, regenerating summaries
/// afterward.
///
/// This requires the AOC_TOOLS_API_TOKEN var to be set, unless you want to
/// specify it on as an argument, which is not recommended.
#[derive(Debug, Clone, Args)]
pub struct PublishBenches {
    /// The path to a unified benches CSV.
    benches: PathBuf,

    #[clap(flatten)]
    api: ApiCommon,
}

impl PublishBenches {
    pub fn run(&self) -> Result<()> {
        if !self.benches.is_file() {
            bail!("Benches path does not exist or is not a file");
        }

        let benches: Vec<BenchCSVRow> = {
            let mut out = Vec::default();
            let mut reader = csv::Reader::from_path(&self.benches)?;
            for result in reader.deserialize() {
                out.push(result?);
            }
            out
        };

        if benches.is_empty() {
            bail!("Specified benches CSV is empty");
        }

        let client = reqwest::blocking::Client::new();

        let publish_endpoint = self.api.api_base.join("v1/benchmarks")?;
        let generate_endpoint = self.api.api_base.join("v1/summaries/generate")?;

        println!(
            "> Publishing {} benches to {}",
            benches.len(),
            &publish_endpoint
        );

        // first publish the benchmarks
        let res = client
            .post(publish_endpoint)
            .bearer_auth(&self.api.api_token)
            .header(CONTENT_TYPE, "application/json")
            .json(&benches)
            .send()
            .context("Failed to publish benches")?;

        if !res.status().is_success() {
            bail!("Publish was unsuccessful.\n{:#?}", res);
        }

        println!("> Regenerating summaries via {}", &generate_endpoint);
        // now we want to regenerate the summaries corresponding to the current
        // year, which we can determine by getting the first benchmark since all
        // the years _should_ be the same from a unified CSV.
        let year = benches[0].year;

        let res = client
            .post(generate_endpoint)
            .bearer_auth(&self.api.api_token)
            .header(CONTENT_TYPE, "application/json")
            .json(&year)
            .send()
            .context("Failed to generate summaries")?;

        if !res.status().is_success() {
            bail!("Summary generation was unsuccessful.\n{:#?}", res);
        }

        println!("> Done");

        Ok(())
    }
}
