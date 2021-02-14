use crate::data::map::generation::step::{get_attribute_id, GenerationStepError};
use crate::data::map::Map2d;
use crate::data::math::generator::generator2d::{Generator2d, Generator2dData};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

/// Distorts an [`Attribute`] along 2 dimensions.
#[derive(new)]
pub struct Distortion2d {
    attribute_id: usize,
    generator_x: Generator2d,
    generator_y: Generator2d,
}

impl Distortion2d {
    fn distort_map(&self, map: &Map2d) -> Vec<u8> {
        let length = map.size.get_area();
        let attribute = map.get_attribute(self.attribute_id);
        let mut values = Vec::with_capacity(length);

        for y in 0..map.size.height() {
            for x in 0..map.size.width() {
                let shift_x = self.generator_x.generate(x, y) as u32;
                let shift_y = self.generator_y.generate(x, y) as u32;
                let distorted_x = x + shift_x;
                let distorted_y = y + shift_y;
                let index = map.size.saturating_to_index(distorted_x, distorted_y);
                values.push(attribute.get(index));
            }
        }

        values
    }

    // Runs the step.
    pub fn run(&self, map: &mut Map2d) {
        info!(
            "Distort attribute '{}' of map '{}' in 2 dimensions.",
            map.get_attribute(self.attribute_id).get_name(),
            map.get_name()
        );

        let values = self.distort_map(map);
        let attribute = map.get_attribute_mut(self.attribute_id);

        attribute.replace_all(values);
    }
}

/// For serializing, deserializing & validating [`Distortion2d`].
///
///```
///# use ofws_core::data::map::generation::attributes::distortion2d::{Distortion2d, Distortion2dData};
///# use ofws_core::data::math::generator::generator2d::Generator2dData::IndexGenerator;
///# use ofws_core::data::math::size2d::Size2d;
///# use std::convert::TryInto;
/// let generator_x = IndexGenerator(Size2d::new(1, 2));
/// let generator_y = IndexGenerator(Size2d::new(3, 4));
/// let data = Distortion2dData::new("test".to_string(), generator_x, generator_y);
/// let attributes = vec!["test".to_string()];
/// let step: Distortion2d = data.clone().try_convert(&attributes).unwrap();
/// let result: Distortion2dData = step.convert(&attributes);
/// assert_eq!(data, result)
///```
#[derive(new, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Distortion2dData {
    attribute: String,
    generator_x: Generator2dData,
    generator_y: Generator2dData,
}

impl Distortion2dData {
    pub fn try_convert(self, attributes: &[String]) -> Result<Distortion2d, GenerationStepError> {
        let id = get_attribute_id(&self.attribute, attributes)?;
        let generator_x: Generator2d = self.generator_x.try_into()?;
        let generator_y: Generator2d = self.generator_y.try_into()?;
        Ok(Distortion2d::new(id, generator_x, generator_y))
    }
}

impl Distortion2d {
    pub fn convert(&self, attributes: &[String]) -> Distortion2dData {
        let attribute = attributes[self.attribute_id].clone();
        Distortion2dData::new(
            attribute,
            (&self.generator_x).into(),
            (&self.generator_y).into(),
        )
    }
}
