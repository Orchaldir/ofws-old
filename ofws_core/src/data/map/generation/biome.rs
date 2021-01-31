use crate::data::map::generation::GenerationStep;
use crate::data::map::Map2d;
use crate::data::size2d::Size2d;

/// Selects a biome for the target attribute based on 2 input attributes.
pub struct BiomeSelector {
    input_id0: usize,
    input_id1: usize,
    target_id: usize,
    table_size: Size2d,
    category_size: Size2d,
    biome_ids: Vec<u8>,
}

impl BiomeSelector {
    pub fn new(
        input_id0: usize,
        input_id1: usize,
        target_id: usize,
        size: Size2d,
        biome_ids: Vec<u8>,
    ) -> BiomeSelector {
        BiomeSelector {
            input_id0,
            input_id1,
            target_id,
            table_size: size,
            category_size: Size2d::new(256 / size.width(), 256 / size.height()),
            biome_ids,
        }
    }

    fn calculate_biome(&self, input0: u8, input1: u8) -> u8 {
        let x = input0 as u32 / self.category_size.width();
        let y = input1 as u32 / self.category_size.height();
        let index = self.table_size.to_index(x, y);
        *self.biome_ids.get(index).unwrap()
    }

    fn calculate_biomes(&self, map: &mut Map2d) -> Vec<u8> {
        let size = map.size;
        let input0 = map.get_attribute(self.input_id0);
        let input1 = map.get_attribute(self.input_id1);
        let mut biomes = Vec::with_capacity(size.get_area());

        for index in 0..size.get_area() {
            let value0 = input0.get(index);
            let value1 = input1.get(index);
            biomes.push(self.calculate_biome(value0, value1));
        }

        biomes
    }
}

impl GenerationStep for BiomeSelector {
    // Executes the step.
    ///
    /// ```
    ///# use ofws_core::data::generator::IndexGenerator;
    ///# use ofws_core::data::map::Map2d;
    ///# use ofws_core::data::map::generation::biome::BiomeSelector;
    ///# use ofws_core::data::map::generation::GenerationStep;
    ///# use ofws_core::data::size2d::Size2d;
    /// let size = Size2d::new(2, 2);
    /// let mut map = Map2d::new(size);
    /// map.create_attribute_from("input0", vec![0, 100, 200, 255]);
    /// map.create_attribute_from("input1", vec![0, 200, 100, 255]);
    /// map.create_attribute("target", 255);
    /// let step = BiomeSelector::new(0, 1, 2, size, vec![10, 20, 30, 40]);
    ///
    /// step.execute(&mut map);
    ///
    /// let attribute = map.get_attribute(2);
    /// assert_eq!(attribute.get(0), 10);
    /// assert_eq!(attribute.get(1), 30);
    /// assert_eq!(attribute.get(2), 20);
    /// assert_eq!(attribute.get(3), 40);
    /// ```
    fn execute(&self, map: &mut Map2d) {
        info!(
            "Set '{}' based on attributes '{}' & '{}' of map '{}'",
            map.get_attribute(self.target_id).get_name(),
            map.get_attribute(self.input_id0).get_name(),
            map.get_attribute(self.input_id1).get_name(),
            map.get_name()
        );

        let biomes = self.calculate_biomes(map);
        let target = map.get_attribute_mut(self.target_id);

        for (index, biome) in biomes.iter().enumerate() {
            *target.get_mut(index) = *biome;
        }
    }
}
