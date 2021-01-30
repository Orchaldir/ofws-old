#[macro_use]
extern crate log;
extern crate ofws_rendering_glium;

use ofws_core::data::color::{Color, BLACK, CYAN, WHITE};
use ofws_core::data::generator::gradient::circular::CircularGradient;
use ofws_core::data::map::generation::generator::AddGeneratorStep;
use ofws_core::data::map::generation::GenerationStep;
use ofws_core::data::map::Map2d;
use ofws_core::data::math::interpolation::vector::VectorInterpolator;
use ofws_core::data::math::interpolation::Interpolator;
use ofws_core::data::size2d::Size2d;
use ofws_core::interface::app::App;
use ofws_core::interface::rendering::{Initialization, Renderer, TextureId};
use ofws_core::interface::window::Window;
use ofws_core::rendering::cell::{AttributeRenderer, CellRenderer};
use ofws_noise::NoiseGenerator;
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

        let mountain_step = AttributeExample::create_mountain_step(map, elevation_id);
        let noise_step = AttributeExample::create_noise_step(elevation_id);

        vec![noise_step, mountain_step]
    }

    fn create_mountain_step(map: &Map2d, elevation_id: usize) -> Box<dyn GenerationStep> {
        let half_x = map.get_size().width() / 2;
        let half_y = map.get_size().height() / 2;

        let mountain = Box::new(CircularGradient::new(125, 0, half_x, half_y, half_x / 2));
        Box::new(AddGeneratorStep::new(elevation_id, mountain))
    }

    fn create_noise_step(elevation_id: usize) -> Box<dyn GenerationStep> {
        let noise = Box::new(NoiseGenerator::new(20.0, 125));
        Box::new(AddGeneratorStep::new(elevation_id, noise))
    }

    fn create_elevation_color_interpolator(&self) -> Box<dyn Interpolator<Color>> {
        let dark_blue = Color::new(0, 0, 128);
        let light_green = Color::new(100, 255, 100);
        let dark_green = Color::new(0, 80, 0);
        let light_gray = Color::gray(200);
        let dark_gray = Color::gray(50);

        let vector = vec![
            (0.0, dark_blue),
            (0.3, CYAN),
            (0.31, light_green),
            (0.6, dark_green),
            (0.61, dark_gray),
            (0.8, light_gray),
            (0.95, WHITE),
        ];

        Box::new(VectorInterpolator::new(vector).unwrap())
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
        let interpolator = self.create_elevation_color_interpolator();
        let attribute_renderer = AttributeRenderer::new(self.attribute_id, interpolator);

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
