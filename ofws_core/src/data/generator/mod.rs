/// A trait to generate values for 2d points.
/// Used for the procedural generation of 2d maps.
pub trait Generator {
    /// Generates a value for a 2d point.
    fn generate(&self, x: u32, y: u32) -> u8;
}

/// Returns the same value for all points.
pub struct ConstantValue {
    value: u8,
}

impl ConstantValue {
    pub fn new(value: u8) -> ConstantValue {
        ConstantValue { value }
    }
}

impl Generator for ConstantValue {
    /// Returns the same value for all points.
    ///
    /// ```
    ///# use ofws_core::data::generator::{ConstantValue, Generator};
    /// let generator = ConstantValue::new(42);
    /// assert_eq!(generator.generate(0, 0), 42);
    /// assert_eq!(generator.generate(10, 0), 42);
    /// assert_eq!(generator.generate(0, 20), 42);
    /// assert_eq!(generator.generate(123, 345), 42);
    /// ```
    fn generate(&self, _x: u32, _y: u32) -> u8 {
        self.value
    }
}
