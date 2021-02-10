use crate::data::math::interpolation::lerp;
use noise::{NoiseFn, Seedable, SuperSimplex};

/// Generates values for a 1d input.
pub enum Generator1d {
    /// Generates a linear gradient between a start and an end value.
    Gradient1d {
        value_start: u8,
        value_end: u8,
        length: u32,
    },
    /// Returns the input as output.
    InputAsOutput,
    /// Generates values with Super Simplex noise.
    Noise1d {
        algo: Box<SuperSimplex>,
        scale: f64,
        factor: f64,
    },
}

impl Generator1d {
    pub fn new_gradient(value_start: u8, value_end: u8, length: u32) -> Generator1d {
        Generator1d::Gradient1d {
            value_start,
            value_end,
            length,
        }
    }

    pub fn new_noise(seed: u32, scale: f64, max_value: u8) -> Generator1d {
        Generator1d::Noise1d {
            algo: Box::new(SuperSimplex::new().set_seed(seed)),
            scale,
            factor: max_value as f64 / 2.0,
        }
    }

    /// Generates an output for an input.
    pub fn generate(&self, input: u32) -> u8 {
        match self {
            Generator1d::Gradient1d {
                value_start,
                value_end,
                length,
            } => {
                let factor = input as f32 / *length as f32;
                lerp(*value_start, *value_end, factor)
            }
            Generator1d::InputAsOutput => input as u8,
            Generator1d::Noise1d {
                algo,
                scale,
                factor,
            } => {
                let input = input as f64 / scale;
                let positive_value = algo.get([input, 0.0]) + 1.0;
                (positive_value * factor) as u8
            }
        }
    }
}
