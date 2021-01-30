pub mod pair;
pub mod vector;

/// Define how to interpolate between elements of the same type.
pub trait Interpolate {
    /// Linear interpolation between 2 elements of the same type.
    fn lerp(&self, other: &Self, factor: f32) -> Self;
}

impl Interpolate for u8 {
    /// Linear interpolation between 2 u8.
    ///
    /// ```
    ///# use ofws_core::data::math::interpolation::Interpolate;
    ///
    /// assert_eq!(100.lerp(&200, 0.5), 150u8);
    /// ```
    fn lerp(&self, other: &u8, factor: f32) -> u8 {
        lerp(*self, *other, factor)
    }
}

/// Interpolates between 2 or more elements of the same type.
pub trait Interpolator<T: Interpolate> {
    /// Returns the interpolated value.
    fn interpolate(&self, factor: f32) -> T;
}

/// Interpolates between 2 u8 linearly.
///
/// ```
///# use ofws_core::data::math::interpolation::lerp;
///
/// assert_eq!(lerp(100, 200, 0.0), 100);
/// assert_eq!(lerp(100, 200, 0.5), 150);
/// assert_eq!(lerp(100, 200, 1.0), 200);
/// ```
pub fn lerp(start: u8, end: u8, factor: f32) -> u8 {
    if factor > 1.0 {
        return end;
    }

    if end >= start {
        let diff = (end - start) as f32;
        return start + (diff * factor) as u8;
    }

    let diff = (start - end) as f32;

    start - (diff * factor) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp_from_high_to_low() {
        assert_eq!(lerp(200, 100, 0.0), 200);
        assert_eq!(lerp(200, 100, 0.5), 150);
        assert_eq!(lerp(200, 100, 1.0), 100);
    }

    #[test]
    fn test_lerp_with_negative_factor() {
        assert_eq!(lerp(100, 200, -0.5), 100);
        assert_eq!(lerp(200, 100, -0.5), 200);
    }

    #[test]
    fn test_lerp_with_too_high_factor() {
        assert_eq!(lerp(100, 200, 2.0), 200);
        assert_eq!(lerp(200, 100, 2.5), 100);
    }
}
