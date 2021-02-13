use crate::data::color::Color;
use crate::data::map::Map2d;
use crate::data::math::selector::Selector;
use crate::rendering::tile::FULL_TILE;
use serde::{Deserialize, Serialize};

/// Renders a cell of a [`Map2d`].
#[derive(Debug, Serialize, Deserialize)]
pub enum CellRenderer {
    /// Renders a cell of a [`Map2d`] based on a specific attribute & a selector.
    AttributeRenderer {
        attribute_id: usize,
        color_selector: Selector<Color>,
    },
}

impl CellRenderer {
    pub fn new_attribute_renderer(
        attribute_id: usize,
        color_selector: Selector<Color>,
    ) -> CellRenderer {
        CellRenderer::AttributeRenderer {
            attribute_id,
            color_selector,
        }
    }
}

impl CellRenderer {
    /// Returns the ascii code & color of the cell for rendering.
    pub fn get(&self, map: &Map2d, index: usize) -> (u8, Color) {
        match self {
            CellRenderer::AttributeRenderer {
                attribute_id,
                color_selector,
            } => {
                let attribute = map.get_attribute(*attribute_id);
                let value = attribute.get(index);
                let color = color_selector.get(value);
                (FULL_TILE, color)
            }
        }
    }
}
