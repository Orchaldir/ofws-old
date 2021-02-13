extern crate log;
extern crate ofws_rendering_glium;

use ofws_core::data::color::{Color, BLACK, BLUE, CYAN, GREEN, ORANGE, RED, WHITE, YELLOW};
use ofws_core::data::map::generation::attribute::CreateAttribute;
use ofws_core::data::map::generation::biome::{BiomeSelector, SetValueIfBelowThreshold};
use ofws_core::data::map::generation::distortion::Distortion1d;
use ofws_core::data::map::generation::generator::GeneratorStep;
use ofws_core::data::map::generation::modify::ModifyWithAttribute;
use ofws_core::data::map::generation::{GenerationStep, MapGeneration};
use ofws_core::data::map::Map2d;
use ofws_core::data::math::generator::generator1d::Generator1d;
use ofws_core::data::math::generator::generator2d::Generator2d;
use ofws_core::data::math::generator::gradient::Gradient;
use ofws_core::data::math::generator::noise::Noise;
use ofws_core::data::math::selector::Selector;
use ofws_core::data::math::size2d::Size2d;
use ofws_core::interface::app::App;
use ofws_core::interface::input::KeyCode;
use ofws_core::interface::rendering::{Initialization, Renderer, TextureId};
use ofws_core::interface::window::Window;
use ofws_core::rendering::cell::CellRenderer;
use ofws_rendering_glium::window::GliumWindow;
use std::cell::RefCell;
use std::rc::Rc;

const OCEAN_ID: u8 = 12;
const OCEAN_VALUE: u8 = 76;

pub struct BiomeExample {
    size: Size2d,
    map: Option<Map2d>,
    attribute_renderer: CellRenderer,
    texture_id: TextureId,
}

impl BiomeExample {
    pub fn new(size: Size2d) -> BiomeExample {
        BiomeExample {
            size,
            map: None,
            attribute_renderer: create_elevation_renderer(),
            texture_id: 0,
        }
    }
}

fn create_map(size: Size2d) -> Option<Map2d> {
    let map_generation = MapGeneration::new("biome example", size, create_generation_steps(size));

    Some(map_generation.generate())
}

fn create_generation_steps(size: Size2d) -> Vec<GenerationStep> {
    let elevation_id = 0;
    let temperature_id = 1;
    let rainfall_id = 2;
    let biome_id = 3;

    vec![
        create_attribute("elevation", 0),
        create_attribute("temperature", 0),
        create_attribute("rainfall", 0),
        create_attribute("biome", 0),
        add_continent(size, elevation_id),
        add_islands(elevation_id),
        create_temperature_gradient(size, temperature_id),
        distort_temperature(temperature_id),
        subtract_elevation_from_temperature(elevation_id, temperature_id),
        create_rainfall(rainfall_id),
        select_biome(temperature_id, rainfall_id, biome_id),
        overwrite_ocean(elevation_id, biome_id),
    ]
}

fn create_attribute<S: Into<String>>(name: S, default: u8) -> GenerationStep {
    GenerationStep::CreateAttribute(CreateAttribute::new(name, default))
}

fn add_continent(size: Size2d, elevation_id: usize) -> GenerationStep {
    let half_x = size.width() / 2;
    let half_y = size.height() / 2;

    let gradient = Gradient::new(125, 0, 0, half_x / 2);
    let gradient = Generator1d::Gradient1d(gradient);
    let mountain = Generator2d::new_apply_to_distance(gradient, half_x, half_y);
    let step = GeneratorStep::new("continent", elevation_id, mountain);
    GenerationStep::GeneratorAdd(step)
}

fn add_islands(elevation_id: usize) -> GenerationStep {
    let noise = Noise::new(0, 20.0, 0, 125).unwrap();
    let noise = Generator2d::Noise2d(noise);
    let step = GeneratorStep::new("islands", elevation_id, noise);
    GenerationStep::GeneratorAdd(step)
}

fn create_temperature_gradient(size: Size2d, temperature_id: usize) -> GenerationStep {
    let half_y = size.height() / 2;
    let gradient = Gradient::new(255, 0, half_y, half_y);
    let gradient = Generator1d::AbsoluteGradient1d(gradient);
    let generator = Generator2d::new_apply_to_y(gradient);
    let step = GeneratorStep::new("gradient y", temperature_id, generator);
    GenerationStep::GeneratorAdd(step)
}

fn distort_temperature(temperature_id: usize) -> GenerationStep {
    let noise = Noise::new(0, 60.0, 0, 20).unwrap();
    let noise = Generator1d::Noise1d(noise);
    let step = Distortion1d::new(temperature_id, noise);
    GenerationStep::DistortAlongY(step)
}

