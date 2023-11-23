use anyhow::Result;
use clap::{Parser, Subcommand};

mod ci;
mod criterion_summary;
mod graph;
mod publish_benches;
mod unify_benches;

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
#[command(author, version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn run() -> Result<()> {
        let cli = Self::parse();

        cli.command.run()
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Ci(ci::Ci),
    CriterionSummary(criterion_summary::CriterionSummary),
    Graph(graph::Graph),
    PublishBenches(publish_benches::PublishBenches),
    UnifyBenches(unify_benches::UnifyBenches),
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Ci(cmd) => cmd.run(),
            Self::CriterionSummary(cmd) => cmd.run(),
            Self::Graph(cmd) => cmd.run(),
            Self::PublishBenches(cmd) => cmd.run(),
            Self::UnifyBenches(cmd) => cmd.run(),
        }
    }
}
