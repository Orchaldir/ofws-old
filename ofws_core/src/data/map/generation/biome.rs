use crate::data::map::Map2d;
use crate::data::size2d::Size2d;

/// Overwrite the target attribute with a specific value, if it is below a threshold.
pub struct SetValueIfBelowThreshold {
    source_id: usize,
    target_id: usize,
    value: u8,
    threshold: u8,
}

impl SetValueIfBelowThreshold {
    pub fn new(
        source_id: usize,
        target_id: usize,
        value: u8,
        threshold: u8,
    ) -> SetValueIfBelowThreshold {
        SetValueIfBelowThreshold {
            source_id,
            target_id,
            value,
            threshold,
        }
    }

    fn calculate_indices_to_overwrite(&self, map: &mut Map2d) -> Vec<usize> {
        let source_attribute = map.get_attribute(self.source_id);
        let mut indices = Vec::with_capacity(map.size.get_area());

        for index in 0..map.size.get_area() {
            if source_attribute.get(index) < self.threshold {
                indices.push(index);
            }
        }

        indices
    }

    // Runs the step.
    pub fn run(&self, map: &mut Map2d) {
        info!(
            "Overwrite '{}' with '{}' based on attribute '{}' of map '{}'",
            map.get_attribute(self.target_id).get_name(),
            self.value,
            map.get_attribute(self.source_id).get_name(),
            map.get_name()
        );

        let indices = self.calculate_indices_to_overwrite(map);
        let attribute = map.get_attribute_mut(self.target_id);

        attribute.replace_some(indices, self.value);
    }
}

/// Selects a biome for the target attribute based on 2 input attributes.
pub struct BiomeSelector {
    source_id0: usize,
    source_id1: usize,
    target_id: usize,
    lookup_table_size: Size2d,
    cell_size: Size2d,
    lookup_table: Vec<u8>,
}

fn convert_size(value: u32) -> u32 {
    (256.0 / value as f32).ceil() as u32
}

impl BiomeSelector {
    pub fn new(
        source_id0: usize,
        source_id1: usize,
        target_id: usize,
        size: Size2d,
        biome_ids: Vec<u8>,
    ) -> BiomeSelector {
        let category_width = convert_size(size.width());
        let category_height = convert_size(size.height());

        BiomeSelector {
            source_id0,
            source_id1,
            target_id,
            lookup_table_size: size,
            cell_size: Size2d::new(category_width, category_height),
            lookup_table: biome_ids,
        }
    }

    fn calculate_biome(&self, input0: u8, input1: u8) -> u8 {
        let x = input0 as u32 / self.cell_size.width();
        let y = input1 as u32 / self.cell_size.height();
        let index = self.lookup_table_size.to_index(x, y);

        *self.lookup_table.get(index).unwrap_or_else(|| {
            panic!(
                "Index {} is to large for {} biomes!",
                index,
                self.lookup_table.len()
            )
        })
    }

    fn calculate_biomes(&self, map: &mut Map2d) -> Vec<u8> {
        let size = map.size;
        let source_attribute0 = map.get_attribute(self.source_id0);
        let source_attribute1 = map.get_attribute(self.source_id1);
        let mut biomes = Vec::with_capacity(size.get_area());

        for index in 0..size.get_area() {
            let value0 = source_attribute0.get(index);
            let value1 = source_attribute1.get(index);
            biomes.push(self.calculate_biome(value0, value1));
        }

        biomes
    }

    // Runs the step.
    ///
    /// ```
    ///# use ofws_core::data::map::Map2d;
    ///# use ofws_core::data::map::generation::biome::BiomeSelector;
    ///# use ofws_core::data::map::generation::GenerationStep;
    ///# use ofws_core::data::size2d::Size2d;
    /// let size = Size2d::new(3, 2);
    /// let mut map = Map2d::new(size);
    /// map.create_attribute_from("input0", vec![0, 100, 200, 60, 170, 255]);
    /// map.create_attribute_from("input1", vec![0, 60, 100, 170, 200, 255]);
    /// map.create_attribute("target", 255);
    /// let step = BiomeSelector::new(0, 1, 2, size, vec![10, 20, 30, 40, 50, 60]);
    ///
    /// step.run(&mut map);
    ///
    /// let attribute = map.get_attribute(2);
    /// assert_eq!(attribute.get_all(), &vec![10u8, 20, 30, 40, 50, 60]);
    /// ```
    pub fn run(&self, map: &mut Map2d) {
        info!(
            "Set '{}' based on attributes '{}' & '{}' of map '{}'",
            map.get_attribute(self.target_id).get_name(),
            map.get_attribute(self.source_id0).get_name(),
            map.get_attribute(self.source_id1).get_name(),
            map.get_name()
        );

        let biomes = self.calculate_biomes(map);
        let attribute = map.get_attribute_mut(self.target_id);

        attribute.replace_all(biomes);
    }
}
