use std::{fs::File, io::Write, path::PathBuf};

use anyhow::{Context, Result};
use clap::Args;
use minijinja::{context, Environment};

use crate::{config::Config, success};

/// Generate the summary for a given year.
///
/// This is intended to be the top-level readme for a given year.
#[derive(Debug, Clone, Args)]
pub struct Summary {
    /// The summary template
    ///
    /// This is a minijinja-compliant template.
    #[arg(short, long)]
    template: PathBuf,

    /// Output to the specified file instead of stdout.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

impl Summary {
    pub fn run(&self, config: &Config) -> Result<()> {
        // load the template engine
        let mut env = Environment::new();
        let report_raw =
            std::fs::read_to_string(&self.template).context("Failed to read template file")?;
        env.add_template("report", &report_raw)
            .context("Failed to add template to engine")?;
        let summary_template = env
            .get_template("report")
            .context("Failed to get template")?;

        let participants: Vec<_> = config.participants().values().collect();

        let rendered = summary_template
            .render(context! {
                year => config.year(),
                pipeline_url => config.pipeline_url(),
                participants,
            })
            .context("Failed to render template")?;

        if let Some(ref output_dest) = self.output {
            println!(
                "  {}",
                success!(format!("Writing output to '{}'", output_dest.display()))
            );

            let mut outfile = File::create(output_dest).context("Failed to create output file")?;
            outfile
                .write_all(rendered.as_bytes())
                .context("Failed to write output file")?;
        } else {
            println!("{}", rendered);
        }

        Ok(())
    }
}
