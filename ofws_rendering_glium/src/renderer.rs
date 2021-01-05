use crate::builder::color::ColorBuilder;
use crate::shader::load_program;
use cgmath::ortho;
use glium::{Program, Surface};
use ofws_core::data::color::Color;
use ofws_core::data::size2d::Size2d;
use ofws_core::interface::rendering::{ColorRenderer, Renderer};

const INDICES: glium::index::NoIndices =
    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

pub struct GliumRenderer {
    size: Size2d,
    display: glium::Display,
    target: Option<glium::Frame>,
    color_builder: ColorBuilder,
    colored_program: Program,
    matrix: cgmath::Matrix4<f32>,
}

impl GliumRenderer {
    pub fn new(display: glium::Display, size: Size2d) -> GliumRenderer {
        let colored_program = load_program(&display, "colored.vertex", "colored.fragment");

        let matrix: cgmath::Matrix4<f32> = ortho(
            0.0,
            size.width() as f32,
            0.0,
            size.height() as f32,
            -1.0,
            1.0,
        );

        GliumRenderer {
            size,
            display,
            target: None,
            color_builder: ColorBuilder::default(),
            colored_program,
            matrix,
        }
    }

    fn render_colored_triangles(&mut self) {
        let target = self.target.as_mut().unwrap();
        let vertex_buffer =
            glium::VertexBuffer::new(&self.display, &self.color_builder.vertices).unwrap();

        let uniforms = uniform! {
            matrix: Into::<[[f32; 4]; 4]>::into(self.matrix)
        };

        target
            .draw(
                &vertex_buffer,
                &INDICES,
                &self.colored_program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}

impl Renderer for GliumRenderer {
    fn get_size(&self) -> Size2d {
        self.size
    }

    fn start(&mut self, color: Color) {
        let mut target = self.display.draw();
        target.clear_color(
            color.r() as f32 / 255.0,
            color.g() as f32 / 255.0,
            color.b() as f32 / 255.0,
            1.0,
        );
        self.target = Some(target);

        self.color_builder.vertices.clear();
    }

    fn finish(&mut self) {
        self.render_colored_triangles();

        if let Some(target) = self.target.take() {
            target.finish().unwrap();
        }
    }

    fn get_color_renderer(&mut self) -> &mut dyn ColorRenderer {
        &mut self.color_builder
    }
}

pub fn get_other_corners(position: (f32, f32), size: (f32, f32)) -> [(f32, f32); 3] {
    let corner10 = (position.0 + size.0, position.1);
    let corner01 = (position.0, position.1 + size.1);
    let corner11 = (position.0 + size.0, position.1 + size.1);

    [corner10, corner01, corner11]
}