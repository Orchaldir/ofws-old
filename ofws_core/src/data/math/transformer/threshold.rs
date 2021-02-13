use crate::data::math::transformer::Transformed;
use serde::{Deserialize, Serialize};

/// Overwrites the input if above or below a threshold.
#[derive(Debug, Serialize, Deserialize)]
pub struct OverwriteWithThreshold<T: Transformed> {
    value: T,
    threshold: T,
}

impl<T: Transformed> OverwriteWithThreshold<T> {
    pub fn new(value: T, threshold: T) -> OverwriteWithThreshold<T> {
        OverwriteWithThreshold { value, threshold }
    }

    /// Overwrites the input if equal or above a threshold.
    ///
    /// ```
    ///# use ofws_core::data::math::transformer::threshold::OverwriteWithThreshold;
    /// let overwrite = OverwriteWithThreshold::new(42, 100);
    ///
    /// assert_eq!(overwrite.overwrite_if_above(  0),  0);
    /// assert_eq!(overwrite.overwrite_if_above( 99), 99);
    /// assert_eq!(overwrite.overwrite_if_above(100), 42);
    /// assert_eq!(overwrite.overwrite_if_above(101), 42);
    /// assert_eq!(overwrite.overwrite_if_above(255), 42);
    /// ```
    pub fn overwrite_if_above(&self, input: T) -> T {
        if input >= self.threshold {
            self.value
        } else {
            input
        }
    }

    /// Overwrites the input if equal or above a threshold.
    ///
    /// ```
    ///# use ofws_core::data::math::transformer::threshold::OverwriteWithThreshold;
    /// let overwrite = OverwriteWithThreshold::new(42, 100);
    ///
    /// assert_eq!(overwrite.overwrite_if_below(  0),  42);
    /// assert_eq!(overwrite.overwrite_if_below( 99),  42);
    /// assert_eq!(overwrite.overwrite_if_below(100),  42);
    /// assert_eq!(overwrite.overwrite_if_below(101), 101);
    /// assert_eq!(overwrite.overwrite_if_below(255), 255);
    /// ```
    pub fn overwrite_if_below(&self, input: T) -> T {
        if input <= self.threshold {
            self.value
        } else {
            input
        }
    }
}
