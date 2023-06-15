#![deny(clippy::perf, clippy::pedantic)]
use std::path::PathBuf;

use clap::Parser;
use image::io::Reader as ImageReader;
use log::info;

#[derive(Debug, Parser)]
struct Opts {
    /// The input file
    input: PathBuf,
    /// The output file
    #[arg(short, long, default_value = "./out.png")]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    info!("Got args: {opts:?}");

    let image = ImageReader::open(opts.input)?.decode()?;
    image.save(opts.output)?;

    Ok(())
}
