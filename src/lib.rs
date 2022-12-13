mod color;
mod nebula;
mod stars;

use std::time::Instant;

use color::Color;
use image::ImageBuffer;

const IMG_DIMENSIONS: usize = 1024;

/// Generate a nebula in front of a star-filled sky
pub fn generate(seed: u64) -> ImageBuffer<image::Rgba<u16>, Vec<u16>> {
    let dimensions = IMG_DIMENSIONS as u32;

    let star_start = Instant::now();
    let stars = stars::generate_stars(IMG_DIMENSIONS, seed);
    println!(
        "Generated stars in {:.4} seconds",
        star_start.elapsed().as_secs_f32()
    );

    let nebula_start = Instant::now();
    let nebula = nebula::generate_nebula(IMG_DIMENSIONS, seed, &stars);
    println!(
        "Generated nebula in {:.4} seconds",
        nebula_start.elapsed().as_secs_f32()
    );

    ImageBuffer::from_vec(dimensions, dimensions, nebula).unwrap()
}
