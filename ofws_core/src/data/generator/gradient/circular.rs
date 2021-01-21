use crate::data::generator::gradient::Gradient;
use crate::data::generator::Generator;
use crate::data::math::distance::calculate_distance;

/// Returns a circular gradient around a 2d point.
pub struct CircularGradient {
    gradient: Gradient,
    x: u32,
    y: u32,
}

impl CircularGradient {
    pub fn new(
        value_start: u8,
        value_end: u8,
        x: u32,
        y: u32,
        max_distance: u32,
    ) -> CircularGradient {
        CircularGradient {
            gradient: Gradient::new(value_start, value_end, max_distance),
            x,
            y,
        }
    }
}

impl Generator for CircularGradient {
    /// Returns a circular gradient around a 2d point.
    ///
    /// ```
    ///# use ofws_core::data::generator::Generator;
    ///# use ofws_core::data::generator::gradient::circular::CircularGradient;
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
