use std::ops::Range;
use rand::distributions::WeightedIndex;
use crate::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpectralType {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
}

impl SpectralType {
    pub fn weight(&self) -> f32 {
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

    pub fn color(&self) -> Color {
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

    pub fn radius(&self) -> Range<f32> {
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

    pub fn get_distribution() -> WeightedIndex<f32> {
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

    pub fn from_dist(dist: usize) -> Self {
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
