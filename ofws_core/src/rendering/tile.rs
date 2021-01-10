use crate::data::color::Color;
use crate::data::size2d::Size2d;
use crate::interface::rendering::AsciiRenderer;

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

    /// Renders the tile at `index` with a uniform color.
    pub fn render_full(&mut self, index: usize, color: Color) {
        self.render_ascii(index, 219, color);
    }

    /// Renders the tile at `index` as an ascii character.
    pub fn render_ascii(&mut self, index: usize, ascii: u8, color: Color) {
        let point = self.size.to_x_and_y(index);
        let point = (point[0] as f32, point[1] as f32);
        self.renderer.render_u8(point, SIZE, ascii, color);
    }
}
