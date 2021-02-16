extern crate glium;
extern crate ofws_rendering_glium;

use ofws_core::data::color::{Color, BLUE, GREEN, RED, YELLOW};
use ofws_core::interface::app::App;
use ofws_core::interface::rendering::Renderer;
use ofws_core::interface::window::Window;
use ofws_rendering_glium::window::GliumWindow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct ColoredExample {}

impl App for ColoredExample {
    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLUE);

        let color_renderer = renderer.get_color_renderer();
        color_renderer.render_triangle((400.0, 300.0), (600.0, 300.0), (500.0, 400.0), GREEN);
        color_renderer.render_triangle((100.0, 300.0), (300.0, 300.0), (200.0, 400.0), RED);
        color_renderer.render_rectangle((300.0, 40.0), (140.0, 50.0), YELLOW);
        color_renderer.render_rectangle((300.0, 500.0), (140.0, 50.0), Color::new(50, 0, 0));

        renderer.finish();
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Example with colored Polygons");
    let app = Rc::new(RefCell::new(ColoredExample::default()));

    window.run(app.clone());
}
