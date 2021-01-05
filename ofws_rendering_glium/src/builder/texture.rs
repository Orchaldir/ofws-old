use crate::renderer::get_other_corners;
use crate::vertex::TexturedVertex;
use ofws_core::data::color::Color;
use ofws_core::interface::rendering::{Point, TextureCoordinate, TextureRenderer};

#[derive(Default)]
pub struct TextureBuilder {
    pub vertices: Vec<TexturedVertex>,
}

impl TextureBuilder {
    fn add_vertex(&mut self, position: Point, tc: TextureCoordinate, color: Color) {
        self.vertices.push(TexturedVertex {
            position,
            color: color.into(),
            tc,
        });
    }

    fn add_triangle(
        &mut self,
        a: Point,
        b: Point,
        c: Point,
        tc_a: TextureCoordinate,
        tc_b: TextureCoordinate,
        tc_c: TextureCoordinate,
        color: Color,
    ) {
        self.add_vertex(a, tc_a, color);
        self.add_vertex(b, tc_b, color);
        self.add_vertex(c, tc_c, color);
    }
}

impl TextureRenderer for TextureBuilder {
    fn render_rectangle(
        &mut self,
        position: Point,
        size: Point,
        tc: TextureCoordinate,
        tc_size: TextureCoordinate,
        color: Color,
    ) {
        let [c10, c01, c11] = get_other_corners(position, size);
        let [tc10, tc01, tc11] = get_other_corners(tc, tc_size);

        self.add_triangle(position, c10, c11, tc, tc10, tc11, color);
        self.add_triangle(position, c11, c01, tc, tc11, tc01, color);
    }
}
