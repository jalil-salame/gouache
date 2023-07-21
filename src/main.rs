#![deny(clippy::perf, clippy::pedantic)]
use std::convert::Infallible;
use std::path::PathBuf;

use clap::Parser;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use log::info;

#[derive(Debug, Parser)]
struct Opts {
    /// The input file
    input: PathBuf,
    /// The effect to apply to the image
    #[arg(value_enum, default_value_t)]
    effect: Effect,
    /// The output file
    #[arg(short, long, default_value = "./out.png")]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    info!("Got args: {opts:?}");

    let mut effect = opts.effect;

    // Read image as pixels between 0.0 and 1.0
    let mut image = ImageReader::open(opts.input)?.decode()?.into_rgb32f();

    palette::apply_effect(&mut effect, &mut image);

    DynamicImage::ImageRgb32F(image)
        .into_rgb8()
        .save(opts.output)?;

    Ok(())
}

#[derive(Debug, Default, clap::ValueEnum, Clone, Copy)]
enum Effect {
    /// Don't do anything to the image
    #[default]
    Identity,
    /// Naively calculate the brightness of the pixels
    NaiveGrayscale,
    /// Use luminence to calculate the brigtness of the pixels
    LuminenceGrayscale,
}

impl palette::ImageEffect for Effect {
    type Error = Infallible;

    fn process(&mut self, image: &mut image::Rgb32FImage) -> Result<(), Self::Error> {
        match self {
            Effect::Identity => Ok(()),
            Effect::NaiveGrayscale => Ok(palette::naive_grayscale(image)),
            Effect::LuminenceGrayscale => Ok(palette::luminence_grayscale(image)),
        }
    }
}
