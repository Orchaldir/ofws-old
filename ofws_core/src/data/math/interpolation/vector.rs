use crate::data::math::interpolation::{Interpolate, Interpolator};

/// Interpolates multiple elements. Each element is paired with its position between 0.0 & 1.0.
#[derive(Debug, PartialEq)]
pub struct VectorInterpolator<T: Interpolate> {
    elements: Vec<(f32, T)>,
}

impl<T: Interpolate> VectorInterpolator<T> {
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
    pub fn new(elements: Vec<(f32, T)>) -> Option<VectorInterpolator<T>> {
        if elements.len() < 2 {
            return None;
        }

        let mut last_value = 0.0;

        for (value, _) in &elements {
            if *value < last_value {
                return None;
            }
            last_value = *value;
        }

        if last_value > 1.0 {
            return None;
        }

        Some(VectorInterpolator { elements })
    }
}

impl<T: Interpolate> Interpolator<T> for VectorInterpolator<T> {
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
        let mut last_entry = self.elements.get(0).unwrap();

        if factor <= last_entry.0 {
            return last_entry.1.lerp(&last_entry.1, 0.0);
        }

        for entry in &self.elements[1..] {
            if factor <= entry.0 {
                let factor = (factor - last_entry.0) / (entry.0 - last_entry.0);
                return last_entry.1.lerp(&entry.1, factor);
            }

            last_entry = entry;
        }

        last_entry.1.lerp(&last_entry.1, 0.0)
    }
}
