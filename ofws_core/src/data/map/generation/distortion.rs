use crate::data::generator::generator1d::Generator1d;
use crate::data::generator::generator2d::Generator2d;
use crate::data::map::attribute::Attribute;
use crate::data::map::Map2d;

/// Shifts each column or row of an [`Attribute`] along the axis based on a [`Generator1d`].
pub struct Distortion1d {
    attribute_id: usize,
    generator: Generator1d,
}

impl Distortion1d {
    pub fn new(attribute_id: usize, generator: Generator1d) -> Distortion1d {
        Distortion1d {
            attribute_id,
            generator,
        }
    }

    fn distort_row(&self, y: u32, shift: u8, attribute: &Attribute, values: &mut Vec<u8>) {
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

    fn distort_map_along_x(&self, map: &Map2d) -> Vec<u8> {
        let length = map.size.get_area();
        let attribute = map.get_attribute(self.attribute_id);
        let mut values = Vec::with_capacity(length);

        for y in 0..map.size.height() {
            let shift = self.generator.generate(y);
            debug!("y={} shift={}", y, shift);
            self.distort_row(y, shift, attribute, &mut values);
        }

        values
    }

    // Runs the step.
    ///
    /// ```
    ///# use ofws_core::data::generator::generator1d::Generator1d::InputAsOutput;
    ///# use ofws_core::data::map::Map2d;
    ///# use ofws_core::data::map::generation::distortion::Distortion1d;
    ///# use ofws_core::data::size2d::Size2d;
    /// let size = Size2d::new(3, 3);
    /// let mut map = Map2d::new(size);
    /// let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let attribute_id = map.create_attribute_from("test", values).unwrap();
    /// let step = Distortion1d::new(attribute_id, InputAsOutput);
    ///
    /// step.distort_along_x(&mut map);
    ///
    /// let attribute = map.get_attribute(attribute_id);
    /// assert_eq!(attribute.get_all(), &vec![1u8, 2, 3, 4, 4, 5, 7, 7, 7]);
    /// ```
    pub fn distort_along_x(&self, map: &mut Map2d) {
        info!(
            "Distort attribute '{}' of map '{}' along the x-axis.",
            map.get_attribute(self.attribute_id).get_name(),
            map.get_name()
        );

        let values = self.distort_map_along_x(map);
        let attribute = map.get_attribute_mut(self.attribute_id);

        attribute.replace_values(values);
    }

    fn distort_column(&self, x: u32, shift: u8, attribute: &Attribute, values: &mut Vec<u8>) {
        let start = attribute.get_size().to_index(x, 0);
        let start_value = attribute.get(start);
        let mut index = start;
        let width = attribute.get_size().width() as usize;

        for _y in 0..shift {
            values[index] = start_value;
            index += width;
        }

        let remaining_height = attribute.get_size().height().saturating_sub(shift as u32);
        let mut distorted_index = start;

        for _y in 0..remaining_height {
            values[index] = attribute.get(distorted_index);
            index += width;
            distorted_index += width;
        }
    }

    fn distort_map_along_y(&self, map: &Map2d) -> Vec<u8> {
        let length = map.size.get_area();
        let attribute = map.get_attribute(self.attribute_id);
        let mut values = vec![0; length];

        for x in 0..map.size.width() {
            let shift = self.generator.generate(x);
            debug!("x={} shift={}", x, shift);
            self.distort_column(x, shift, attribute, &mut values);
        }

        values
    }

    // Runs the step.
    ///
    /// ```
    ///# use ofws_core::data::generator::generator1d::Generator1d::InputAsOutput;
    ///# use ofws_core::data::map::Map2d;
    ///# use ofws_core::data::map::generation::distortion::Distortion1d;
    ///# use ofws_core::data::size2d::Size2d;
    /// let size = Size2d::new(3, 3);
    /// let mut map = Map2d::new(size);
    /// let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let attribute_id = map.create_attribute_from("test", values).unwrap();
    /// let step = Distortion1d::new(attribute_id, InputAsOutput);
    ///
    /// step.distort_along_y(&mut map);
    ///
    /// let attribute = map.get_attribute(attribute_id);
    /// assert_eq!(attribute.get_all(), &vec![1u8, 2, 3, 4, 2, 3, 7, 5, 3]);
    /// ```
    pub fn distort_along_y(&self, map: &mut Map2d) {
        info!(
            "Distort attribute '{}' of map '{}' along the y-axis.",
            map.get_attribute(self.attribute_id).get_name(),
            map.get_name()
        );

        let values = self.distort_map_along_y(map);
        let attribute = map.get_attribute_mut(self.attribute_id);

        attribute.replace_values(values);
    }
}

/// Distorts an [`Attribute`] along 2 dimensions.
pub struct Distortion2d {
    attribute_id: usize,
    generator_x: Generator2d,
    generator_y: Generator2d,
}

impl Distortion2d {
    pub fn new(
        attribute_id: usize,
        generator_x: Generator2d,
        generator_y: Generator2d,
    ) -> Distortion2d {
        Distortion2d {
            attribute_id,
            generator_x,
            generator_y,
        }
    }

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

        attribute.replace_values(values);
    }
}
