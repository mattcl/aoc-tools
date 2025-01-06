use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use url::Url;

mod check_config;
mod ci;
mod criterion_summary;
mod graph;
mod publish_benches;
mod python_summary;
mod unify_benches;
mod update_participants;

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
    CheckConfig(check_config::CheckConfig),
    Ci(ci::Ci),
    CriterionSummary(criterion_summary::CriterionSummary),
    Graph(graph::Graph),
    PublishBenches(publish_benches::PublishBenches),
    PythonSummary(python_summary::PythonSummary),
    UnifyBenches(unify_benches::UnifyBenches),
    UpdateParticipants(update_participants::UpdateParticipants),
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::CheckConfig(cmd) => cmd.run(),
            Self::Ci(cmd) => cmd.run(),
            Self::CriterionSummary(cmd) => cmd.run(),
            Self::Graph(cmd) => cmd.run(),
            Self::PublishBenches(cmd) => cmd.run(),
            Self::PythonSummary(cmd) => cmd.run(),
            Self::UnifyBenches(cmd) => cmd.run(),
            Self::UpdateParticipants(cmd) => cmd.run(),
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct ApiCommon {
    /// The base api url for an aoc-web service.
    #[clap(short, long, required = true, env = "AOC_TOOLS_API_BASE")]
    pub api_base: Url,

    /// The token for authenticating with the api.
    #[clap(
        short,
        long,
        required = true,
        env = "AOC_TOOLS_API_TOKEN",
        hide_env_values = true
    )]
    pub api_token: String,
}
