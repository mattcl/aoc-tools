use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Args;
use console::style;

use crate::{config::Config, util::day_directory_name};

/// Copies the inputs for the configured participants to the specified location.
///
/// Inputs will be suffixed with the particpating project's username and
/// organized into folders named `<day>_<padded number>` (e.g. day_002).
///
/// If inputs do not exist or cannot be gatherhed from a particular participant,
/// those inputs are ignored.
#[derive(Debug, Clone, Args)]
pub struct CopyInputs {
    /// The root directory in which to put inputs.
    ///
    /// Inputs will be organized according to day under this directory.
    destination: PathBuf,
}

impl CopyInputs {
    pub fn run(&self, config: &Config) -> Result<()> {
        for day in 1..=25 {
            let day_directory_name = day_directory_name(day);
            let day_directory = self.destination.join(&day_directory_name);

            // fetch all of the inputs
            println!(
                "> Copying inputs for day {} to {}",
                day, &day_directory_name
            );

            if !day_directory.is_dir() {
                println!("  Destination directory does not exist. Creating it.");
                std::fs::create_dir(&day_directory).context("Failed to make directory")?;
            }

            for (_, project) in config.participants().iter() {
                match project.input_path(day) {
                    Ok(Some(path)) => {
                        let output_name = format!("input-{}", project.username());
                        let dest = day_directory.join(output_name);
                        self.copy_input(&path, &dest)?;
                        println!(
                            "{}",
                            style(format!("  Copied input from {}", project.username())).green()
                        )
                    }
                    Ok(None) => {
                        println!(
                            "{}",
                            style(format!("  No input for {}", project.username())).yellow()
                        );
                    }
                    // if we fail to run the input command, we don't want to
                    // error out completely.
                    Err(_) => {
                        println!(
                            "{}",
                            style(format!(
                                "  Input command did not succeed for {}",
                                project.username()
                            ))
                            .yellow()
                        );
                    }
                }
            }
        }

        Ok(())
    }

    fn copy_input(&self, from: &Path, to: &Path) -> Result<()> {
        if !from.is_file() {
            println!(
                "{}",
                style(format!(
                    "  '{}' does not exist or is not a file",
                    from.to_string_lossy()
                ))
                .red()
            );
            return Ok(());
        }

        std::fs::copy(from, to)?;

        Ok(())
    }
}
