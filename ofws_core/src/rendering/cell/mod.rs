use crate::data::color::{Color, BLACK};
use crate::data::map::Map2d;
use crate::data::math::selector::Selector;
use crate::rendering::tile::EMPTY_TILE;
use serde::{Deserialize, Serialize};

/// Renders a cell of a [`Map2d`].
#[derive(Debug, Serialize, Deserialize)]
pub enum CellRenderer {
    /// Renders a cell of a [`Map2d`] based on a specific attribute & a selector.
    AttributeRenderer {
        attribute_id: usize,
        color_selector: Selector<Color>,
        tile_selector: Selector<u8>,
    },
}

impl CellRenderer {
    pub fn new_attribute_renderer(
        attribute_id: usize,
        color_selector: Selector<Color>,
        tile_selector: Selector<u8>,
    ) -> CellRenderer {
        CellRenderer::AttributeRenderer {
            attribute_id,
            color_selector,
            tile_selector,
        }
    }

    pub fn new_color_renderer(
        attribute_id: usize,
        color_selector: Selector<Color>,
    ) -> CellRenderer {
        CellRenderer::AttributeRenderer {
            attribute_id,
            color_selector,
            tile_selector: Selector::Const(EMPTY_TILE),
        }
    }
}

impl CellRenderer {
    /// Returns the ascii code & color of the cell for rendering.
    pub fn get(&self, map: &Map2d, index: usize) -> (u8, Color, Color) {
        match self {
            CellRenderer::AttributeRenderer {
                attribute_id,
                color_selector,
                tile_selector,
            } => {
                let attribute = map.get_attribute(*attribute_id);
                let value = attribute.get(index);
                let background_color = color_selector.get(value);
                let tile = tile_selector.get(value);
                let tile_color = BLACK;
                (tile, tile_color, background_color)
            }
        }
    }
}