fn subtract_elevation_from_temperature(
    elevation_id: usize,
    temperature_id: usize,
) -> GenerationStep {
    let step = ModifyWithAttribute::new(elevation_id, temperature_id, -0.8, OCEAN_VALUE);
    GenerationStep::ModifyWithAttribute(step)
}

fn create_rainfall(rainfall_id: usize) -> GenerationStep {
    let noise = Noise::new(0, 100.0, 0, 255).unwrap();
    let noise = Generator2d::Noise2d(noise);
    let step = GeneratorStep::new("noise", rainfall_id, noise);
    GenerationStep::GeneratorAdd(step)
}

fn select_biome(temperature_id: usize, rainfall_id: usize, biome_id: usize) -> GenerationStep {
    let step = BiomeSelector::new(
        rainfall_id,
        temperature_id,
        biome_id,
        Size2d::new(3, 4),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    );
    GenerationStep::BiomeSelector(step)
}

fn overwrite_ocean(elevation_id: usize, biome_id: usize) -> GenerationStep {
    let step = SetValueIfBelowThreshold::new(elevation_id, biome_id, OCEAN_ID, OCEAN_VALUE);
    GenerationStep::SetValueIfBelowThreshold(step)
}

fn create_elevation_color_interpolator() -> Selector<Color> {
    let dark_blue = Color::new(0, 0, 128);
    let light_green = Color::new(100, 255, 100);
    let dark_green = Color::new(0, 80, 0);
    let light_gray = Color::gray(200);
    let dark_gray = Color::gray(50);

    let vector = vec![
        (0, dark_blue),
        (OCEAN_VALUE, CYAN),
        (OCEAN_VALUE + 1, light_green),
        (153, dark_green),
        (154, dark_gray),
        (204, light_gray),
        (242, WHITE),
    ];

    Selector::new_interpolate_vector(vector).unwrap()
}

fn create_temperature_color_interpolator() -> Selector<Color> {
    let vector = vec![
        (0u8, WHITE),
        (51, CYAN),
        (102, BLUE),
        (153, GREEN),
        (204, YELLOW),
        (255, RED),
    ];

    Selector::new_interpolate_vector(vector).unwrap()
}

fn create_rainfall_color_interpolator() -> Selector<Color> {
    let light_blue = Color::new(100, 200, 255);
    let light_golden_rod = Color::new(250, 250, 220);
    let golden_rod = Color::new(250, 200, 40);

    let vector = vec![
        (0, BLUE),
        (51, light_blue),
        (128, GREEN),
        (204, light_golden_rod),
        (255, golden_rod),
    ];

    Selector::new_interpolate_vector(vector).unwrap()
}

fn create_elevation_renderer() -> CellRenderer {
    CellRenderer::new_attribute_renderer(0, create_elevation_color_interpolator())
}

fn create_temperature_renderer() -> CellRenderer {
    CellRenderer::new_attribute_renderer(1, create_temperature_color_interpolator())
}

fn create_rainfall_renderer() -> CellRenderer {
    CellRenderer::new_attribute_renderer(2, create_rainfall_color_interpolator())
}

fn create_biome_renderer() -> CellRenderer {
    let light_green = Color::new(100, 255, 100);
    let dark_green = Color::new(0, 80, 0);
    let darker_green = Color::new(0, 40, 0);

    let colors = vec![
        (0, WHITE), // ice
        (1, WHITE),
        (2, WHITE),
        (3, darker_green), // cold forest
        (4, darker_green),
        (5, darker_green),
        (6, light_green), // temperate desert or grassland
        (7, dark_green),  // temperate forest
        (8, dark_green),  // temperate forest
        (9, YELLOW),      //desert
        (10, ORANGE),     // savanna
        (11, GREEN),      // rainforest
        (OCEAN_ID, BLUE), // ocean
    ]
    .into_iter()
    .collect();

    let selector = Selector::Lookup(colors);
    CellRenderer::new_attribute_renderer(3, selector)
}

impl App for BiomeExample {
    fn init(&mut self, initialization: &mut dyn Initialization) {
        self.texture_id = initialization.load_texture("ascii.png");
        self.map = create_map(self.size);
    }

    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLACK);

        if let Some(map) = &self.map {
            let tiles = renderer.get_size().get_area();
            let mut tile_renderer = renderer.get_tile_renderer(self.texture_id);

            for index in 0..tiles {
                let (ascii, color) = self.attribute_renderer.get(map, index);
                tile_renderer.render_ascii(index, ascii, color);
            }
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
    let tiles = Size2d::new(400, 300);
    let mut window = GliumWindow::new("Example with biomes", tiles, Size2d::new(2, 2));
    let app = Rc::new(RefCell::new(BiomeExample::new(tiles)));

    window.run(app.clone());
}
