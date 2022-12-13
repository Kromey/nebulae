use crate::color::Color;
use rand::distributions::WeightedIndex;
use std::ops::Range;

/// The basic spectral classifications of stars
///
/// We do not concern ourselves with sub-types nor "extended" spectral types, as
/// true-to-life realism is not the goal
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
    /// Get the relative weight (i.e. percent) for each type of star.
    ///
    /// Numbers are taken from <https://commons.wikimedia.org/wiki/File:Stellar_Classification_Chart.png>
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

    /// Get the blackbody color for each type of star.
    ///
    /// In reality, each spectral type covers a range of color, but one color
    /// for each is enough given that they are not the focus of the image anyway.
    ///
    /// Colors come from <http://www.vendian.org/mncharity/dir3/starcolor/>
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

    /// Get the size (in solar radii) of each type of star
    ///
    /// This returns a range from which a random radius can be picked.
    ///
    /// Data comes from <https://commons.wikimedia.org/wiki/File:Stellar_Classification_Chart.png>,
    /// except that the minimum and maximum values of the smallest and largest stars (respectively)
    /// were chosen arbitrarily by myself.
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

    /// Get a [`WeightedIndex`] suitable for choosing a random star type
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

    /// Chose a star type given an index
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
