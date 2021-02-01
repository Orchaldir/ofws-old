#[macro_use]
extern crate log;
extern crate ofws_rendering_glium;

use chrono::Local;
use env_logger::Builder;
use ofws_core::data::color::{Color, BLACK, BLUE, CYAN, GREEN, ORANGE, RED, WHITE, YELLOW};
use ofws_core::data::generator::gradient::absolute::AbsoluteGradientY;
use ofws_core::data::generator::gradient::circular::CircularGradient;
use ofws_core::data::map::generation::biome::BiomeSelector;
use ofws_core::data::map::generation::generator::AddGeneratorStep;
use ofws_core::data::map::generation::GenerationStep;
use ofws_core::data::map::Map2d;
use ofws_core::data::math::interpolation::vector::VectorInterpolator;
use ofws_core::data::math::interpolation::Interpolator;
use ofws_core::data::size2d::Size2d;
use ofws_core::interface::app::App;
use ofws_core::interface::input::KeyCode;
use ofws_core::interface::rendering::{Initialization, Renderer, TextureId};
use ofws_core::interface::window::Window;
use ofws_core::rendering::cell::{AttributeLookUp, AttributeRenderer, CellRenderer};
use ofws_noise::NoiseGenerator;
use ofws_rendering_glium::window::GliumWindow;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

pub struct AttributeExample {
    map: Map2d,
    attribute_renderer: Box<dyn CellRenderer>,
    texture_id: TextureId,
}

impl AttributeExample {
    pub fn new(size: Size2d) -> AttributeExample {
        let map = create_map(size);

        AttributeExample {
            map,
            attribute_renderer: create_elevation_renderer(),
            texture_id: 0,
        }
    }
}

fn create_map(size: Size2d) -> Map2d {
    info!("Start map creation with {:?}", size);

    let mut map = Map2d::with_name("attribute-map", size);

    create_attributes(&mut map);

    create_generation_steps(&map)
        .iter()
        .for_each(|step| step.execute(&mut map));

    info!("Finish map creation");

    map
}

fn create_attributes(map: &mut Map2d) {
    map.create_attribute("elevation", 0).unwrap();
    map.create_attribute("temperature", 0).unwrap();
    map.create_attribute("rainfall", 0).unwrap();
    map.create_attribute("biome", 0).unwrap();
}

fn create_generation_steps(map: &Map2d) -> Vec<Box<dyn GenerationStep>> {
    let elevation_id = map.get_attribute_id("elevation").unwrap();
    let temperature_id = map.get_attribute_id("temperature").unwrap();
    let rainfall_id = map.get_attribute_id("rainfall").unwrap();
    let biome_id = map.get_attribute_id("biome").unwrap();

    vec![
        create_mountain_step(map, elevation_id),
        create_noise_step(elevation_id),
        create_temperature_gradient(map, temperature_id),
        create_rainfall_gradient(rainfall_id),
        create_biome_selector(temperature_id, rainfall_id, biome_id),
    ]
}

fn create_mountain_step(map: &Map2d, elevation_id: usize) -> Box<dyn GenerationStep> {
    let half_x = map.get_size().width() / 2;
    let half_y = map.get_size().height() / 2;

    let mountain = Box::new(CircularGradient::new(125, 0, half_x, half_y, half_x / 2));
    Box::new(AddGeneratorStep::new("continent", elevation_id, mountain))
}

fn create_noise_step(elevation_id: usize) -> Box<dyn GenerationStep> {
    let noise = Box::new(NoiseGenerator::new(20.0, 125));
    Box::new(AddGeneratorStep::new("islands", elevation_id, noise))
}

fn create_temperature_gradient(map: &Map2d, temperature_id: usize) -> Box<dyn GenerationStep> {
    let half_y = map.get_size().height() / 2;
    let generator = Box::new(AbsoluteGradientY::new(255, 0, half_y, half_y));
    Box::new(AddGeneratorStep::new(
        "gradient y",
        temperature_id,
        generator,
    ))
}

