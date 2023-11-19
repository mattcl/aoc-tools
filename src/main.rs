use anyhow::Result;

use cli::Cli;

mod aoc_project;
mod bench_data;
mod cli;
mod config;
mod solution;
mod util;

fn main() -> Result<()> {
    Cli::run()
}
