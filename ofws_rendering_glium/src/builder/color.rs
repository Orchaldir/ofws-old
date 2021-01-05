use crate::renderer::get_other_corners;
use crate::vertex::ColoredVertex;
use ofws_core::data::color::Color;
use ofws_core::interface::rendering::{ColorRenderer, Point};

#[derive(Default)]
pub struct ColorBuilder {
    pub vertices: Vec<ColoredVertex>,
}

impl ColorBuilder {
    fn add(&mut self, position: Point, color: Color) {
        self.vertices.push(ColoredVertex {
            position,
            color: color.into(),
        });
    }
}

impl ColorRenderer for ColorBuilder {
    fn render_triangle(&mut self, a: Point, b: Point, c: Point, color: Color) {
        self.add(a, color);
        self.add(b, color);
        self.add(c, color);
    }

    fn render_rectangle(&mut self, position: Point, size: Point, color: Color) {
        let [c10, c01, c11] = get_other_corners(position, size);

        self.render_triangle(position, c10, c11, color);
        self.render_triangle(position, c11, c01, color);
    }
}
