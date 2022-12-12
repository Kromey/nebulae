mod color;
mod nebula;
mod stars;

use std::time::Instant;

use color::Color;
use image::ImageBuffer;
pub use nebula::Nebula;
pub use stars::Stars;

const IMG_DIMENSIONS: usize = 1024;

pub fn generate(seed: u64) -> ImageBuffer<image::Rgba<u16>, Vec<u16>> {
    let dimensions = IMG_DIMENSIONS as u32;

    let star_start = Instant::now();
    let stars = Stars::new(IMG_DIMENSIONS, seed).generate();
    println!("Generated stars in {:.4} seconds", star_start.elapsed().as_secs_f32());

    let nebula_start = Instant::now();
    let nebula = Nebula::new(IMG_DIMENSIONS, seed).generate(&stars);
    println!("Generated nebula in {:.4} seconds", nebula_start.elapsed().as_secs_f32());

    ImageBuffer::from_vec(dimensions, dimensions, nebula).unwrap()
}
