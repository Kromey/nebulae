use crate::color::Color;
use bracket_noise::prelude::*;
use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use fast_poisson::Poisson2D;

const DEFAULT_SEED: u64 = 0xCAFEBABE;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Stars {
    seed: u64,
    size: usize,
    cells: usize,
}

impl Stars {
    pub fn new(size: usize, cells: usize) -> Self {
        Self::seeded(size, cells, DEFAULT_SEED)
    }

    pub fn seeded(size: usize, cells: usize, seed: u64) -> Self {
        Self {
            seed,
            size,
            cells,
        }
    }

    pub fn generate(self) -> Vec<Color> {
        let mut sky = vec![Color::new(0.02, 0.02, 0.095, 1.0); self.size.pow(2)];
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(self.seed);

        for _ in 0..5 {
            let mut noise = FastNoise::seeded(rng.gen());
            noise.set_noise_type(NoiseType::SimplexFractal);
            noise.set_fractal_type(FractalType::FBM);
            noise.set_fractal_octaves(5);
            noise.set_fractal_gain(0.6);
            noise.set_fractal_lacunarity(2.0);
            noise.set_frequency(2.0);

            let scale = self.size as f32;

            for point in Poisson2D::new()
                .with_seed(rng.gen())
                .with_dimensions([(self.size - 1) as f32; 2], 20.0)
                .with_samples(5)
                .iter()
            {
                let alpha = noise.get_noise(point[0] / scale, point[1] / scale);
                if alpha <= f32::EPSILON {
                    continue; // ignore "invisible" stars
                }

                let x = point[0].round() as usize;
                let y = point[1].round() as usize;
                let idx = y * self.size + x;

                sky[idx] = Color::new(
                    1.0,
                    1.0,
                    1.0,
                    alpha.clamp(0.0, 1.0) * 0.5,
                );
            }
        }

        sky
    }
}
