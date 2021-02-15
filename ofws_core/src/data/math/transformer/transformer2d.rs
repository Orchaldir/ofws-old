use crate::data::math::transformer::clusterer2d::{Clusterer2d, Clusterer2dData, Clusterer2dError};
use crate::data::math::transformer::threshold::OverwriteWithThreshold;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use Transformer2d::*;

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
#[derive(Debug)]
pub enum Transformer2d {
    /// Determine a cluster id from both inputs. E.g. biome from rainfall & temperature.
    Clusterer(Clusterer2d),
    /// Returns a const value.
    Const(u8),
    /// Overwrites the input, if it is above a threshold.
    OverwriteIfAbove(OverwriteWithThreshold<u8>),
    /// Overwrites the input, if it is below a threshold.
    OverwriteIfBelow(OverwriteWithThreshold<u8>),
}

impl Transformer2d {
    pub fn new_overwrite_if_above(value: u8, threshold: u8) -> Transformer2d {
        OverwriteIfAbove(OverwriteWithThreshold::new(value, threshold))
    }

    pub fn new_overwrite_if_below(value: u8, threshold: u8) -> Transformer2d {
        OverwriteIfBelow(OverwriteWithThreshold::new(value, threshold))
    }

    /// Transforms 2 inputs into an output.
    pub fn transform(&self, input0: u8, input1: u8) -> u8 {
        match self {
            Clusterer(clusterer) => clusterer.cluster(input0, input1),
            Const(value) => *value,
            OverwriteIfAbove(data) => data.overwrite_output_if_above(input0, input1),
            OverwriteIfBelow(data) => data.overwrite_output_if_below(input0, input1),
        }
    }
}

/// For serializing, deserializing & validating [`Transformer2d`].
///
///```
///# use ofws_core::data::math::size2d::Size2d;
///# use ofws_core::data::math::transformer::clusterer2d::Clusterer2dData;
///# use ofws_core::data::math::transformer::threshold::OverwriteWithThreshold;
///# use ofws_core::data::math::transformer::transformer2d::{Transformer2dData, assert_eq};
/// let clusterer = Clusterer2dData::new(Size2d::new(1, 2), vec![10, 11]);
/// let overwrite_data = OverwriteWithThreshold::new(100, 200);
///
/// assert_eq(Transformer2dData::Clusterer(clusterer));
/// assert_eq(Transformer2dData::Const(42));
/// assert_eq(Transformer2dData::OverwriteIfAbove(overwrite_data));
/// assert_eq(Transformer2dData::OverwriteIfBelow(overwrite_data));
///```
#[derive(new, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Transformer2dData {
    Clusterer(Clusterer2dData),
    Const(u8),
    OverwriteIfAbove(OverwriteWithThreshold<u8>),
    OverwriteIfBelow(OverwriteWithThreshold<u8>),
}

impl TryFrom<Transformer2dData> for Transformer2d {
    type Error = Transformer2dError;

    fn try_from(data: Transformer2dData) -> Result<Self, Self::Error> {
        match data {
            Transformer2dData::Clusterer(c) => Ok(Clusterer(c.try_into()?)),
            Transformer2dData::Const(value) => Ok(Const(value)),
            Transformer2dData::OverwriteIfAbove(o) => Ok(OverwriteIfAbove(o)),
            Transformer2dData::OverwriteIfBelow(o) => Ok(OverwriteIfBelow(o)),
        }
    }
}

impl From<&Transformer2d> for Transformer2dData {
    fn from(generator: &Transformer2d) -> Self {
        match generator {
            Clusterer(c) => Transformer2dData::Clusterer(c.into()),
            Const(value) => Transformer2dData::Const(*value),
            OverwriteIfAbove(o) => Transformer2dData::OverwriteIfAbove(*o),
            OverwriteIfBelow(o) => Transformer2dData::OverwriteIfBelow(*o),
        }
    }
}

pub fn assert_eq(data: Transformer2dData) {
    let generator: Transformer2d = data.clone().try_into().unwrap();
    let result: Transformer2dData = (&generator).into();
    assert_eq!(result, data)
}
