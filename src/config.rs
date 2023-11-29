use std::{collections::BTreeMap, path::Path};

use anyhow::{Context, Result};
use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;
use url::Url;

use crate::aoc_project::AocProject;

fn default_timeout() -> usize {
    30
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct Config {
    general: General,
    participants: BTreeMap<String, AocProject>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct General {
    year: usize,
    pipeline_url: Url,
    #[serde(default = "default_timeout")]
    timeout: usize,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        Figment::new()
            .merge(Toml::file(path))
            .extract()
            .context("Invalid config file")
    }

    pub fn participants(&self) -> &BTreeMap<String, AocProject> {
        &self.participants
    }

    pub fn year(&self) -> usize {
        self.general.year
    }

    pub fn pipeline_url(&self) -> &Url {
        &self.general.pipeline_url
    }

    pub fn timeout(&self) -> usize {
        self.general.timeout
    }
}
