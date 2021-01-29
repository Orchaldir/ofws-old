#[macro_use]
extern crate log;
extern crate ofws_rendering_glium;

use ofws_core::data::color::{BLACK, GREEN};
use ofws_core::data::generator::gradient::circular::CircularGradient;
use ofws_core::data::map::generation::generator::AddGeneratorStep;
use ofws_core::data::map::generation::GenerationStep;
use ofws_core::data::map::Map2d;
use ofws_core::data::math::interpolation::pair::PairInterpolator;
use ofws_core::data::size2d::Size2d;
use ofws_core::interface::app::App;
use ofws_core::interface::rendering::{Initialization, Renderer, TextureId};
use ofws_core::interface::window::Window;
use ofws_core::rendering::cell::{AttributeRenderer, CellRenderer};
use ofws_rendering_glium::window::GliumWindow;
use std::cell::RefCell;
use std::rc::Rc;

pub struct AttributeExample {
    map: Map2d,
    attribute_id: usize,
    texture_id: TextureId,
}

impl AttributeExample {
    pub fn new(size: Size2d) -> AttributeExample {
        let map = AttributeExample::create_map(size);

        AttributeExample {
            map,
            attribute_id: 0,
            texture_id: 0,
        }
    }

    fn create_map(size: Size2d) -> Map2d {
        info!("Start map creation with {:?}", size);

        let mut map = Map2d::with_name("attribute-map", size);

        AttributeExample::create_attributes(&mut map);

        AttributeExample::create_generation_steps(&map)
            .iter()
            .for_each(|step| step.execute(&mut map));

        info!("Finish map creation");

        map
    }

    fn create_attributes(map: &mut Map2d) {
        map.create_attribute("elevation", 0).unwrap();
    }

    fn create_generation_steps(map: &Map2d) -> Vec<Box<dyn GenerationStep>> {
        let elevation_id = map.get_attribute_id("elevation").unwrap();
        let half_x = map.get_size().width() / 2;
        let half_y = map.get_size().height() / 2;

        let gradient = CircularGradient::new(255, 0, half_x, half_y, half_x);
        let generator = Box::new(gradient);
        let step = Box::new(AddGeneratorStep::new(elevation_id, generator));

        vec![step]
    }
}

impl App for AttributeExample {
    fn init(&mut self, initialization: &mut dyn Initialization) {
        self.texture_id = initialization.load_texture("ascii.png");
    }

    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLACK);

        let tiles = renderer.get_size().get_area();
        let mut tile_renderer = renderer.get_tile_renderer(self.texture_id);
        let interpolator = PairInterpolator::new(BLACK, GREEN);
        let attribute_renderer = AttributeRenderer::new(self.attribute_id, Box::new(interpolator));

        for index in 0..tiles {
            let (ascii, color) = attribute_renderer.get(&self.map, index);
            tile_renderer.render_ascii(index, ascii, color);
        }

        renderer.finish();
    }
}

fn main() {
    env_logger::init();

    let tiles = Size2d::new(400, 300);
    let mut window = GliumWindow::new("Example with map attributes", tiles, Size2d::new(2, 2));
    let app = Rc::new(RefCell::new(AttributeExample::new(tiles)));

    window.run(app.clone());
}
