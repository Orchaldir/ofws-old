use crate::data::map::Map2d;
use crate::data::math::generator::generator2d::Generator2d;

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
