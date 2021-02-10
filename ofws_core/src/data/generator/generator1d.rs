use crate::data::math::distance::abs_diff;
use crate::data::math::interpolation::lerp;
use noise::{NoiseFn, Seedable, SuperSimplex};

#[svgbobdoc::transform]
/// Generates values for a 1d input.
pub enum Generator1d {
    /// Generates a linear gradient between a center and both sides.
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
    ///    end |----*       *----
    ///        |
    ///        +----*-------*---> input
    ///          center +- length
    /// ```
    ///
    /// # Example
    ///
    /// ```
    ///# use ofws_core::data::generator::generator1d::Generator1d;
    /// let generator = Generator1d::new_absolute_gradient(100, 0, 80, 100);
    ///
    /// assert_eq!(generator.generate(  0),  20);
    /// assert_eq!(generator.generate(  1),  21);
    /// assert_eq!(generator.generate( 79),  99);
    /// assert_eq!(generator.generate( 80), 100);
    /// assert_eq!(generator.generate( 81),  99);
    /// assert_eq!(generator.generate(130),  50);
    /// assert_eq!(generator.generate(179),   1);
    /// assert_eq!(generator.generate(180),   0);
    /// assert_eq!(generator.generate(181),   0);
    /// assert_eq!(generator.generate(200),   0);
    /// ```
    AbsoluteGradient1d {
        value_center: u8,
        value_end: u8,
        center: u32,
        length: u32,
    },
    /// Generates a linear gradient between a start and an end value.
    ///
    /// # Diagram
    ///
    /// ```svgbob
    ///     value
    ///       ^
    ///       |
    ///       |
    ///   end |        *------
    ///       |       /
    ///       |      /
    ///       |     /
    /// start |----*
    ///       |
    ///       +----*---*------> input
    ///         start  end
    /// ```
    ///
    /// # Example
    ///
    ///```
    ///# use ofws_core::data::generator::generator1d::Generator1d;
    /// let generator = Generator1d::new_gradient(100, 200, 1000, 100);
    ///
    /// assert_eq!(generator.generate(   0), 100);
    /// assert_eq!(generator.generate( 500), 100);
    /// assert_eq!(generator.generate(1000), 100);
    /// assert_eq!(generator.generate(1001), 101);
    /// assert_eq!(generator.generate(1050), 150);
    /// assert_eq!(generator.generate(1099), 199);
    /// assert_eq!(generator.generate(1100), 200);
    /// assert_eq!(generator.generate(1101), 200);
    /// assert_eq!(generator.generate(1200), 200);
    ///```
    Gradient1d {
        value_start: u8,
        value_end: u8,
        start: u32,
        length: u32,
    },
    /// Returns the input as output.
    ///
    /// # Example
    ///
    ///```
    ///# use ofws_core::data::generator::generator1d::Generator1d::InputAsOutput;
    ///
    /// assert_eq!(InputAsOutput.generate(0), 0);
    /// assert_eq!(InputAsOutput.generate(1), 1);
    /// assert_eq!(InputAsOutput.generate(2), 2);
    ///```
    InputAsOutput,
    /// Generates values with Super Simplex noise.
    Noise1d {
        algo: Box<SuperSimplex>,
        scale: f64,
        factor: f64,
    },
}

impl Generator1d {
    pub fn new_absolute_gradient(
        value_center: u8,
        value_end: u8,
        center: u32,
        length: u32,
    ) -> Generator1d {
        Generator1d::AbsoluteGradient1d {
            value_center,
            value_end,
            center,
            length,
        }
    }

    pub fn new_gradient(value_start: u8, value_end: u8, start: u32, length: u32) -> Generator1d {
        Generator1d::Gradient1d {
            value_start,
            value_end,
            start,
            length,
        }
    }

    pub fn new_noise(seed: u32, scale: f64, max_value: u8) -> Generator1d {
        Generator1d::Noise1d {
            algo: Box::new(SuperSimplex::new().set_seed(seed)),
            scale,
            factor: max_value as f64 / 2.0,
        }
    }

    /// Generates an output for an input.
    pub fn generate(&self, input: u32) -> u8 {
        match self {
            Generator1d::AbsoluteGradient1d {
                value_center,
                value_end,
                center,
                length,
            } => {
                let distance = abs_diff(*center, input) as f32;
                let factor = distance / *length as f32;
                lerp(*value_center, *value_end, factor)
            }
            Generator1d::Gradient1d {
                value_start,
                value_end,
                start,
                length,
            } => {
                if input <= *start {
                    return *value_start;
                }
                let distance = (input - start) as f32;
                let factor = distance / *length as f32;
                lerp(*value_start, *value_end, factor)
            }
            Generator1d::InputAsOutput => input as u8,
            Generator1d::Noise1d {
                algo,
                scale,
                factor,
            } => {
                let input = input as f64 / scale;
                let positive_value = algo.get([input, 0.0]) + 1.0;
                (positive_value * factor) as u8
            }
        }
    }
}
