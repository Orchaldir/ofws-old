extern crate glium;
extern crate ofws_rendering_glium;

use ofws_core::data::color::{BLUE, RED};
use ofws_core::interface::app::App;
use ofws_core::interface::rendering::{Initialization, Renderer, TextureId};
use ofws_core::interface::window::Window;
use ofws_rendering_glium::window::GliumWindow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct TextureExample {
    texture_id: TextureId,
}

impl App for TextureExample {
    fn init(&mut self, initialization: &mut dyn Initialization) {
        self.texture_id = initialization.load_texture("ascii.png");
    }

    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLUE);
        renderer
            .get_texture_renderer(self.texture_id)
            .render_rectangle((10.0, 5.0), (20.0, 20.0), (0.0, 0.0), (1.0, 1.0), RED);
        renderer.finish();
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Example with a texture");
    let app = Rc::new(RefCell::new(TextureExample::default()));

    window.run(app.clone());
}
