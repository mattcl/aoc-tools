use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::config::Config;

mod bench;
mod check_solutions;
mod copy_inputs;
mod report;
mod solve_inputs;
mod summary;

/// CI-related commands
#[derive(Debug, Clone, Args)]
pub struct Ci {
    /// The config file to use.
    ///
    /// This is required.
    #[arg(short, long, required = true, env = "AOC_TOOLS_CONFIG")]
    config: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

impl Ci {
    pub fn run(&self) -> Result<()> {
        // load the config since it's poetentially used everywhere
        let config = Config::load(&self.config)?;

        self.command.run(&config)
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Bench(bench::Bench),
    CheckSolutions(check_solutions::CheckSolutions),
    CopyInputs(copy_inputs::CopyInputs),
    Report(report::Report),
    SolveInputs(solve_inputs::SolveInputs),
    Summary(summary::Summary),
}

impl Commands {
    pub fn run(&self, config: &Config) -> Result<()> {
        match self {
            Self::Bench(cmd) => cmd.run(config),
            Self::CheckSolutions(cmd) => cmd.run(config),
            Self::CopyInputs(cmd) => cmd.run(config),
            Self::Report(cmd) => cmd.run(config),
            Self::SolveInputs(cmd) => cmd.run(config),
            Self::Summary(cmd) => cmd.run(config),
        }
    }
}
