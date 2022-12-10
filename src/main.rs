use std::time::Instant;



use sky::{Nebula, Stars};

const IMG_DIMENSIONS: usize = 1024;

fn main() {

    let dimensions = IMG_DIMENSIONS as u32;

    let star_start = Instant::now();
    let stars = Stars::new(IMG_DIMENSIONS, 200).generate();
    println!("Generated stars in {:.4} seconds", star_start.elapsed().as_secs_f32());

    let nebula_start = Instant::now();
    let nebula = Nebula::new(IMG_DIMENSIONS).generate(&stars);
    println!("Generated nebula in {:.4} seconds", nebula_start.elapsed().as_secs_f32());

    let img: image::ImageBuffer<image::Rgba<u16>, Vec<u16>> =
        image::ImageBuffer::from_vec(dimensions, dimensions, nebula).unwrap();

    img.save("nebula.png").unwrap();
}
