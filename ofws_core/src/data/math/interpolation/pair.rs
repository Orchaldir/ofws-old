use crate::data::math::interpolation::{Interpolate, Interpolator};

/// Interpolates 2 elements.
pub struct PairInterpolator<T: Interpolate> {
    first: T,
    second: T,
}

impl<T: Interpolate> PairInterpolator<T> {
    pub fn new(first: T, second: T) -> PairInterpolator<T> {
        PairInterpolator { first, second }
    }
}

impl<T: Interpolate> Interpolator<T> for PairInterpolator<T> {
    /// Returns the interpolated value.
    ///
    /// ```
    ///# use ofws_core::data::math::interpolation::pair::PairInterpolator;
    ///# use ofws_core::data::math::interpolation::Interpolator;
    /// let interpolator = PairInterpolator::new(100, 200);
    ///
    /// assert_eq!(interpolator.interpolate(0.5), 150);
    /// ```
    fn interpolate(&self, factor: f32) -> T {
        self.first.lerp(&self.second, factor)
    }
}
