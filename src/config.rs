use std::{collections::HashMap, path::Path};

use anyhow::{Context, Result};
use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;
use url::Url;

use crate::aoc_project::AocProject;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct Config {
    general: General,
    participants: HashMap<String, AocProject>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct General {
    year: usize,
    pipeline_url: Url,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        Figment::new()
            .merge(Toml::file(path))
            .extract()
            .context("Invalid config file")
    }

    pub fn participants(&self) -> &HashMap<String, AocProject> {
        &self.participants
    }

    pub fn year(&self) -> usize {
        self.general.year
    }

    pub fn pipeline_url(&self) -> &Url {
        &self.general.pipeline_url
    }
}