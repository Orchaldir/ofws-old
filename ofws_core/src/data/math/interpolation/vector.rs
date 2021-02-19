use crate::data::math::interpolation::Interpolate;
use num_traits::int::PrimInt;
use num_traits::AsPrimitive;
use serde::{Deserialize, Serialize};

pub trait Threshold: PrimInt + AsPrimitive<f32> + Clone + Copy {}

impl Threshold for u8 {}
impl Threshold for u32 {}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct InterpolationEntry<T: Threshold, V: Interpolate> {
    threshold: T,
    value: V,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct VectorInterpolation<T: Threshold, V: Interpolate> {
    vector: Vec<InterpolationEntry<T, V>>,
}

impl<T: Threshold, V: Interpolate> VectorInterpolation<T, V> {
    /// Returns a VectorInterpolation, if the input is valid. It needs 2 or more elements:
    ///
    /// ```
    ///# use ofws_core::data::math::interpolation::vector::VectorInterpolation;
    /// assert!(VectorInterpolation::new(vec![(0u32,50)]).is_err());
    /// ```
    ///
    /// The elements must be ordered based in their position:
    ///
    /// ```
    ///# use ofws_core::data::math::interpolation::vector::VectorInterpolation;
    /// assert!(VectorInterpolation::new(vec![(50u32,50),(0,200)]).is_err());
    /// ```
    pub fn new(vector: Vec<(T, V)>) -> Result<VectorInterpolation<T, V>, &'static str> {
        if vector.len() < 2 {
            return Err("The vector needs at least 2 elements!");
        }

        let mut last_value = T::zero();

        for (value, _) in &vector {
            if *value < last_value {
                return Err("The elements of vector are not ordered!");
            }
            last_value = *value;
        }

        Ok(VectorInterpolation {
            vector: vector
                .into_iter()
                .map(|e| InterpolationEntry {
                    threshold: e.0,
                    value: e.1,
                })
                .collect::<Vec<_>>(),
        })
    }

    /// Interpolates between the values of a vector of [`InterpolationEntry`] based on the input and their thresholds.
    ///
    /// ```
    ///# use ofws_core::data::math::interpolation::vector::VectorInterpolation;
    /// let interpolator = VectorInterpolation::new(vec![(100u32,150), (150,200), (200, 100)]).unwrap();
    ///
    /// assert_eq!(interpolator.interpolate(  0), 150);
    /// assert_eq!(interpolator.interpolate( 50), 150);
    /// assert_eq!(interpolator.interpolate(100), 150);
    /// assert_eq!(interpolator.interpolate(125), 175);
    /// assert_eq!(interpolator.interpolate(150), 200);
    /// assert_eq!(interpolator.interpolate(175), 150);
    /// assert_eq!(interpolator.interpolate(200), 100);
    /// assert_eq!(interpolator.interpolate(255), 100);
    /// ```
    pub fn interpolate(&self, input: T) -> V {
        let mut last_entry = self.vector.get(0).unwrap();

        if input <= last_entry.threshold {
            return last_entry.value.clone();
        }

        for entry in &self.vector[1..] {
            if input <= entry.threshold {
                let factor_in_interval = (input - last_entry.threshold).as_()
                    / (entry.threshold - last_entry.threshold).as_();
                return last_entry.value.lerp(&entry.value, factor_in_interval);
            }

            last_entry = entry;
        }

        last_entry.value.clone()
    }
}
