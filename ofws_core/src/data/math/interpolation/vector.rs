use crate::data::math::interpolation::{Interpolate, Interpolator};

/// Interpolates multiple elements. Each element is paired with its position between 0.0 & 1.0.
#[derive(Debug, PartialEq)]
pub struct VectorInterpolator<T: Clone + Interpolate> {
    vector: Vec<(f32, T)>,
}

impl<T: Clone + Interpolate> VectorInterpolator<T> {
    /// Returns a VectorInterpolator, if the input is valid:
    ///
    /// ```
    ///# use ofws_core::data::math::interpolation::vector::VectorInterpolator;
    /// assert!(VectorInterpolator::new(vec![(0.0,150), (0.5,100), (1.0, 200)]).is_some());
    /// ```
    ///
    /// It needs 2 or more elements:
    ///
    /// ```
    ///# use ofws_core::data::math::interpolation::vector::VectorInterpolator;
    /// assert!(VectorInterpolator::new(vec![(0.0,50)]).is_none());
    /// ```
    ///
    /// The elements must be ordered based in their position:
    ///
    /// ```
    ///# use ofws_core::data::math::interpolation::vector::VectorInterpolator;
    /// assert!(VectorInterpolator::new(vec![(0.5,50),(0.0,200)]).is_none());
    /// ```
    ///
    /// The positions must not be outside of 0 & 1:
    ///
    /// ```
    ///# use ofws_core::data::math::interpolation::vector::VectorInterpolator;
    /// assert!(VectorInterpolator::new(vec![(-0.5,50),(0.8,200)]).is_none());
    /// assert!(VectorInterpolator::new(vec![(0.5,50),(1.2,200)]).is_none());
    /// ```
    pub fn new(vector: Vec<(f32, T)>) -> Option<VectorInterpolator<T>> {
        if vector.len() < 2 {
            return None;
        }

        let mut last_value = 0.0;

        for (value, _) in &vector {
            if *value < last_value {
                return None;
            }
            last_value = *value;
        }

        if last_value > 1.0 {
            return None;
        }

        Some(VectorInterpolator { vector })
    }
}

impl<T: Clone + Interpolate> Interpolator<T> for VectorInterpolator<T> {
    /// Returns the interpolated value.
    ///
    /// ```
    ///# use ofws_core::data::math::interpolation::vector::VectorInterpolator;
    ///# use ofws_core::data::math::interpolation::Interpolator;
    /// let interpolator = VectorInterpolator::new(vec![(0.1,150), (0.5,100), (0.9, 200)]).unwrap();
    ///
    /// assert_eq!(interpolator.interpolate(0.0), 150);
    /// assert_eq!(interpolator.interpolate(0.05), 150);
    /// assert_eq!(interpolator.interpolate(0.1), 150);
    /// assert_eq!(interpolator.interpolate(0.3), 125);
    /// assert_eq!(interpolator.interpolate(0.5), 100);
    /// assert_eq!(interpolator.interpolate(0.7), 150);
    /// assert_eq!(interpolator.interpolate(0.9), 200);
    /// assert_eq!(interpolator.interpolate(1.0), 200);
    /// ```
    fn interpolate(&self, factor: f32) -> T {
        let mut last_entry = self.vector.get(0).unwrap();

        if factor <= last_entry.0 {
            return last_entry.1.clone();
        }

        for entry in &self.vector[1..] {
            if factor <= entry.0 {
                let factor_in_interval = (factor - last_entry.0) / (entry.0 - last_entry.0);
                return last_entry.1.lerp(&entry.1, factor_in_interval);
            }

            last_entry = entry;
        }

        last_entry.1.clone()
    }
}
