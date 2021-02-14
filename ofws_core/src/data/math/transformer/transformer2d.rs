use crate::data::math::transformer::clusterer2d::{Clusterer2d, Clusterer2dError};
use crate::data::math::transformer::threshold::OverwriteWithThreshold;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
pub enum Transformer2dError {
    Clusterer(Clusterer2dError),
}

impl From<Clusterer2dError> for Transformer2dError {
    fn from(error: Clusterer2dError) -> Self {
        Transformer2dError::Clusterer(error)
    }
}

/// Transforms 2 inputs into an output.
#[derive(Debug, Serialize, Deserialize)]
pub enum Transformer2d {
    /// Determine a cluster id from both inputs. E.g. biome from rainfall & temperature.
    Clusterer(Clusterer2d),
    /// Returns a const value.
    Const(u8),
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
            Transformer2d::Clusterer(clusterer) => clusterer.cluster(input0, input1),
            Transformer2d::Const(value) => *value,
            Transformer2d::OverwriteIfAboveThreshold(data) => {
                data.overwrite_output_if_above(input0, input1)
            }
            Transformer2d::OverwriteIfBelowThreshold(data) => {
                data.overwrite_output_if_below(input0, input1)
            }
        }
    }
}

/// For serializing, deserializing & validating [`Transformer2d`].
///
///```
///# use ofws_core::data::math::size2d::Size2d;
///# use ofws_core::data::math::transformer::clusterer2d::Clusterer2d;
///# use ofws_core::data::math::transformer::threshold::OverwriteWithThreshold;
///# use ofws_core::data::math::transformer::transformer2d::{Transformer2dData, assert_eq};
/// let clusterer = Clusterer2d::new(Size2d::new(1, 2), vec![10, 11]).unwrap();
/// let overwrite_data = OverwriteWithThreshold::new(100, 200);
///
/// assert_eq(Transformer2dData::Clusterer(clusterer));
/// assert_eq(Transformer2dData::Const(42));
/// assert_eq(Transformer2dData::OverwriteIfAboveThreshold(overwrite_data));
/// assert_eq(Transformer2dData::OverwriteIfBelowThreshold(overwrite_data));
///```
#[derive(new, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Transformer2dData {
    Clusterer(Clusterer2d),
    Const(u8),
    OverwriteIfAboveThreshold(OverwriteWithThreshold<u8>),
    OverwriteIfBelowThreshold(OverwriteWithThreshold<u8>),
}

impl TryFrom<Transformer2dData> for Transformer2d {
    type Error = Transformer2dError;

    fn try_from(data: Transformer2dData) -> Result<Self, Self::Error> {
        match data {
            Transformer2dData::Clusterer(c) => Ok(Transformer2d::Clusterer(c)),
            Transformer2dData::Const(value) => Ok(Transformer2d::Const(value)),
            Transformer2dData::OverwriteIfAboveThreshold(o) => {
                Ok(Transformer2d::OverwriteIfAboveThreshold(o))
            }
            Transformer2dData::OverwriteIfBelowThreshold(o) => {
                Ok(Transformer2d::OverwriteIfBelowThreshold(o))
            }
        }
    }
}

impl From<&Transformer2d> for Transformer2dData {
    fn from(generator: &Transformer2d) -> Self {
        match generator {
            Transformer2d::Clusterer(c) => Transformer2dData::Clusterer(c.clone()),
            Transformer2d::Const(value) => Transformer2dData::Const(*value),
            Transformer2d::OverwriteIfAboveThreshold(o) => {
                Transformer2dData::OverwriteIfAboveThreshold(*o)
            }
            Transformer2d::OverwriteIfBelowThreshold(o) => {
                Transformer2dData::OverwriteIfBelowThreshold(*o)
            }
        }
    }
}

pub fn assert_eq(data: Transformer2dData) {
    let generator: Transformer2d = data.clone().try_into().unwrap();
    let result: Transformer2dData = (&generator).into();
    assert_eq!(result, data)
}
