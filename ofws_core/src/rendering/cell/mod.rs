use crate::data::color::{Color, PINK};
use crate::data::map::Map2d;
use crate::data::math::interpolation::Interpolator;
use crate::rendering::tile::FULL_TILE;
use std::collections::HashMap;

/// Renders a cell of a [`Map2d`].
pub trait CellRenderer {
    /// Returns the ascii code & color of the cell.
    fn get(&self, map: &Map2d, index: usize) -> (u8, Color);
}

/// Renders a cell of a [`Map2d`] based on a specific attribute.
pub struct AttributeRenderer<T: Interpolator<Color>> {
    attribute_id: usize,
    interpolator: T,
}

impl<T: Interpolator<Color>> AttributeRenderer<T> {
    pub fn new(attribute_id: usize, interpolator: T) -> AttributeRenderer<T> {
        AttributeRenderer {
            attribute_id,
            interpolator,
        }
    }
}

impl<T: Interpolator<Color>> CellRenderer for AttributeRenderer<T> {
    /// Returns the ascii code & color of the cell based on the attribute.
    fn get(&self, map: &Map2d, index: usize) -> (u8, Color) {
        let attribute = map.get_attribute(self.attribute_id);
        let value = attribute.get(index);
        let factor = value as f32 / 255.0;
        let color = self.interpolator.interpolate(factor);
        (FULL_TILE, color)
    }
}

/// Renders a cell of a [`Map2d`] based on a specific attribute.
pub struct AttributeLookUp {
    attribute_id: usize,
    colors: HashMap<u8, Color>,
}

impl AttributeLookUp {
    pub fn new(attribute_id: usize, colors: HashMap<u8, Color>) -> AttributeLookUp {
        AttributeLookUp {
            attribute_id,
            colors,
        }
    }
}

impl CellRenderer for AttributeLookUp {
    /// Returns the ascii code & color of the cell based on the attribute lookup.
    fn get(&self, map: &Map2d, index: usize) -> (u8, Color) {
        let attribute = map.get_attribute(self.attribute_id);
        let value = attribute.get(index);
        let color = self.colors.get(&value).unwrap_or(&PINK);
        (FULL_TILE, *color)
    }
}
