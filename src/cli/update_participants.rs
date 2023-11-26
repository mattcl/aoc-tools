use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::Args;
use reqwest::header::CONTENT_TYPE;
use serde::Serialize;
use url::Url;

use crate::{aoc_project::AocProject, config::Config};

use super::ApiCommon;

/// Update the participants in aoc-web with the participants specified in the
/// config.
#[derive(Debug, Clone, Args)]
pub struct UpdateParticipants {
    /// The config file to use.
    ///
    /// This is required.
    #[arg(short, long, required = true, env = "AOC_TOOLS_CONFIG")]
    config: PathBuf,

    #[clap(flatten)]
    api: ApiCommon,
}

impl UpdateParticipants {
    pub fn run(&self) -> Result<()> {
        let config = Config::load(&self.config)?;
        let year = config.year();

        let participants: Vec<_> = config
            .participants()
            .values()
            .map(|p| Participant::new(year, p))
            .collect();

        let endpoint = self.api.api_base.join("v1/participants")?;

        println!(
            "> Updating {} participants at {}",
            participants.len(),
            &endpoint
        );

        let client = reqwest::blocking::Client::new();
        let res = client
            .post(endpoint)
            .bearer_auth(&self.api.api_token)
            .header(CONTENT_TYPE, "application/json")
            .json(&participants)
            .send()
            .context("Failed to update participants")?;

        if !res.status().is_success() {
            bail!("Update was unsuccessful.\n{:#?}", res);
        }

        println!("> Done");

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Participant<'a> {
    year: usize,
    name: &'a str,
    language: &'a str,
    repo: &'a Url,
}

impl<'a> Participant<'a> {
    pub fn new(year: usize, p: &'a AocProject) -> Self {
        Self {
            year,
            name: p.username(),
            language: p.language(),
            repo: p.repo(),
        }
    }
}
