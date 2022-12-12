use std::ops::Range;

use crate::color::Color;
use bracket_noise::prelude::*;
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use rand_xoshiro::Xoshiro256PlusPlus;

const DEFAULT_SEED: u64 = 0xCAFEBABE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpectralType {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
}

impl SpectralType {
    fn weight(&self) -> f32 {
        use SpectralType::*;

        match *self {
            O => 0.0000003,
            B => 0.0013,
            A => 0.006,
            F => 0.03,
            G => 0.076,
            K => 0.121,
            M => 0.765,
        }
    }

    fn color(&self) -> Color {
        use SpectralType::*;

        match *self {
            O => Color::new_from_u8(155, 176, 255, 255),
            B => Color::new_from_u8(170, 191, 255, 255),
            A => Color::new_from_u8(202, 215, 255, 255),
            F => Color::new_from_u8(248, 247, 255, 255),
            G => Color::new_from_u8(255, 244, 234, 255),
            K => Color::new_from_u8(255, 210, 161, 255),
            M => Color::new_from_u8(255, 204, 111, 255),
        }
    }

    fn radius(&self) -> Range<f32> {
        use SpectralType::*;

        match *self {
            O => 6.6..7.5,
            B => 1.8..6.6,
            A => 1.4..1.8,
            F => 1.1..1.4,
            G => 0.9..1.1,
            K => 0.7..0.9,
            M => 0.6..0.7,
        }
    }

    fn get_distribution() -> WeightedIndex<f32> {
        let weights = [
            SpectralType::O.weight(),
            SpectralType::B.weight(),
            SpectralType::A.weight(),
            SpectralType::F.weight(),
            SpectralType::G.weight(),
            SpectralType::K.weight(),
            SpectralType::M.weight(),
        ];

        WeightedIndex::new(&weights).unwrap()
    }

    fn from_dist(dist: usize) -> Self {
        use SpectralType::*;

        match dist {
            0 => O,
            1 => B,
            2 => A,
            3 => F,
            4 => G,
            5 => K,
            6 => M,
            _ => panic!("Invalid index: {dist}"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Stars {
    seed: u64,
    size: usize,
}

impl Stars {
    pub fn new(size: usize) -> Self {
        Self::seeded(size, DEFAULT_SEED)
    }

    pub fn seeded(size: usize, seed: u64) -> Self {
        Self {
            seed,
            size,
        }
    }

    pub fn generate(self) -> Vec<Color> {
        let mut sky = vec![Color::new(0.02, 0.02, 0.095, 1.0); self.size.pow(2)];
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(self.seed);
        let stellar_distribution = SpectralType::get_distribution();

        for _ in 0..5 {
            let mut noise = FastNoise::seeded(rng.gen());
            noise.set_noise_type(NoiseType::SimplexFractal);
            noise.set_fractal_type(FractalType::FBM);
            noise.set_fractal_octaves(5);
            noise.set_fractal_gain(0.6);
            noise.set_fractal_lacunarity(2.0);
            noise.set_frequency(2.0);

            let scale = self.size as f32;

            for _ in 0..750
            {
                let x = rng.gen_range(0..self.size);
                let y = rng.gen_range(0..self.size);

                let alpha = noise.get_noise(x as f32 / scale, y as f32 / scale) * 1.5;
                if alpha <= f32::EPSILON {
                    continue; // ignore "invisible" stars
                }

                let star = SpectralType::from_dist(stellar_distribution.sample(&mut rng));
                let star_color = star.color();
                let radius = rng.gen_range(star.radius());
                let iradius = radius as usize;

                let xmin = if x <= iradius { 0 } else { x - iradius };
                let xmax = std::cmp::min(self.size - 1, x + iradius);
                let ymin = if y <= iradius { 0 } else { y - iradius };
                let ymax = std::cmp::min(self.size - 1, y + iradius);

                for px in xmin..=xmax {
                    for py in ymin..=ymax {
                        if px != x && py != y {
                            continue;
                        }
                        let dist = Self::distance(x, y, px, py);
                        if dist >= radius {
                            continue;
                        }

                        let radius_alpha = 1.0 - dist / radius;

                        let idx = py * self.size + px;
                        let p_alpha = alpha.clamp(0.0, 1.0) * 0.5 * radius_alpha;
                        sky[idx] = sky[idx].blend(star_color.mul_alpha(p_alpha));
                    }
                }
            }
        }

        sky
    }

    fn distance(x1: usize, y1: usize, x2: usize, y2: usize) -> f32 {
        ((x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2)) as f32).sqrt()
    }
}
