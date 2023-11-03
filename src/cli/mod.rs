mod ci;

use anyhow::Result;
use clap::{Parser, Subcommand};

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
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Ci(cmd) => cmd.run(),
        }
    }
}
