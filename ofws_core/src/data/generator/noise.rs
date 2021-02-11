use noise::{NoiseFn, Seedable, SuperSimplex};
use std::convert::TryFrom;

/// Hide the noise functions from [`noise`].
pub struct Noise {
    algo: Box<SuperSimplex>,
    scale: f64,
    factor: f64,
}

impl Noise {
    /// Try to create a Noise.
    pub fn new(seed: u32, scale: f64, max_value: u8) -> Result<Noise, &'static str> {
        if scale <= 0.0 {
            return Err("Noise's scale must be positive!");
        }

        Ok(Noise {
            algo: Box::new(SuperSimplex::new().set_seed(seed)),
            scale,
            factor: max_value as f64 / 2.0,
        })
    }

    /// Generates noise for an input.
    pub fn generate1d(&self, input: u32) -> u8 {
        let input = input as f64 / self.scale;
        let positive_value = self.algo.get([input, 0.0]) + 1.0;
        (positive_value * self.factor) as u8
    }

    /// Generates noise for a 2d point (x,y).
    pub fn generate2d(&self, x: u32, y: u32) -> u8 {
        let x = x as f64 / self.scale;
        let y = y as f64 / self.scale;
        let positive_value = self.algo.get([x, y]) + 1.0;
        (positive_value * self.factor) as u8
    }
}

/// For serializing, deserializing & validating [`Noise`].
///
///```
///# use ofws_core::data::generator::noise::{NoiseData, Noise};
/// use std::convert::TryInto;
///
/// let data = NoiseData { seed: 300, scale: 5, max_value: 128 };
/// let noise: Noise = data.clone().try_into().unwrap();
/// let result: NoiseData = noise.into();
/// assert_eq!(data, result)
///```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct NoiseData {
    pub seed: u32,
    pub scale: u32,
    pub max_value: u8,
}

impl TryFrom<NoiseData> for Noise {
    type Error = &'static str;

    fn try_from(data: NoiseData) -> Result<Self, Self::Error> {
        Noise::new(data.seed, data.scale as f64, data.max_value)
    }
}

impl From<Noise> for NoiseData {
    fn from(noise: Noise) -> Self {
        NoiseData {
            seed: noise.algo.seed(),
            scale: noise.scale as u32,
            max_value: (noise.factor * 2.0) as u8,
        }
    }
}
