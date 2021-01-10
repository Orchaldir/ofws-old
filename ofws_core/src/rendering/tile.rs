use crate::data::color::Color;
use crate::data::size2d::Size2d;
use crate::interface::rendering::{AsciiRenderer, Point};

pub const FULL_TILE: u8 = 219;
const SIZE: (f32, f32) = (1.0, 1.0);

/// Simplifies rendering by focusing on a grid of tiles
pub struct TileRenderer<'a> {
    size: Size2d,
    renderer: &'a mut dyn AsciiRenderer,
}

impl<'a> TileRenderer<'a> {
    /// Creates a new TileRenderer
    pub fn new(size: Size2d, renderer: &'a mut dyn AsciiRenderer) -> TileRenderer {
        TileRenderer { size, renderer }
    }

    /// Renders a whole string starting at `index`.
    pub fn render_text(&mut self, index: usize, string: &str, color: Color) {
        let point = self.calculate_point(index);
        self.renderer.render_text(point, SIZE, string, color);
    }

    /// Renders the tile at `index` as an ascii character.
    pub fn render_ascii(&mut self, index: usize, ascii: u8, color: Color) {
        let point = self.calculate_point(index);
        self.renderer.render_u8(point, SIZE, ascii, color);
    }

    fn calculate_point(&mut self, index: usize) -> Point {
        let point0 = self.size.to_x_and_y(index);
        (point0[0] as f32, point0[1] as f32)
    }
}
