use super::Color;
use bracket_noise::prelude::*;

/// A struct representing a cloud of gas in a nebula
pub struct GasCloud {
    color: Color,
    noise: FastNoise,
    billow: FastNoise,
}

impl GasCloud {
    /// Create a new cloud with the given color and specified seed
    pub fn new(color: Color, seed: u64) -> Self {
        let noise = Self::new_noise(seed, FractalType::FBM);
        let billow = Self::new_noise(seed, FractalType::Billow);

        Self {
            color,
            noise,
            billow,
        }
    }

    /// Get a noise object using the specified fractal kind
    fn new_noise(seed: u64, kind: FractalType) -> FastNoise {
        // I have no idea what these parameters do!
        // They're stolen directly from https://github.com/amethyst/bracket-lib/blob/master/bracket-noise/examples/simplex_fractal.rs
        // They do seem to give me results I like, though!
        let mut noise = FastNoise::seeded(seed);
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(kind);
        noise.set_fractal_octaves(5);
        noise.set_fractal_gain(0.6);
        noise.set_fractal_lacunarity(2.0);
        noise.set_frequency(2.0);

        noise
    }

    /// Get the [`Color`] at the specified (x, y) pixel
    pub fn pixel(&self, x: f32, y: f32) -> Color {
        let radius = 0.5 - 2.0 * ((x - 0.5).powi(2) + (y - 0.5).powi(2));
        if radius < f32::EPSILON {
            return Color::splat(0.0);
        }

        let distort = 0.2;
        let offset_x = 1.7;
        let offset_y = -3.2;

        let x_distort = self.get_noise(x + offset_x, y) * distort;
        let y_distort = self.get_noise(x, y - offset_y) * distort;

        let billow = self.get_billow(x, y);
        let w = self.get_noise(x + x_distort, y + y_distort) + billow;

        let a = self.get_noise(x - x_distort, y - y_distort) * radius.min(0.4) + billow * radius;

        (self.color * w).mul_alpha(a)
    }

    /// Get and normalize noise at the specified point
    #[inline(always)]
    fn get_noise(&self, x: f32, y: f32) -> f32 {
        (self.noise.get_noise(x, y) + 0.5).clamp(0.0, 1.0)
    }

    /// Get and manipulate the "billow" noise at the specified point
    ///
    /// This is used to create both "glowy" spots (positive values) and dark spots (negative values)
    #[inline(always)]
    fn get_billow(&self, x: f32, y: f32) -> f32 {
        self.billow.get_noise(x, y).max(0.0) * 2.0
            - self.billow.get_noise(x + 1.7, y).max(0.0) * 3.0
    }
}
