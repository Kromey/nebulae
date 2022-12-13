use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::prelude::*;

mod cloud;
use super::Color;
use cloud::GasCloud;

/// Generate a gaseous nebula in front of the supplied background
///
/// # Panics
/// 
/// This function will panic if the background is smaller than the specified size.
pub fn generate_nebula(size: usize, seed: u64, background: &[Color]) -> Vec<u16> {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
    let scale = size as f32;

    let clouds = vec![
        GasCloud::new(Color::new(0.5, 1.0, 0.3, 1.0), rng.gen()),
        GasCloud::new(Color::new(0.2, 1.0, 1.0, 0.8), rng.gen()),
    ];

    background
        .into_par_iter()
        .enumerate()
        .flat_map(|(i, bg_color)| {
            let (x, y) = {
                let x = i % size;
                let y = i / size;
        
                (x as f32 / scale, y as f32 / scale)
            };

            clouds
                .iter()
                .map(|gas| gas.pixel(x, y))
                .fold(*bg_color, |bg, fg| bg.blend(fg))
                .to_array()
        })
        .collect()
}
