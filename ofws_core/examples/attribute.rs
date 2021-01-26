extern crate ofws_rendering_glium;

use ofws_core::data::color::BLACK;
use ofws_core::data::generator::gradient::circular::CircularGradient;
use ofws_core::data::map::generation::generator::AddGeneratorStep;
use ofws_core::data::map::generation::GenerationStep;
use ofws_core::data::map::Map2d;
use ofws_core::data::size2d::Size2d;
use ofws_core::interface::app::App;
use ofws_core::interface::rendering::Renderer;
use ofws_core::interface::window::Window;
use ofws_rendering_glium::window::GliumWindow;
use std::cell::RefCell;
use std::rc::Rc;

pub struct AttributeExample {
    map: Map2d,
    attribute_id: usize,
}

impl AttributeExample {
    pub fn new(size: Size2d) -> AttributeExample {
        let mut map = Map2d::new(size);
        let attribute_id = map.create_attribute("elevation", 0).unwrap();
        let gradient = CircularGradient::new(255, 0, 20, 15, 20);
        let generator = Box::new(gradient);
        let step = AddGeneratorStep::new(attribute_id, generator);

        step.execute(&mut map);

        AttributeExample { map, attribute_id }
    }
}

impl App for AttributeExample {
    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLACK);

        renderer.finish();
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Example with map attributes");
    let app = Rc::new(RefCell::new(AttributeExample::new(Size2d::new(40, 30))));

    window.run(app.clone());
}
