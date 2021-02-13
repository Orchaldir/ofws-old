use crate::data::math::distance::calculate_distance;
use crate::data::math::generator::generator1d::{Generator1d, Generator1dData};
use crate::data::math::generator::noise::{Noise, NoiseData};
use crate::data::math::size2d::Size2d;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

#[svgbobdoc::transform]
/// Generate values for 2d points.
/// Used for the procedural generation of 2d maps.
pub enum Generator2d {
    /// Feeds the x values to a [`Generator1d`].
    ///
    /// ```
    ///# use ofws_core::data::math::generator::generator1d::Generator1d;
    ///# use ofws_core::data::math::generator::generator2d::Generator2d;
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
    ///# use ofws_core::data::math::generator::generator1d::Generator1d;
    ///# use ofws_core::data::math::generator::generator2d::Generator2d;
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
    ///# use ofws_core::data::math::generator::generator1d::Generator1d;
    ///# use ofws_core::data::math::generator::generator2d::Generator2d;
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
    ///# use ofws_core::data::math::generator::generator2d::Generator2d;
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

/// For serializing, deserializing & validating [`Generator2d`].
///
///```
///# use ofws_core::data::math::generator::generator1d::Generator1dData::InputAsOutput;
///# use ofws_core::data::math::generator::generator2d::{Generator2dData, assert_eq};
///# use ofws_core::data::math::generator::gradient::Gradient;
///# use ofws_core::data::math::generator::noise::NoiseData;
///# use ofws_core::data::math::size2d::Size2d;
/// let noise_data = NoiseData { seed: 300, scale: 5, min_value: 10, max_value: 128 };
///
/// assert_eq(Generator2dData::ApplyToX(InputAsOutput));
/// assert_eq(Generator2dData::ApplyToY(InputAsOutput));
/// assert_eq(Generator2dData::ApplyToDistance { generator: InputAsOutput, center_x: 10, center_y: 20});
/// assert_eq(Generator2dData::IndexGenerator(Size2d::new(3, 5)));
/// assert_eq(Generator2dData::Noise2d(noise_data));
///```
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Generator2dData {
    ApplyToX(Generator1dData),
    ApplyToY(Generator1dData),
    ApplyToDistance {
        generator: Generator1dData,
        center_x: u32,
        center_y: u32,
    },
    IndexGenerator(Size2d),
    Noise2d(NoiseData),
}

impl TryFrom<Generator2dData> for Generator2d {
    type Error = &'static str;

    fn try_from(data: Generator2dData) -> Result<Self, Self::Error> {
        match data {
            Generator2dData::ApplyToX(data) => {
                let generator: Generator1d = data.try_into()?;
                Ok(Generator2d::ApplyToX(generator))
            }
            Generator2dData::ApplyToY(data) => {
                let generator: Generator1d = data.try_into()?;
                Ok(Generator2d::ApplyToY(generator))
            }
            Generator2dData::ApplyToDistance {
                generator,
                center_x,
                center_y,
            } => {
                let generator: Generator1d = generator.try_into()?;
                Ok(Generator2d::new_apply_to_distance(
                    generator, center_x, center_y,
                ))
            }
            Generator2dData::IndexGenerator(size) => Ok(Generator2d::IndexGenerator(size)),
            Generator2dData::Noise2d(data) => {
                let noise: Noise = data.try_into()?;
                Ok(Generator2d::Noise2d(noise))
            }
        }
    }
}

impl From<Generator2d> for Generator2dData {
    fn from(generator: Generator2d) -> Self {
        match generator {
            Generator2d::ApplyToX(generator) => Generator2dData::ApplyToX(generator.into()),
            Generator2d::ApplyToY(generator) => Generator2dData::ApplyToY(generator.into()),
            Generator2d::ApplyToDistance {
                generator,
                center_x,
                center_y,
            } => Generator2dData::ApplyToDistance {
                generator: generator.into(),
                center_x,
                center_y,
            },
            Generator2d::IndexGenerator(size) => Generator2dData::IndexGenerator(size),
            Generator2d::Noise2d(noise) => Generator2dData::Noise2d(noise.into()),
        }
    }
}

pub fn assert_eq(data: Generator2dData) {
    let generator: Generator2d = data.try_into().unwrap();
    let result: Generator2dData = generator.into();
    assert_eq!(result, data)
}
