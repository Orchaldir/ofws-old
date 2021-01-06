use crate::renderer::get_other_corners;
use crate::vertex::TexturedVertex;
use ofws_core::data::color::{Color, PINK};
use ofws_core::interface::rendering::{AsciiRenderer, Point, TextureCoordinate, TextureRenderer};

const INVALID_COLOR: Color = PINK;

pub struct TextureBuilder {
    rows_and_columns: u8,
    row_and_column_size: f32,
    tc_size: TextureCoordinate,
    pub vertices: Vec<TexturedVertex>,
}

impl TextureBuilder {
    pub fn new(rows_and_columns: u8) -> TextureBuilder {
        let row_and_column_size = 1.0 / rows_and_columns as f32;

        TextureBuilder {
            rows_and_columns,
            row_and_column_size,
            tc_size: (row_and_column_size, row_and_column_size),
            vertices: Vec::new(),
        }
    }

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

impl AsciiRenderer for TextureBuilder {
    fn render_text(&mut self, position: Point, size: Point, string: &str, color: Color) {
        let mut position = position;

        for character in string.chars() {
            self.render_char(position, size, character, color);
            position.0 += size.0;
        }
    }

    fn render_char(&mut self, position: Point, size: Point, character: char, color: Color) {
        if character.is_ascii() {
            self.render_u8(position, size, character as u8, color);
        } else {
            self.render_u8(position, size, b'?', INVALID_COLOR);
        }
    }

    fn render_u8(&mut self, position: Point, size: Point, ascii: u8, color: Color) {
        let row: u8 = ascii / self.rows_and_columns;
        let column: u8 = ascii % self.rows_and_columns;

        let tc = (
            column as f32 * self.row_and_column_size,
            1.0 - (row + 1) as f32 * self.row_and_column_size,
        );

        self.render_rectangle(position, size, tc, self.tc_size, color);
    }
}
