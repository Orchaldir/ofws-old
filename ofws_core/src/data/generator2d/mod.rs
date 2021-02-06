use crate::data::size2d::Size2d;

pub mod composition;
pub mod gradient;

/// A trait to generate values for 2d points.
/// Used for the procedural generation of 2d maps.
pub trait Generator2d {
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

impl Generator2d for ConstantValue {
    /// Generates a value for a 2d point (x,y).
    ///
    /// ```
    ///# use ofws_core::data::generator2d::{ConstantValue, Generator2d};
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

impl Generator2d for MockGenerator {
    /// Generates a value for a 2d point (x,y).
    ///
    /// ```
    ///# use ofws_core::data::generator2d::{Generator2d, MockGenerator};
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

/// Generates the index of each 2d point.
pub struct IndexGenerator {
    size: Size2d,
}

impl IndexGenerator {
    pub fn new(size: Size2d) -> IndexGenerator {
        IndexGenerator { size }
    }
}

impl Generator2d for IndexGenerator {
    /// Generates a value for a 2d point (x,y).
    ///
    /// ```
    ///# use ofws_core::data::generator2d::{Generator2d, IndexGenerator};
    ///# use ofws_core::data::size2d::Size2d;
    /// let generator = IndexGenerator::new(Size2d::new(2, 3));
    ///
    /// assert_eq!(generator.generate(0, 0), 0);
    /// assert_eq!(generator.generate(1, 0), 1);
    /// assert_eq!(generator.generate(0, 1), 2);
    /// assert_eq!(generator.generate(1, 1), 3);
    /// assert_eq!(generator.generate(0, 2), 4);
    /// assert_eq!(generator.generate(1, 2), 5);
    /// ```
    fn generate(&self, x: u32, y: u32) -> u8 {
        self.size.to_index(x, y) as u8
    }
}
