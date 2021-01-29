use crate::data::color::Color;
use crate::data::map::Map2d;
use crate::data::math::interpolation::Interpolator;
use crate::rendering::tile::FULL_TILE;

/// Renders a cell of a [`Map2d`].
pub trait CellRenderer {
    /// Returns the ascii code & color of the cell.
    fn get(&self, map: &Map2d, index: usize) -> (u8, Color);
}

/// Renders a cell of a [`Map2d`] based on a specific attribute.
pub struct AttributeRenderer {
    attribute_id: usize,
    interpolator: Box<dyn Interpolator<Color>>,
}

impl AttributeRenderer {
    pub fn new(
        attribute_id: usize,
        interpolator: Box<dyn Interpolator<Color>>,
    ) -> AttributeRenderer {
        AttributeRenderer {
            attribute_id,
            interpolator,
        }
    }
}

impl CellRenderer for AttributeRenderer {
    /// Returns the ascii code & color of the cell based on the attribute.
    fn get(&self, map: &Map2d, index: usize) -> (u8, Color) {
        let attribute = map.get_attribute(self.attribute_id);
        let value = attribute.get(index);
        let factor = value as f32 / 255.0;
        let color = self.interpolator.interpolate(factor);
        (FULL_TILE, color)
    }
}
