use std::{collections::BTreeMap, f64::EPSILON, path::PathBuf};

use anyhow::{bail, Result};
use clap::Args;
use plotly::{
    color::Rgb,
    layout::{Axis, BarMode},
    Bar, ImageFormat, Layout, Plot,
};

use crate::bench_data::{load_benches, BenchCSVRow};

/// Generate graph(s) using a combined benches CSV.
#[derive(Debug, Clone, Args)]
pub struct Graph {
    /// The path to a combined benches CSV
    input: PathBuf,

    /// If set, stores a HTML representation of the graphs.
    #[clap(long)]
    output_html: Option<PathBuf>,

    /// If set, stores a PNG representation of the graphs.
    #[clap(long)]
    output_png: Option<PathBuf>,
}

impl Graph {
    pub fn run(&self) -> Result<()> {
        if !self.input.is_file() {
            bail!("Input file does not exist or is not a file.");
        }

        let benches = {
            let mut out = Vec::default();
            load_benches(&self.input, &mut out)?;
            out
        };

        let accumulated = accumulated_graph(&benches);

        if let Some(ref output_html) = self.output_html {
            println!("> saving html");
            std::fs::write(output_html, accumulated.to_html())?;
        }

        if let Some(ref output_png) = self.output_png {
            println!("> saving PNG");
            accumulated.write_image(output_png, ImageFormat::PNG, 1200, 1000, 1.0);
        }

        Ok(())
    }
}

fn accumulated_graph(benches: &[BenchCSVRow]) -> Plot {
    let mut plot = Plot::new();

    let layout = Layout::new()
        .bar_mode(BarMode::Stack)
        .title("Total runtime by day (lower is better)".into())
        .height(1000)
        .colorway(default_colorway())
        .y_axis(Axis::new().title("Time (ms)".into()));
    plot.set_layout(layout);

    // aggregate into a more useful datastructure
    let mut map: BTreeMap<String, [Vec<f64>; 25]> = BTreeMap::default();
    for bench in benches.iter() {
        let e = map
            .entry(format!("{} ({})", &bench.participant, &bench.language))
            .or_default();
        let day_idx = bench.day - 1;
        e[day_idx].push(bench.mean);
    }

    // btree already sorted
    let participants: Vec<_> = map.keys().cloned().collect();

    // for every day, get a set of data

    for day_index in 0..25 {
        let data: Vec<f64> = participants
            .iter()
            .map(|p| {
                // unwrap should be safe because we made participants from the
                // keys.
                let vals = &map.get(p.as_str()).unwrap()[day_index];

                // we are going to convert from seconds to ms
                1000.0
                    * match vals.len() {
                        0 => 0.0,
                        1 => vals[1],
                        x => vals.iter().sum::<f64>() / x as f64,
                    }
            })
            .collect();

        // skip day if no results for any participant
        if data.iter().all(|d| d - 0.0 <= EPSILON) {
            continue;
        }

        let trace = Bar::new(participants.clone(), data).name(format!("day {}", day_index + 1));
        plot.add_trace(trace);
    }

    plot
}

fn default_colorway() -> Vec<Rgb> {
    vec![
        Rgb::new(114, 229, 239),
        Rgb::new(17, 160, 170),
        Rgb::new(12, 65, 82),
        Rgb::new(168, 184, 230),
        Rgb::new(76, 49, 158),
        Rgb::new(219, 119, 230),
        Rgb::new(106, 116, 170),
        Rgb::new(181, 226, 135),
        Rgb::new(11, 83, 19),
        Rgb::new(66, 241, 143),
        Rgb::new(72, 149, 15),
        Rgb::new(132, 238, 21),
        Rgb::new(101, 42, 13),
        Rgb::new(189, 133, 74),
        Rgb::new(235, 17, 56),
        Rgb::new(216, 95, 45),
        Rgb::new(246, 207, 137),
        Rgb::new(137, 151, 91),
        Rgb::new(78, 72, 9),
        Rgb::new(244, 212, 3),
        Rgb::new(219, 111, 138),
        Rgb::new(161, 19, 178),
        Rgb::new(114, 18, 255),
        Rgb::new(253, 4, 143),
        Rgb::new(155, 27, 92),
    ]
}
