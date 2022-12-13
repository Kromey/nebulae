use std::ops::Mul;

/// A helper function to scale a float in the 0-1 range to an integer in the 0-`u16::MAX` range
#[inline(always)]
fn scale_u16(f: f32) -> u16 {
    const SCALE_TO: f32 = u16::MAX as f32;

    (f.clamp(0.0, 1.0) * SCALE_TO).round() as u16
}

/// A simple RGBA color
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    /// Create a new `Color`
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create a `Color` using 8-bit integers
    pub fn new_from_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        const SCALE: f32 = u8::MAX as f32;

        Self::new(
            r as f32 / SCALE,
            g as f32 / SCALE,
            b as f32 / SCALE,
            a as f32 / SCALE,
        )
    }

    /// Create a `Color` where all 4 channels are set to the same value
    #[inline(always)]
    pub fn splat(value: f32) -> Self {
        Self::new(value, value, value, value)
    }

    /// Multiply the `Color`'s alpha channel by some value
    #[inline(always)]
    pub fn mul_alpha(mut self, a: f32) -> Self {
        self.a *= a;
        self
    }

    /// Convert this `Color` into an array of `u16` values
    pub fn to_array(self) -> [u16; 4] {
        [
            scale_u16(self.r),
            scale_u16(self.g),
            scale_u16(self.b),
            scale_u16(self.a),
        ]
    }

    /// Blend this `Color` with another, using [alpha compositing]
    ///
    /// [alpha compositing]: https://stackoverflow.com/questions/7438263/alpha-compositing-algorithm-blend-modes#answer-11163848
    pub fn blend(self, fg: Color) -> Self {
        if self.a < f32::EPSILON {
            return fg;
        } else if fg.a < f32::EPSILON {
            return self;
        }

        let alpha = self.a + fg.a - self.a * fg.a;

        let (bg_r, bg_g, bg_b) = self.premultiply();
        let (fg_r, fg_g, fg_b) = fg.premultiply();

        let final_r = fg_r + bg_r * (1.0 - fg.a);
        let final_g = fg_g + bg_g * (1.0 - fg.a);
        let final_b = fg_b + bg_b * (1.0 - fg.a);

        Color {
            r: final_r / alpha,
            g: final_g / alpha,
            b: final_b / alpha,
            a: alpha,
        }
    }

    /// Compute the pre-multiplied R, G, and B channel values
    #[inline(always)]
    fn premultiply(&self) -> (f32, f32, f32) {
        (self.r * self.a, self.g * self.a, self.b * self.a)
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    /// As a convenience for some of the operations we do, define multiplying a `Color` by a `f32`
    /// as multiplying each of the R, G, and B values by the `f32`; the alpha channel is left alone.
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a,
        }
    }
}
