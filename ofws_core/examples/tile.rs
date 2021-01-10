extern crate ofws_rendering_glium;

use ofws_core::data::color::{BLACK, BLUE, GREEN, RED, YELLOW};
use ofws_core::interface::app::App;
use ofws_core::interface::input::{KeyCode, MouseButton};
use ofws_core::interface::rendering::{Initialization, Renderer, TextureId};
use ofws_core::interface::window::Window;
use ofws_core::rendering::tile::TileRenderer;
use ofws_rendering_glium::window::GliumWindow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct TileExample {
    texture_id: TextureId,
    index: usize,
}

impl App for TileExample {
    fn init(&mut self, initialization: &mut dyn Initialization) {
        self.texture_id = initialization.load_texture("ascii.png");
    }

    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLACK);

        let mut tile_renderer = renderer.get_tile_renderer(self.texture_id);

        tile_renderer.render_full(self.index, RED);
        tile_renderer.render_full(1, GREEN);
        tile_renderer.render_full(2, BLUE);
        tile_renderer.render_ascii(40, b'A', YELLOW);

        renderer.finish();
    }

    fn on_button_released(&mut self, button: MouseButton, index: usize) {
        println!("Released {:?} at {}", button, index);
        self.index = index;
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Example with tiles");
    let app = Rc::new(RefCell::new(TileExample::default()));

    window.run(app.clone());
}
