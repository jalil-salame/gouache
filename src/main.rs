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
    /// The output file
    #[arg(short, long, default_value = "./out.png")]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    info!("Got args: {opts:?}");

    // Read image as pixels between 0.0 and 1.0
    let mut image = ImageReader::open(opts.input)?.decode()?.into_rgb32f();

    apply_effect(&mut naive_grayscale, &mut image);

    DynamicImage::ImageRgb32F(image)
        .into_rgb8()
        .save(opts.output)?;

    Ok(())
}

fn apply_effect<E>(effect: &mut E, image: &mut image::Rgb32FImage)
where
    E: ImageEffect,
    E::Error: std::error::Error,
{
    effect.process(image).expect("failed to apply effect");
}

trait ImageEffect {
    type Error;

    fn process(&mut self, image: &mut image::Rgb32FImage) -> Result<(), Self::Error>;
}

impl<F: FnMut(&mut image::Rgb32FImage)> ImageEffect for F {
    type Error = Infallible;

    fn process(&mut self, image: &mut image::Rgb32FImage) -> Result<(), Self::Error> {
        self(image);
        Ok(())
    }
}

fn naive_grayscale(image: &mut image::Rgb32FImage) {
    for pixels in image.pixels_mut() {
        let mean = pixels.0.iter().sum::<f32>() / 3.0;

        *pixels = image::Rgb([mean, mean, mean]);
    }
}
