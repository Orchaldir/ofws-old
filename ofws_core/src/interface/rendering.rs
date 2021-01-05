use crate::data::color::Color;
use crate::data::size2d::Size2d;

pub type TextureId = usize;

/// A trait to load & init resources for rendering during initialization.
pub trait Initialization {
    /// Loads a texture from a file and returns a `TextureId` as a handle.
    ///
    /// # Panics
    ///
    /// Panics if the file does not exist.
    ///
    /// Panics if the file is not an image.
    ///
    /// Panics if it can not create a texture from the image.
    fn load_texture(&mut self, filename: &str) -> TextureId;
}

/// A trait to abstract away different rendering libraries and render targets.
pub trait Renderer {
    /// Returns the size of the render target in tiles.
    /// A tile is big enough to hold a single ascii character.
    fn get_size(&self) -> Size2d;

    /// Starts the rendering and fills the render target with the Color `color`.
    fn start(&mut self, color: Color);

    /// Finishes the rendering.
    fn finish(&mut self);

    /// Gets a renderer for colored polygons.
    fn get_color_renderer(&mut self) -> &mut dyn ColorRenderer;

    /// Gets a renderer for textured polygons.
    fn get_texture_renderer(&mut self, id: TextureId) -> &mut dyn TextureRenderer;
}

pub type Point = (f32, f32);

/// A trait that focuses on rendering colored polygons.
pub trait ColorRenderer {
    #[svgbobdoc::transform]
    /// Renders the triangle defined by the points a, b & c.
    ///
    /// The points must be in counter-clockwise order:
    /// ```svgbob
    ///    c
    ///    *
    ///   / \
    ///  /   \
    /// *-----*
    /// a     b
    /// ```
    fn render_triangle(&mut self, a: Point, b: Point, c: Point, color: Color);

    /// Renders an axis-aligned rectangle.
    fn render_rectangle(&mut self, position: Point, size: Point, color: Color);
}

pub type TextureCoordinate = (f32, f32);

/// A trait that focuses on rendering textured polygons.
pub trait TextureRenderer {
    /// Renders an axis-aligned textured rectangle.
    ///
    /// The parameters tc & tc_size define an axis-aligned rectangle inside the texture.
    fn render_rectangle(
        &mut self,
        position: Point,
        size: Point,
        tc: TextureCoordinate,
        tc_size: TextureCoordinate,
        color: Color,
    );
}
