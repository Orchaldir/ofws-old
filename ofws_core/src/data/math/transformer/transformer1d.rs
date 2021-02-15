use crate::data::math::transformer::threshold::OverwriteWithThreshold;
use crate::data::math::transformer::Transformed;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Transforms the input into an output of the same type.
#[derive(Debug, Serialize, Deserialize)]
pub enum Transformer1d<T: Transformed> {
    /// Overwrites the input, if it is above a threshold.
    OverwriteIfAbove(OverwriteWithThreshold<T>),
    /// Overwrites the input, if it is below a threshold.
    OverwriteIfBelow(OverwriteWithThreshold<T>),
    /// Overwrites the input, if the map containes a value for it.
    ///
    /// ```
    ///# use ofws_core::data::math::transformer::transformer1d::Transformer1d;
    /// let hashmap = vec![(1u8, 25u8), (3, 100)].into_iter().collect();
    /// let selector = Transformer1d::OverwriteWithMap(hashmap);
    ///
    /// assert_eq!(selector.get(0), 0);
    /// assert_eq!(selector.get(1), 25);
    /// assert_eq!(selector.get(2), 2);
    /// assert_eq!(selector.get(3), 100);
    /// assert_eq!(selector.get(4), 4);
    /// ```
    OverwriteWithMap(HashMap<T, T>),
}

impl<T: Transformed> Transformer1d<T> {
    pub fn new_overwrite_if_above(value: T, threshold: T) -> Transformer1d<T> {
        Transformer1d::OverwriteIfAbove(OverwriteWithThreshold::new(value, threshold))
    }

    pub fn new_overwrite_if_below(value: T, threshold: T) -> Transformer1d<T> {
        Transformer1d::OverwriteIfBelow(OverwriteWithThreshold::new(value, threshold))
    }

    /// Selects an object of type T based on the input.
    pub fn get(&self, input: T) -> T {
        match self {
            Transformer1d::OverwriteIfAbove(data) => data.overwrite_if_above(input),
            Transformer1d::OverwriteIfBelow(data) => data.overwrite_if_below(input),
            Transformer1d::OverwriteWithMap(hashmap) => {
                hashmap.get(&input).copied().unwrap_or(input)
            }
        }
    }
}
