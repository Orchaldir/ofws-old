use crate::data::map::generation::step::{get_attribute_id, GenerationStepError};
use crate::data::map::Map2d;
use crate::data::math::transformer::transformer2d::{Transformer2d, Transformer2dData};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

/// Transforms 2 [`Attribute`]s and writes into another.
#[derive(new)]
pub struct TransformAttribute2d {
    source_id0: usize,
    source_name0: String,
    source_id1: usize,
    source_name1: String,
    target_id: usize,
    target_name: String,
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
    /// let step = TransformAttribute2d::new(0, "input0".to_string(), 1, "input1".to_string(), 2, "target".to_string(), transformer);
    ///
    /// step.run(&mut map);
    ///
    /// assert_eq!(map.get_attribute(0).get_all(), &vec![  0,   1,  99, 100, 101, 255]);
    /// assert_eq!(map.get_attribute(1).get_all(), &vec![200, 199, 198, 197, 196, 195]);
    /// assert_eq!(map.get_attribute(2).get_all(), &vec![ 42,  42,  42,  42, 196, 195]);
    /// ```
    pub fn run(&self, map: &mut Map2d) {
        info!(
            "Transform attributes '{}' & '{}' and write into '{}' of map '{}'",
            self.source_name0,
            self.source_name1,
            self.target_name,
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
///# use std::convert::TryInto;
///# use ofws_core::data::map::generation::attributes::transformer::{TransformAttribute2dData, TransformAttribute2d};
///# use ofws_core::data::math::transformer::transformer2d::Transformer2dData;
/// let transformer = Transformer2dData::Const(99);
/// let data = TransformAttribute2dData::new("s0".to_string(), "s1".to_string(), "t".to_string(), transformer);
/// let step: TransformAttribute2d = data.clone().try_convert(&mut vec!["s0".to_string(), "s1".to_string(), "t".to_string()]).unwrap();
/// let result: TransformAttribute2dData = (&step).into();
/// assert_eq!(data, result)
///```
#[derive(new, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TransformAttribute2dData {
    source0: String,
    source1: String,
    target: String,
    transformer: Transformer2dData,
}

impl TransformAttribute2dData {
    pub fn try_convert(
        self,
        attributes: &mut Vec<String>,
    ) -> Result<TransformAttribute2d, GenerationStepError> {
        let source_id0 = get_attribute_id(&self.source0, attributes)?;
        let source_id1 = get_attribute_id(&self.source1, attributes)?;
        let target_id = get_attribute_id(&self.target, attributes)?;
        let transformer: Transformer2d = self.transformer.try_into()?;
        Ok(TransformAttribute2d::new(
            source_id0,
            self.source0,
            source_id1,
            self.source1,
            target_id,
            self.target,
            transformer,
        ))
    }
}

impl From<&TransformAttribute2d> for TransformAttribute2dData {
    fn from(step: &TransformAttribute2d) -> Self {
        TransformAttribute2dData::new(
            step.source_name0.clone(),
            step.source_name1.clone(),
            step.target_name.clone(),
            (&step.transformer).into(),
        )
    }
}
