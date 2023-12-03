use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::config::Config;

/// Displays the debug contents of the config
#[derive(Debug, Clone, Args)]
pub struct CheckConfig {
    /// The config file to use
    #[arg(env = "AOC_TOOLS_CONFIG")]
    config: PathBuf,
}

impl CheckConfig {
    pub fn run(&self) -> Result<()> {
        let config = Config::load(&self.config)?;

        println!("{:#?}", config);

        Ok(())
    }
}
