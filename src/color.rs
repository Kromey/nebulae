use std::ops::Mul;



#[inline(always)]
fn scale_u16(f: f32) -> u16 {
    const SCALE_TO: f32 = u16::MAX as f32;

    (f.clamp(0.0, 1.0) * SCALE_TO).round() as u16
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a}
    }

    #[inline(always)]
    pub fn splat(value: f32) -> Self {
        Self::new(value, value, value, value)
    }

    #[inline(always)]
    pub fn mul_alpha(mut self, a: f32) -> Self {
        self.a *= a;
        self
    }

    pub fn to_array(self) -> [u16; 4] {
        [
            scale_u16(self.r),
            scale_u16(self.g),
            scale_u16(self.b),
            scale_u16(self.a),
        ]
    }
    
    pub fn blend(self, fg: Color) -> Self {

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

    #[inline(always)]
    fn premultiply(&self) -> (f32, f32, f32) {
        (
            self.r * self.a,
            self.g * self.a,
            self.b * self.a,
        )
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a,
        }    
    }
}
