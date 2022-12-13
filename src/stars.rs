use crate::color::Color;
use bracket_noise::prelude::*;
use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;

mod spectral_type;
use spectral_type::SpectralType;

/// Generate a sky filled with stars
pub fn generate_stars(size: usize, seed: u64) -> Vec<Color> {
    let mut sky = vec![Color::new(0.02, 0.02, 0.095, 1.0); size.pow(2)];
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
    let stellar_distribution = SpectralType::get_distribution();

    for _ in 0..5 {
        let mut noise = FastNoise::seeded(rng.gen());
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(5);
        noise.set_fractal_gain(0.6);
        noise.set_fractal_lacunarity(2.0);
        noise.set_frequency(2.0);

        let scale = size as f32;

        for _ in 0..750 {
            let x = rng.gen_range(0..size);
            let y = rng.gen_range(0..size);

            let alpha = noise.get_noise(x as f32 / scale, y as f32 / scale) * 1.5;
            if alpha <= f32::EPSILON {
                continue; // ignore "invisible" stars
            }

            let star = SpectralType::from_dist(stellar_distribution.sample(&mut rng));
            let star_color = star.color();
            let radius = rng.gen_range(star.radius());
            let iradius = radius as usize;

            let xmin = if x <= iradius { 0 } else { x - iradius };
            let xmax = std::cmp::min(size - 1, x + iradius);
            let ymin = if y <= iradius { 0 } else { y - iradius };
            let ymax = std::cmp::min(size - 1, y + iradius);

            for px in xmin..=xmax {
                for py in ymin..=ymax {
                    if px != x && py != y {
                        continue;
                    }
                    let dist = distance(x, y, px, py);
                    if dist >= radius {
                        continue;
                    }

                    let radius_alpha = 1.0 - dist / radius;

                    let idx = py * size + px;
                    let p_alpha = alpha.clamp(0.0, 1.0) * 0.5 * radius_alpha;
                    sky[idx] = sky[idx].blend(star_color.mul_alpha(p_alpha));
                }
            }
        }
    }

    sky
}

/// Get the distance between two points
fn distance(x1: usize, y1: usize, x2: usize, y2: usize) -> f32 {
    ((x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2)) as f32).sqrt()
}
