use crate::data::generator2d::gradient::Gradient;
use crate::data::generator2d::Generator2d;
use crate::data::math::distance::abs_diff;

#[svgbobdoc::transform]
/// Generates a linear gradient between a center and 2 equidistant end points along the x-axis.
///
/// # Diagram
///
/// ```svgbob
///      value
///        ^
///        |
///        |        center
/// center |        *
///        |       / \
///        |      /   \
///        |     /     \
///    end |----*       *---
///        |
///        +----*-------*--> x-axis
///          start      end
/// ```
///
pub struct AbsoluteGradientX {
    gradient: Gradient,
    center: u32,
}

impl AbsoluteGradientX {
    pub fn new(
        value_center: u8,
        value_end: u8,
        center: u32,
        max_distance: u32,
    ) -> AbsoluteGradientX {
        AbsoluteGradientX {
            gradient: Gradient::new(value_center, value_end, max_distance),
            center,
        }
    }
}

impl Generator2d for AbsoluteGradientX {
    /// Generates a value for a 2d point (x,y).
    ///
    /// ```
    ///# use ofws_core::data::generator2d::Generator2d;
    ///# use ofws_core::data::generator2d::gradient::absolute::AbsoluteGradientX;
    /// let generator = AbsoluteGradientX::new(100, 0, 80, 100);
    ///
    /// assert_eq!(generator.generate(  0,  0),  20);
    /// assert_eq!(generator.generate(  1,  1),  21);
    /// assert_eq!(generator.generate( 79, 12),  99);
    /// assert_eq!(generator.generate( 80, 23), 100);
    /// assert_eq!(generator.generate( 81, 34), 99);
    /// assert_eq!(generator.generate(130,  0), 50);
    /// assert_eq!(generator.generate(179, 45),  1);
    /// assert_eq!(generator.generate(180, 56),  0);
    /// assert_eq!(generator.generate(181, 66),  0);
    /// assert_eq!(generator.generate(200,  0),  0);
    /// ```
    fn generate(&self, x: u32, _y: u32) -> u8 {
        let distance = abs_diff(self.center, x);
        self.gradient.generate(distance)
    }
}

/// Generates a linear gradient between a center and 2 equidistant end points along the y-axis.
///
/// # Diagram
///
/// See [`AbsoluteGradientX`].
///
pub struct AbsoluteGradientY {
    gradient: Gradient,
    center: u32,
}

impl AbsoluteGradientY {
    pub fn new(
        value_center: u8,
        value_end: u8,
        center: u32,
        max_distance: u32,
    ) -> AbsoluteGradientY {
        AbsoluteGradientY {
            gradient: Gradient::new(value_center, value_end, max_distance),
            center,
        }
    }
}

impl Generator2d for AbsoluteGradientY {
    /// Generates a value for a 2d point (x,y).
    ///
    /// ```
    ///# use ofws_core::data::generator2d::Generator2d;
    ///# use ofws_core::data::generator2d::gradient::absolute::AbsoluteGradientY;
    /// let generator = AbsoluteGradientY::new(100, 0, 80, 100);
    ///
    /// assert_eq!(generator.generate(  0,   0),  20);
    /// assert_eq!(generator.generate(  1,   1),  21);
    /// assert_eq!(generator.generate( 12,  79),  99);
    /// assert_eq!(generator.generate( 23,  80), 100);
    /// assert_eq!(generator.generate( 34,  81), 99);
    /// assert_eq!(generator.generate(  0, 130), 50);
    /// assert_eq!(generator.generate( 45, 179),  1);
    /// assert_eq!(generator.generate( 56, 180),  0);
    /// assert_eq!(generator.generate( 66, 181),  0);
    /// assert_eq!(generator.generate(  0, 200),  0);
    /// ```
    fn generate(&self, _x: u32, y: u32) -> u8 {
        let distance = abs_diff(self.center, y);
        self.gradient.generate(distance)
    }
}
