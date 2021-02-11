use crate::data::generator::gradient::Gradient;
use crate::data::generator::noise::Noise;

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
    ///# use ofws_core::data::generator::gradient::Gradient;
    /// let gradient = Gradient::new(100, 0, 80, 100);
    /// let generator = Generator1d::AbsoluteGradient1d(gradient);
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
    AbsoluteGradient1d(Gradient),
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
    ///# use ofws_core::data::generator::gradient::Gradient;
    /// let gradient = Gradient::new(100, 200, 1000, 100);
    /// let generator = Generator1d::Gradient1d(gradient);
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
    Gradient1d(Gradient),
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
    Noise1d(Noise),
}

impl Generator1d {
    /// Generates an output for an input.
    pub fn generate(&self, input: u32) -> u8 {
        match self {
            Generator1d::AbsoluteGradient1d(gradient) => gradient.generate_absolute(input),
            Generator1d::Gradient1d(gradient) => gradient.generate(input),
            Generator1d::InputAsOutput => input as u8,
            Generator1d::Noise1d(noise) => noise.generate1d(input),
        }
    }
}
