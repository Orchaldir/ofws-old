use noise::{NoiseFn, Seedable, SuperSimplex};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// Hide the noise functions from [`noise`].
pub struct Noise {
    algo: Box<SuperSimplex>,
    scale: f64,
    base: f64,
    factor: f64,
}

impl Noise {
    /// Try to create a Noise. Fails if scale is negative:
    ///
    ///```
    ///# use ofws_core::data::math::generator::noise::Noise;
    /// assert!(Noise::new(0, -1.0, 0, 255).is_err())
    ///```
    /// Also fails if min_value >= max_value:
    ///
    ///```
    ///# use ofws_core::data::math::generator::noise::Noise;
    /// assert!(Noise::new(0, 5.0, 200, 105).is_err())
    ///```
    ///
    pub fn new(seed: u32, scale: f64, min_value: u8, max_value: u8) -> Result<Noise, &'static str> {
        if scale <= 0.0 {
            return Err("Noise's scale must be positive!");
        } else if min_value >= max_value {
            return Err("Noise's min_value must be smaller than max_value!");
        }

        Ok(Noise {
            algo: Box::new(SuperSimplex::new().set_seed(seed)),
            scale,
            base: 1.0 + min_value as f64 / 255.0,
            factor: (max_value - min_value) as f64 / 2.0,
        })
    }

    /// Generates noise for an input.
    pub fn generate1d(&self, input: u32) -> u8 {
        let input = input as f64 / self.scale;
        let positive_value = self.algo.get([input, 0.0]) + self.base;
        (positive_value * self.factor) as u8
    }

    /// Generates noise for a 2d point (x,y).
    pub fn generate2d(&self, x: u32, y: u32) -> u8 {
        let x = x as f64 / self.scale;
        let y = y as f64 / self.scale;
        let positive_value = self.algo.get([x, y]) + self.base;
        (positive_value * self.factor) as u8
    }
}

/// For serializing, deserializing & validating [`Noise`].
///
///```
///# use ofws_core::data::math::generator::noise::{NoiseData, Noise};
///# use std::convert::TryInto;
///
/// let data = NoiseData { seed: 300, scale: 5, min_value: 10, max_value: 128 };
/// let noise: Noise = data.clone().try_into().unwrap();
/// let result: NoiseData = (&noise).into();
/// assert_eq!(data, result)
///```
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct NoiseData {
    pub seed: u32,
    pub scale: u32,
    pub min_value: u8,
    pub max_value: u8,
}

impl TryFrom<NoiseData> for Noise {
    type Error = &'static str;

    fn try_from(data: NoiseData) -> Result<Self, Self::Error> {
        Noise::new(data.seed, data.scale as f64, data.min_value, data.max_value)
    }
}

impl From<&Noise> for NoiseData {
    fn from(noise: &Noise) -> Self {
        let min_value = ((noise.base - 1.0) * 255.0) as u8;
        NoiseData {
            seed: noise.algo.seed(),
            scale: noise.scale as u32,
            min_value,
            max_value: (noise.factor * 2.0) as u8 + min_value,
        }
    }
}
