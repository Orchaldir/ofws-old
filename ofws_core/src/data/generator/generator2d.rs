use crate::data::generator::generator1d::Generator1d;
use crate::data::generator::noise::Noise;
use crate::data::math::distance::calculate_distance;
use crate::data::size2d::Size2d;

#[svgbobdoc::transform]
/// Generate values for 2d points.
/// Used for the procedural generation of 2d maps.
pub enum Generator2d {
    /// Feeds the x values to a [`Generator1d`].
    ///
    /// ```
    ///# use ofws_core::data::generator::generator1d::Generator1d;
    ///# use ofws_core::data::generator::generator2d::Generator2d;
    /// let generator = Generator2d::new_apply_to_x(Generator1d::InputAsOutput);
    ///
    /// assert_eq!(generator.generate(0, 0), 0);
    /// assert_eq!(generator.generate(1, 0), 1);
    /// assert_eq!(generator.generate(2, 0), 2);
    /// assert_eq!(generator.generate(0, 1), 0);
    /// assert_eq!(generator.generate(1, 1), 1);
    /// assert_eq!(generator.generate(2, 1), 2);
    /// assert_eq!(generator.generate(0, 2), 0);
    /// assert_eq!(generator.generate(1, 2), 1);
    /// assert_eq!(generator.generate(2, 2), 2);
    /// ```
    ApplyToX(Generator1d),
    /// Feeds the y values to a [`Generator1d`].
    ///
    /// ```
    ///# use ofws_core::data::generator::generator1d::Generator1d;
    ///# use ofws_core::data::generator::generator2d::Generator2d;
    /// let generator = Generator2d::new_apply_to_y(Generator1d::InputAsOutput);
    ///
    /// assert_eq!(generator.generate(0, 0), 0);
    /// assert_eq!(generator.generate(1, 0), 0);
    /// assert_eq!(generator.generate(2, 0), 0);
    /// assert_eq!(generator.generate(0, 1), 1);
    /// assert_eq!(generator.generate(1, 1), 1);
    /// assert_eq!(generator.generate(2, 1), 1);
    /// assert_eq!(generator.generate(0, 2), 2);
    /// assert_eq!(generator.generate(1, 2), 2);
    /// assert_eq!(generator.generate(2, 2), 2);
    /// ```
    ApplyToY(Generator1d),
    /// Feeds the distance from a point to a [`Generator1d`].
    ///
    /// ```
    ///# use ofws_core::data::generator::generator1d::Generator1d;
    ///# use ofws_core::data::generator::generator2d::Generator2d;
    /// let generator = Generator2d::new_apply_to_distance(Generator1d::InputAsOutput, 10, 5);
    ///
    /// assert_eq!(generator.generate(10,  5), 0);
    /// assert_eq!(generator.generate(10,  0), 5);
    /// assert_eq!(generator.generate(10, 10), 5);
    /// assert_eq!(generator.generate( 5,  5), 5);
    /// assert_eq!(generator.generate(15,  5), 5);
    /// ```
    ApplyToDistance {
        generator: Generator1d,
        center_x: u32,
        center_y: u32,
    },
    /// Generates the index of each 2d point.
    ///
    /// ```
    ///# use ofws_core::data::generator::generator2d::Generator2d;
    /// let generator = Generator2d::new_index(2, 3);
    ///
    /// assert_eq!(generator.generate(0, 0), 0);
    /// assert_eq!(generator.generate(1, 0), 1);
    /// assert_eq!(generator.generate(0, 1), 2);
    /// assert_eq!(generator.generate(1, 1), 3);
    /// assert_eq!(generator.generate(0, 2), 4);
    /// assert_eq!(generator.generate(1, 2), 5);
    /// ```
    IndexGenerator(Size2d),
    /// Generates noise for each 2d point.
    Noise2d(Noise),
}

impl Generator2d {
    pub fn new_apply_to_x(generator: Generator1d) -> Generator2d {
        Generator2d::ApplyToX(generator)
    }

    pub fn new_apply_to_y(generator: Generator1d) -> Generator2d {
        Generator2d::ApplyToY(generator)
    }

    pub fn new_apply_to_distance(generator: Generator1d, x: u32, y: u32) -> Generator2d {
        Generator2d::ApplyToDistance {
            generator,
            center_x: x,
            center_y: y,
        }
    }

    pub fn new_index(width: u32, height: u32) -> Generator2d {
        Generator2d::IndexGenerator(Size2d::new(width, height))
    }

    /// Generates a value for a 2d point (x,y).
    pub fn generate(&self, x: u32, y: u32) -> u8 {
        match self {
            Generator2d::ApplyToX(generator) => generator.generate(x),
            Generator2d::ApplyToY(generator) => generator.generate(y),
            Generator2d::ApplyToDistance {
                generator,
                center_x,
                center_y,
            } => {
                let distance = calculate_distance(*center_x, *center_y, x, y);
                generator.generate(distance)
            }
            Generator2d::IndexGenerator(size) => size.saturating_to_index(x, y) as u8,
            Generator2d::Noise2d(noise) => noise.generate2d(x, y),
        }
    }
}
