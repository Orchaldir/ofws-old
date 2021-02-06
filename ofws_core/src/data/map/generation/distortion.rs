use crate::data::generator1d::Generator1d;
use crate::data::map::attribute::Attribute;
use crate::data::map::generation::GenerationStep;
use crate::data::map::Map2d;

/// Shifts each row of an [`Attribute`] along the x-axis based on a [`Generator1d`].
pub struct DistortAlongX {
    attribute_id: usize,
    generator: Box<dyn Generator1d>,
}

impl DistortAlongX {
    pub fn new(attribute_id: usize, generator: Box<dyn Generator1d>) -> DistortAlongX {
        DistortAlongX {
            attribute_id,
            generator,
        }
    }

    fn distort_column(&self, y: u32, shift: u8, attribute: &Attribute, values: &mut Vec<u8>) {
        let start = attribute.get_size().to_index(0, y);
        let start_value = attribute.get(start);

        for _x in 0..shift {
            values.push(start_value);
        }

        let width = attribute.get_size().width().saturating_sub(shift as u32) as usize;

        for x in 0..width {
            values.push(attribute.get(start + x));
        }
    }

    fn distort_map(&self, map: &mut Map2d) -> Vec<u8> {
        let length = map.size.get_area();
        let attribute = map.get_attribute(self.attribute_id);
        let mut values = Vec::with_capacity(length);

        for y in 0..map.size.height() {
            let shift = self.generator.generate(y);
            info!("y={} shift={}", y, shift);
            self.distort_column(y, shift, attribute, &mut values);
        }

        values
    }
}

impl GenerationStep for DistortAlongX {
    // Executes the step.
    ///
    /// ```
    ///# use ofws_core::data::generator1d::InputToOutput;
    ///# use ofws_core::data::map::Map2d;
    ///# use ofws_core::data::map::generation::distortion::DistortAlongX;
    ///# use ofws_core::data::map::generation::GenerationStep;
    ///# use ofws_core::data::size2d::Size2d;
    /// let size = Size2d::new(3, 3);
    /// let mut map = Map2d::new(size);
    /// let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let attribute_id = map.create_attribute_from("test", values).unwrap();
    /// let generator = Box::new(InputToOutput);
    /// let step = DistortAlongX::new(attribute_id, generator);
    ///
    /// step.execute(&mut map);
    ///
    /// let attribute = map.get_attribute(attribute_id);
    /// assert_eq!(attribute.get_all(), &vec![1u8, 2, 3, 4, 4, 5, 7, 7, 7]);
    /// ```
    fn execute(&self, map: &mut Map2d) {
        info!(
            "Distort attribute '{}' of map '{}' along the x-axis.",
            map.get_attribute(self.attribute_id).get_name(),
            map.get_name()
        );

        let values = self.distort_map(map);
        let attribute = map.get_attribute_mut(self.attribute_id);

        attribute.replace_values(values);
    }
}
