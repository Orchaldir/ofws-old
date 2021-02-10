use crate::data::generator::generator1d::Generator1d;
use crate::data::generator2d::Generator2d;
use crate::data::math::distance::calculate_distance;

#[svgbobdoc::transform]
/// Generates a circular gradient around a 2d point.
///
/// # Diagram
///
/// ```svgbob
///  y-axis
///    ^
///    |
///    |       _____
///    |     ,'     `.
///    |    /         \
///  y *   (     .     )
///    |    \         /
///    |     `._____.'
///    |
///    +---------*----> x-axis
///              x
/// ```
///
/// * Points on the circle & outside have the value of *value_end*.
/// * The point (x,y) has the value of *value_center*.
/// * Points inside the circle are a linear interpolation between those values.
pub struct CircularGradient {
    gradient: Generator1d,
    x: u32,
    y: u32,
}

impl CircularGradient {
    pub fn new(
        value_center: u8,
        value_end: u8,
        x: u32,
        y: u32,
        max_distance: u32,
    ) -> CircularGradient {
        CircularGradient {
            gradient: Generator1d::new_gradient(value_center, value_end, max_distance),
            x,
            y,
        }
    }
}

impl Generator2d for CircularGradient {
    /// Generates a value for a 2d point (x,y).
    ///
    /// ```
    ///# use ofws_core::data::generator2d::Generator2d;
    ///# use ofws_core::data::generator2d::gradient::circular::CircularGradient;
    /// let generator = CircularGradient::new(60, 10, 50, 50, 50);
    ///
    /// assert_eq!(generator.generate( 50,  0), 10);
    /// assert_eq!(generator.generate( 50,  1), 11);
    /// assert_eq!(generator.generate( 50, 49), 59);
    /// assert_eq!(generator.generate( 50, 50), 60);
    /// assert_eq!(generator.generate( 50, 51), 59);
    /// assert_eq!(generator.generate( 50, 99), 11);
    /// assert_eq!(generator.generate( 50,100), 10);
    /// assert_eq!(generator.generate(  0, 50), 10);
    /// assert_eq!(generator.generate(  1, 50), 11);
    /// assert_eq!(generator.generate( 49, 50), 59);
    /// assert_eq!(generator.generate( 50, 50), 60);
    /// assert_eq!(generator.generate( 51, 50), 59);
    /// assert_eq!(generator.generate( 99, 50), 11);
    /// assert_eq!(generator.generate(100, 50), 10);
    /// ```
    fn generate(&self, x: u32, y: u32) -> u8 {
        let distance = calculate_distance(self.x, self.y, x, y);
        self.gradient.generate(distance)
    }
}
