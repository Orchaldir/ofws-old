use crate::data::math::generator::gradient::Gradient;
use crate::data::math::generator::noise::{Noise, NoiseData, NoiseError};
use crate::data::math::interpolation::vector::VectorInterpolation;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use Generator1d::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Generator1dError {
    Noise(NoiseError),
}

impl From<NoiseError> for Generator1dError {
    fn from(error: NoiseError) -> Self {
        Generator1dError::Noise(error)
    }
}

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
    ///# use ofws_core::data::math::generator::generator1d::Generator1d;
    ///# use ofws_core::data::math::generator::gradient::Gradient;
    /// let gradient = Gradient::new(100, 0, 80, 100);
    /// let generator = Generator1d::AbsoluteGradient(gradient);
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
    AbsoluteGradient(Gradient),
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
    ///# use ofws_core::data::math::generator::generator1d::Generator1d;
    ///# use ofws_core::data::math::generator::gradient::Gradient;
    /// let gradient = Gradient::new(100, 200, 1000, 100);
    /// let generator = Generator1d::Gradient(gradient);
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
    Gradient(Gradient),
    /// Returns the input as output.
    ///
    /// # Example
    ///
    ///```
    ///# use ofws_core::data::math::generator::generator1d::Generator1d::InputAsOutput;
    ///
    /// assert_eq!(InputAsOutput.generate(0), 0);
    /// assert_eq!(InputAsOutput.generate(1), 1);
    /// assert_eq!(InputAsOutput.generate(2), 2);
    ///```
    InputAsOutput,
    /// Interpolates multiple elements.
    ///
    /// # Diagram
    ///
    /// ```svgbob
    ///      value
    ///        ^
    ///        |                   *
    ///        |                  / \
    ///        |        *        /   \
    ///        |       / \      /     *--
    ///        |      /   *----*
    ///        |     /
    ///        |----*
    ///        |
    ///        +----*-----------------*--> input
    ///           first             last
    /// ```
    ///
    /// # Example
    ///
    /// ```
    ///# use ofws_core::data::math::generator::generator1d::Generator1d::InterpolateVector;
    ///# use ofws_core::data::math::interpolation::vector::VectorInterpolation;
    /// let interpolator = VectorInterpolation::new(vec![(100u32,150), (150,200), (200, 100)]). unwrap();
    /// let generator = InterpolateVector(interpolator);
    ///
    /// assert_eq!(generator.generate(125), 175);
    /// ```
    InterpolateVector(VectorInterpolation<u32, u8>),
    /// Generates values with [`Noise`].
    Noise(Noise),
}

impl Generator1d {
    /// Generates an output for an input.
    pub fn generate(&self, input: u32) -> u8 {
        match self {
            AbsoluteGradient(gradient) => gradient.generate_absolute(input),
            Gradient(gradient) => gradient.generate(input),
            InputAsOutput => input as u8,
            InterpolateVector(interpolator) => interpolator.interpolate(input),
            Noise(noise) => noise.generate1d(input),
        }
    }
}

/// For serializing, deserializing & validating [`Generator1d`].
///
///```
///# use ofws_core::data::math::generator::generator1d::{Generator1dData, assert_eq};
///# use ofws_core::data::math::generator::gradient::Gradient;
///# use ofws_core::data::math::generator::noise::NoiseData;
///# use ofws_core::data::math::interpolation::vector::VectorInterpolation;
/// let gradient = Gradient::new(0, 255, 1000, 500);
/// let interpolator = VectorInterpolation::new(vec![(100,150), (150,200), (200, 100)]). unwrap();
/// let noise_data = NoiseData { seed: 300, scale: 5, min_value: 10, max_value: 128 };
///
/// assert_eq(Generator1dData::AbsoluteGradient(gradient));
/// assert_eq(Generator1dData::Gradient(gradient));
/// assert_eq(Generator1dData::InputAsOutput);
/// assert_eq(Generator1dData::InterpolateVector(interpolator));
/// assert_eq(Generator1dData::Noise(noise_data));
///```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Generator1dData {
    AbsoluteGradient(Gradient),
    Gradient(Gradient),
    InputAsOutput,
    InterpolateVector(VectorInterpolation<u32, u8>),
    Noise(NoiseData),
}

type Data = Generator1dData;

impl TryFrom<Generator1dData> for Generator1d {
    type Error = Generator1dError;

    fn try_from(data: Generator1dData) -> Result<Self, Self::Error> {
        match data {
            Data::AbsoluteGradient(gradient) => Ok(AbsoluteGradient(gradient)),
            Data::Gradient(gradient) => Ok(Gradient(gradient)),
            Data::InputAsOutput => Ok(InputAsOutput),
            Data::InterpolateVector(interpolator) => Ok(InterpolateVector(interpolator)),
            Data::Noise(noise_data) => {
                let noise: Noise = noise_data.try_into()?;
                Ok(Noise(noise))
            }
        }
    }
}

impl From<&Generator1d> for Generator1dData {
    fn from(generator: &Generator1d) -> Self {
        match generator {
            AbsoluteGradient(gradient) => Data::AbsoluteGradient(*gradient),
            Gradient(gradient) => Data::Gradient(*gradient),
            InputAsOutput => Data::InputAsOutput,
            InterpolateVector(interpolator) => Data::InterpolateVector(interpolator.clone()),
            Noise(noise) => Data::Noise(noise.into()),
        }
    }
}

pub fn assert_eq(data: Generator1dData) {
    let generator: Generator1d = data.clone().try_into().unwrap();
    let result: Generator1dData = (&generator).into();
    assert_eq!(result, data)
}
