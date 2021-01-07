extern crate glium;
extern crate ofws_rendering_glium;

use ofws_core::data::color::{BLUE, GREEN, RED, WHITE, YELLOW};
use ofws_core::interface::app::App;
use ofws_core::interface::input::KeyCode;
use ofws_core::interface::rendering::{Initialization, Renderer, TextureId};
use ofws_core::interface::window::Window;
use ofws_rendering_glium::window::GliumWindow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct AsciiExample {
    texture_id: TextureId,
    take_screenshot: bool,
}

impl App for AsciiExample {
    fn init(&mut self, initialization: &mut dyn Initialization) {
        self.texture_id = initialization.load_texture("ascii.png");
    }

    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLUE);

        let ascii_renderer = renderer.get_ascii_renderer(self.texture_id);
        ascii_renderer.render_u8((10.0, 10.0), (5.0, 5.0), b'a', RED);
        ascii_renderer.render_char((15.0, 10.0), (5.0, 5.0), 'b', GREEN);
        ascii_renderer.render_text((15.0, 25.0), (1.0, 1.0), "Test?", WHITE);
        ascii_renderer.render_text(
            (0.0, 2.5),
            (1.0, 1.0),
            "Non-Ascii Symbols are replaced with '🎉'!",
            YELLOW,
        );

        renderer.finish();

        if self.take_screenshot {
            println!("Take screenshot");
            renderer.take_screenshot("ascii.png");
            self.take_screenshot = false;
        }
    }

    fn on_key_released(&mut self, key: KeyCode) {
        if key == KeyCode::Snapshot {
            self.take_screenshot = true;
        }
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Example with ascii");
    let app = Rc::new(RefCell::new(AsciiExample::default()));

    window.run(app.clone());
}
