use rayon::prelude::*;
use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;

const DEFAULT_SEED: u64 = 0xCAFEBABE;

mod cloud;
use cloud::GasCloud;
use super::Color;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Nebula {
    size: usize,
    scale: f32,
    seed: u64,
}

impl Nebula {
    pub fn new(size: usize) -> Self {
        Self::seeded(size, DEFAULT_SEED)
    }

    pub fn seeded(size: usize, seed: u64) -> Self {
        Self {
            size,
            scale: size as f32,
            seed,
        }
    }

    pub fn generate(self) -> Vec<u16> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(self.seed);

        let clouds = vec![
            GasCloud::new(Color::new(0.5, 1.0, 0.3, 1.0), rng.gen()),
            GasCloud::new(Color::new(0.2, 1.0, 1.0, 0.8), rng.gen()),
        ];

        let bg_color = Color::new(0.02, 0.02, 0.095, 1.0);

        (0..(self.size.pow(2)))
            .into_par_iter()
            .map(|i| {
                let (x, y) = self.get_xy(i);
                // print!("\n{x},{y}: ");
                clouds.iter()
                    .map(|gas| gas.pixel(x, y))
                    .fold(bg_color, |bg, fg| bg.blend(fg))
                    .to_array()
            })
            .flatten()
            .collect()
    }

    #[inline(always)]
    fn get_xy(&self, i: usize) -> (f32, f32) {
        let x = i % self.size;
        let y = i / self.size;

        (x as f32 / self.scale, y as f32 / self.scale)
    }
}
