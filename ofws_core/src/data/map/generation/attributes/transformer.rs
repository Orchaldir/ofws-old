use crate::data::map::generation::step::{get_attribute_id, GenerationStepError};
use crate::data::map::Map2d;
use crate::data::math::transformer::transformer2d::{Transformer2d, Transformer2dData};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

/// Transforms 2 [`Attribute`]s and writes into another.
#[derive(new)]
pub struct TransformAttribute2d {
    name: String,
    source_id0: usize,
    source_id1: usize,
    target_id: usize,
    transformer: Transformer2d,
}

impl TransformAttribute2d {
    fn transform(&self, map: &mut Map2d) -> Vec<u8> {
        let size = map.size;
        let source_attribute0 = map.get_attribute(self.source_id0);
        let source_attribute1 = map.get_attribute(self.source_id1);
        let mut biomes = Vec::with_capacity(size.get_area());

        for index in 0..size.get_area() {
            let value0 = source_attribute0.get(index);
            let value1 = source_attribute1.get(index);
            biomes.push(self.transformer.transform(value0, value1));
        }

        biomes
    }

    // Runs the step.
    ///
    /// ```
    ///# use ofws_core::data::map::Map2d;
    ///# use ofws_core::data::map::generation::attributes::transformer::TransformAttribute2d;
    ///# use ofws_core::data::math::size2d::Size2d;
    ///# use ofws_core::data::math::transformer::transformer2d::Transformer2d;
    /// let mut map = Map2d::new(Size2d::new(3, 2));
    /// map.create_attribute_from("input0", vec![  0,   1,  99, 100, 101, 255]);
    /// map.create_attribute_from("input1", vec![200, 199, 198, 197, 196, 195]);
    /// map.create_attribute("target", 10);
    /// let transformer = Transformer2d::new_overwrite_if_below(42, 100);
    /// let step = TransformAttribute2d::new("name".to_string(), 0, 1, 2, transformer);
    ///
    /// step.run(&mut map);
    ///
    /// assert_eq!(map.get_attribute(0).get_all(), &vec![  0,   1,  99, 100, 101, 255]);
    /// assert_eq!(map.get_attribute(1).get_all(), &vec![200, 199, 198, 197, 196, 195]);
    /// assert_eq!(map.get_attribute(2).get_all(), &vec![ 42,  42,  42,  42, 196, 195]);
    /// ```
    pub fn run(&self, map: &mut Map2d) {
        info!(
            "Apply transformation '{}' using '{}' & '{}' to '{}' of map '{}'",
            self.name,
            map.get_attribute(self.source_id0).get_name(),
            map.get_attribute(self.source_id1).get_name(),
            map.get_attribute(self.target_id).get_name(),
            map.get_name()
        );

        let biomes = self.transform(map);
        let attribute = map.get_attribute_mut(self.target_id);

        attribute.replace_all(biomes);
    }
}

/// For serializing, deserializing & validating [`TransformAttribute2d`].
///
///```
///# use ofws_core::data::map::generation::attributes::transformer::{TransformAttribute2dData, TransformAttribute2d};
///# use ofws_core::data::math::transformer::transformer2d::Transformer2dData;
/// let transformer = Transformer2dData::Const(99);
/// let data = TransformAttribute2dData::new("name".to_string(), "s0".to_string(), "s1".to_string(), "t".to_string(), transformer);
/// let attributes = vec!["s0".to_string(), "s1".to_string(), "t".to_string()];
/// let step: TransformAttribute2d = data.clone().try_convert(&attributes).unwrap();
/// let result: TransformAttribute2dData = step.convert(&attributes);
/// assert_eq!(data, result)
///```
#[derive(new, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TransformAttribute2dData {
    name: String,
    source0: String,
    source1: String,
    target: String,
    transformer: Transformer2dData,
}

impl TransformAttribute2dData {
    pub fn try_convert(
        self,
        attributes: &[String],
    ) -> Result<TransformAttribute2d, GenerationStepError> {
        let source_id0 = get_attribute_id(&self.source0, attributes)?;
        let source_id1 = get_attribute_id(&self.source1, attributes)?;
        let target_id = get_attribute_id(&self.target, attributes)?;
        let transformer: Transformer2d = self.transformer.try_into()?;

        Ok(TransformAttribute2d::new(
            self.name,
            source_id0,
            source_id1,
            target_id,
            transformer,
        ))
    }
}

impl TransformAttribute2d {
    pub fn convert(&self, attributes: &[String]) -> TransformAttribute2dData {
        let source0 = attributes[self.source_id0].clone();
        let source1 = attributes[self.source_id1].clone();
        let target = attributes[self.target_id].clone();
        TransformAttribute2dData::new(
            self.name.clone(),
            source0,
            source1,
            target,
            (&self.transformer).into(),
        )
    }
}
