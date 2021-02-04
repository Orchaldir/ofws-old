use crate::data::map::generation::GenerationStep;
use crate::data::map::Map2d;

/// Modifies one [`Attribute`] with another transformed one.
pub struct ModifyWithAttribute {
    source_id: usize,
    target_id: usize,
    factor: f32,
    minimum: u8,
}

impl ModifyWithAttribute {
    pub fn new(
        source_id: usize,
        target_id: usize,
        factor: f32,
        minimum: u8,
    ) -> ModifyWithAttribute {
        ModifyWithAttribute {
            source_id,
            target_id,
            factor: factor * 255.0 / (255.0 - minimum as f32),
            minimum,
        }
    }

    fn calculate_value(&self, source: u8, target: u8) -> u8 {
        (target as f32 + (source.max(self.minimum) - self.minimum) as f32 * self.factor) as u8
    }

    fn calculate_values(&self, map: &mut Map2d) -> Vec<u8> {
        let length = map.size.get_area();
        let source_attribute = map.get_attribute(self.source_id);
        let target_attribute = map.get_attribute(self.target_id);
        let mut values = Vec::with_capacity(length);

        for index in 0..length {
            let source = source_attribute.get(index);
            let target = target_attribute.get(index);
            values.push(self.calculate_value(source, target));
        }

        values
    }
}

impl GenerationStep for ModifyWithAttribute {
    // Executes the step.
    fn execute(&self, map: &mut Map2d) {
        info!(
            "Modify attribute '{}' with attribute '{}' of map '{}'",
            map.get_attribute(self.target_id).get_name(),
            map.get_attribute(self.source_id).get_name(),
            map.get_name()
        );

        let values = self.calculate_values(map);
        let attribute = map.get_attribute_mut(self.target_id);

        attribute.replace_values(values);
    }
}