fn create_rainfall_gradient(rainfall_id: usize) -> Box<dyn GenerationStep> {
    let generator = Box::new(NoiseGenerator::new(100.0, 255));
    Box::new(AddGeneratorStep::new("noise", rainfall_id, generator))
}

fn create_biome_selector(
    temperature_id: usize,
    rainfall_id: usize,
    biome_id: usize,
) -> Box<dyn GenerationStep> {
    Box::new(BiomeSelector::new(
        rainfall_id,
        temperature_id,
        biome_id,
        Size2d::new(3, 3),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
    ))
}

fn create_elevation_color_interpolator() -> Box<dyn Interpolator<Color>> {
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

fn create_temperature_color_interpolator() -> Box<dyn Interpolator<Color>> {
    let vector = vec![
        (0.0, WHITE),
        (0.2, CYAN),
        (0.4, BLUE),
        (0.6, GREEN),
        (0.8, YELLOW),
        (1.0, RED),
    ];

    Box::new(VectorInterpolator::new(vector).unwrap())
}

fn create_rainfall_color_interpolator() -> Box<dyn Interpolator<Color>> {
    let light_blue = Color::new(100, 200, 255);
    let light_golden_rod = Color::new(250, 250, 220);
    let golden_rod = Color::new(250, 200, 40);

    let vector = vec![
        (0.0, BLUE),
        (0.2, light_blue),
        (0.5, GREEN),
        (0.8, light_golden_rod),
        (1.0, golden_rod),
    ];

    Box::new(VectorInterpolator::new(vector).unwrap())
}

fn create_elevation_renderer() -> Box<AttributeRenderer> {
    Box::new(AttributeRenderer::new(
        0,
        create_elevation_color_interpolator(),
    ))
}

fn create_temperature_renderer() -> Box<AttributeRenderer> {
    Box::new(AttributeRenderer::new(
        1,
        create_temperature_color_interpolator(),
    ))
}

fn create_rainfall_renderer() -> Box<AttributeRenderer> {
    Box::new(AttributeRenderer::new(
        2,
        create_rainfall_color_interpolator(),
    ))
}

fn create_biome_renderer() -> Box<AttributeLookUp> {
    let light_green = Color::new(100, 255, 100);
    let dark_green = Color::new(0, 80, 0);

    let colors = vec![
        (0, WHITE),
        (1, WHITE),
        (2, WHITE),
        (3, YELLOW),      // temperate desert
        (4, light_green), // temperate grassland
        (5, dark_green),  // temperate forest
        (6, RED),         //desert
        (7, ORANGE),      // savanna
        (8, GREEN),       // rainforest
    ]
    .into_iter()
    .collect();

    Box::new(AttributeLookUp::new(3, colors))
}

impl App for AttributeExample {
    fn init(&mut self, initialization: &mut dyn Initialization) {
        self.texture_id = initialization.load_texture("ascii.png");
    }

    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLACK);

        let tiles = renderer.get_size().get_area();
        let mut tile_renderer = renderer.get_tile_renderer(self.texture_id);

        for index in 0..tiles {
            let (ascii, color) = self.attribute_renderer.get(&self.map, index);
            tile_renderer.render_ascii(index, ascii, color);
        }

        renderer.finish();
    }

    fn on_key_released(&mut self, key: KeyCode) {
        if key == KeyCode::Key1 {
            self.attribute_renderer = create_elevation_renderer();
        } else if key == KeyCode::Key2 {
            self.attribute_renderer = create_temperature_renderer();
        } else if key == KeyCode::Key3 {
            self.attribute_renderer = create_rainfall_renderer();
        } else if key == KeyCode::Key4 {
            self.attribute_renderer = create_biome_renderer();
        }
    }
}

fn main() {
    Builder::from_env("RUST_LOG")
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}]: {}",
                Local::now().format("%H:%M:%S.%3f"),
                record.level(),
                record.args()
            )
        })
        .init();

    let tiles = Size2d::new(400, 300);
    let mut window = GliumWindow::new("Example with map attributes", tiles, Size2d::new(2, 2));
    let app = Rc::new(RefCell::new(AttributeExample::new(tiles)));

    window.run(app.clone());
}
