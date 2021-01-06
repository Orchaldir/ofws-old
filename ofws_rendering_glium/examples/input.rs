extern crate glium;
extern crate ofws_rendering_glium;

use ofws_core::data::color::BLACK;
use ofws_core::interface::app::App;
use ofws_core::interface::input::{KeyCode, MouseButton};
use ofws_core::interface::rendering::Renderer;
use ofws_core::interface::window::Window;
use ofws_rendering_glium::window::GliumWindow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct InputExample;

impl App for InputExample {
    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLACK);
        renderer.finish();
    }

    fn on_key_released(&mut self, key: KeyCode) {
        println!("Released key {:?}", key);
    }

    fn on_button_released(&mut self, button: MouseButton, index: usize) {
        println!("Released button {:?} at index {}", button, index);
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Example with input");
    let app = Rc::new(RefCell::new(InputExample::default()));

    window.run(app.clone());
}
