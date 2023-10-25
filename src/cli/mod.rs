use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::config::Config;

mod copy_inputs;
mod solve_inputs;

#[macro_export]
macro_rules! attention {
    ($msg:expr) => {
        console::Style::new().magenta().apply_to($msg)
    };
}

#[macro_export]
macro_rules! highlight {
    ($msg:expr) => {
        console::Style::new().yellow().apply_to($msg)
    };
}

#[macro_export]
macro_rules! success {
    ($msg:expr) => {
        console::Style::new().green().apply_to($msg)
    };
}

#[macro_export]
macro_rules! failure {
    ($msg:expr) => {
        console::Style::new().red().apply_to($msg)
    };
}

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    /// The config file to use.
    ///
    /// This is required.
    #[arg(short, long, required = true, env = "AOC_TOOLS_CONFIG")]
    config: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn run() -> Result<()> {
        let cli = Self::parse();

        // load the config since it's poetentially used everywhere
        let config = Config::load(&cli.config)?;

        cli.command.run(&config)
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    CopyInputs(copy_inputs::CopyInputs),
    SolveInputs(solve_inputs::SolveInputs),
}

impl Commands {
    pub fn run(&self, config: &Config) -> Result<()> {
        match self {
            Self::CopyInputs(cmd) => cmd.run(config),
            Self::SolveInputs(cmd) => cmd.run(config),
        }
    }
}
