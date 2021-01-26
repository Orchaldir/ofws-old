extern crate glium;
extern crate ofws_rendering_glium;

use ofws_core::data::color::{BLUE, GREEN, RED, YELLOW, Color};
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
        color_renderer.render_triangle((20.0, 15.0), (30.0, 15.0), (25.0, 20.0), GREEN);
        color_renderer.render_triangle((5.0, 15.0), (15.0, 15.0), (10.0, 20.0), RED);
        color_renderer.render_rectangle((15.0, 2.0), (7.0, 2.5), YELLOW);
        color_renderer.render_rectangle((15.0, 25.0), (7.0, 2.5), Color::new(50, 0, 0));

        renderer.finish();
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Example with colored Polygons");
    let app = Rc::new(RefCell::new(ColoredExample::default()));

    window.run(app.clone());
}
