#![deny(clippy::perf, clippy::pedantic)]
use std::convert::Infallible;

pub fn apply_effect<E>(effect: &mut E, image: &mut image::Rgb32FImage)
where
    E: ImageEffect,
    E::Error: std::error::Error,
{
    effect.process(image).expect("failed to apply effect");
}

pub trait ImageEffect {
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

pub fn luminence_grayscale(image: &mut image::Rgb32FImage) {
    const KR: f32 = 0.299;
    const KG: f32 = 0.587;
    const KB: f32 = 0.114;

    for pixels in image.pixels_mut() {
        let [r, g, b] = pixels.0;
        let mean = KR * r + KG * g + KB * b;

        *pixels = image::Rgb([mean, mean, mean]);
    }
}

pub fn naive_grayscale(image: &mut image::Rgb32FImage) {
    for pixels in image.pixels_mut() {
        let mean = pixels.0.iter().sum::<f32>() / 3.0;

        *pixels = image::Rgb([mean, mean, mean]);
    }
}
