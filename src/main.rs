use anyhow::Result;

use cli::Cli;

mod aoc_project;
mod cli;
mod config;
mod util;

fn main() -> Result<()> {
    Cli::run()
}
