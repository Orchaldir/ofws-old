use crate::data::math::transformer::threshold::OverwriteWithThreshold;
use serde::{Deserialize, Serialize};

/// Transforms 2 inputs into an output.
#[derive(Debug, Serialize, Deserialize)]
pub enum Transformer2d {
    /// Overwrites the input, if it is above a threshold.
    OverwriteIfAboveThreshold(OverwriteWithThreshold<u8>),
    /// Overwrites the input, if it is below a threshold.
    OverwriteIfBelowThreshold(OverwriteWithThreshold<u8>),
}

impl Transformer2d {
    pub fn new_overwrite_if_above(value: u8, threshold: u8) -> Transformer2d {
        Transformer2d::OverwriteIfAboveThreshold(OverwriteWithThreshold::new(value, threshold))
    }

    pub fn new_overwrite_if_below(value: u8, threshold: u8) -> Transformer2d {
        Transformer2d::OverwriteIfBelowThreshold(OverwriteWithThreshold::new(value, threshold))
    }

    /// Transforms 2 inputs into an output.
    pub fn transform(&self, input0: u8, input1: u8) -> u8 {
        match self {
            Transformer2d::OverwriteIfAboveThreshold(data) => {
                data.overwrite_output_if_above(input0, input1)
            }
            Transformer2d::OverwriteIfBelowThreshold(data) => {
                data.overwrite_output_if_below(input0, input1)
            }
        }
    }
}
