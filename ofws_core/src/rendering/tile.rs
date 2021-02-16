use crate::data::color::Color;
use crate::data::math::size2d::Size2d;
use crate::interface::rendering::{AsciiRenderer, Point};

pub const FULL_TILE: u8 = 219;

/// Simplifies rendering by focusing on a grid of tiles
pub struct TileRenderer<'a> {
    tiles: Size2d,
    tile_size: Point,
    renderer: &'a mut dyn AsciiRenderer,
}

impl<'a> TileRenderer<'a> {
    /// Creates a new TileRenderer
    pub fn new(
        tiles: Size2d,
        tile_size: Size2d,
        renderer: &'a mut dyn AsciiRenderer,
    ) -> TileRenderer {
        let tile_size = (tile_size.width() as f32, tile_size.height() as f32);
        TileRenderer {
            tiles,
            tile_size,
            renderer,
        }
    }

    pub fn get_tiles(&self) -> Size2d {
        self.tiles
    }

    /// Renders a whole string starting at `index`.
    pub fn render_text(&mut self, index: usize, string: &str, color: Color) {
        let point = self.calculate_point(index);
        self.renderer
            .render_text(point, self.tile_size, string, color);
    }

    /// Renders the tile at `index` as an ascii character.
    pub fn render_ascii(&mut self, index: usize, ascii: u8, color: Color) {
        let point = self.calculate_point(index);
        self.renderer.render_u8(point, self.tile_size, ascii, color);
    }

    fn calculate_point(&mut self, index: usize) -> Point {
        let point0 = self.tiles.to_x_and_y(index);
        (
            point0[0] as f32 * self.tile_size.0,
            point0[1] as f32 * self.tile_size.1,
        )
    }
}

/// Calculates how many tiles of size tile_size have space inside the the window.
///
/// ```
///# use ofws_core::data::math::size2d::Size2d;
///# use ofws_core::rendering::tile::calculate_tiles;
/// assert_eq!(calculate_tiles(Size2d::new(800, 600), Size2d::new(20, 30)), Size2d::new(40, 20));
/// ```
pub fn calculate_tiles(window_size: Size2d, tile_size: Size2d) -> Size2d {
    Size2d::new(
        (window_size.width() as f32 / tile_size.width() as f32).ceil() as u32,
        (window_size.height() as f32 / tile_size.height() as f32).ceil() as u32,
    )
}

/// Calculates how many tiles of size tile_size have space inside the the window.
///
/// ```
///# use ofws_core::data::math::size2d::Size2d;
///# use ofws_core::rendering::tile::calculate_tile_index;
/// let window_size = Size2d::new(800, 600);
/// let tile_size = Size2d::new(20, 30);
/// assert_eq!(calculate_tile_index(window_size, tile_size, (0,0)), 0);
/// assert_eq!(calculate_tile_index(window_size, tile_size, (799,0)), 39);
/// assert_eq!(calculate_tile_index(window_size, tile_size, (0,599)), 760);
/// assert_eq!(calculate_tile_index(window_size, tile_size, (799,599)), 799);
/// ```
pub fn calculate_tile_index(window_size: Size2d, tile_size: Size2d, point: (u32, u32)) -> usize {
    let tiles = calculate_tiles(window_size, tile_size);
    tiles.to_index_risky(point.0 / tile_size.width(), point.1 / tile_size.height())
}
