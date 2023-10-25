use std::{collections::HashMap, path::Path};

use anyhow::{Context, Result};
use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;

use crate::aoc_project::AocProject;

#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize)]
pub struct Config {
    participants: HashMap<String, AocProject>,
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
}
