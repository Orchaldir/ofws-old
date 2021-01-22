pub mod composition;
pub mod gradient;

/// A trait to generate values for 2d points.
/// Used for the procedural generation of 2d maps.
pub trait Generator {
    /// Generates a value for a 2d point (x,y).
    fn generate(&self, x: u32, y: u32) -> u8;
}

/// Generates the same value for all 2d points.
pub struct ConstantValue {
    value: u8,
}

impl ConstantValue {
    pub fn new(value: u8) -> ConstantValue {
        ConstantValue { value }
    }
}

impl Generator for ConstantValue {
    /// Generates a value for a 2d point (x,y).
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

/// Generates a specific value for a specific 2d point and 0 otherwise.
pub struct MockGenerator {
    x: u32,
    y: u32,
    value: u8,
}

impl MockGenerator {
    pub fn new(x: u32, y: u32, value: u8) -> MockGenerator {
        MockGenerator { x, y, value }
    }
}

impl Generator for MockGenerator {
    /// Generates a value for a 2d point (x,y).
    ///
    /// ```
    ///# use ofws_core::data::generator::{Generator, MockGenerator};
    /// let generator = MockGenerator::new(3, 4, 42);
    /// assert_eq!(generator.generate(0, 0), 0);
    /// assert_eq!(generator.generate(10, 0), 0);
    /// assert_eq!(generator.generate(0, 20), 0);
    /// assert_eq!(generator.generate(123, 345), 0);
    /// assert_eq!(generator.generate(3, 4), 42);
    /// ```
    fn generate(&self, x: u32, y: u32) -> u8 {
        if self.x == x && self.y == y {
            self.value
        } else {
            0
        }
    }
}
