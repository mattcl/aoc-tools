use anyhow::Result;

use cli::Cli;

mod aoc_project;
mod beanch_data;
mod cli;
mod config;
mod solution;
mod util;

fn main() -> Result<()> {
    Cli::run()
}
