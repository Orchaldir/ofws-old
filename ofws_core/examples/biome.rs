#[macro_use]
extern crate log;
extern crate ofws_rendering_glium;

use ofws_core::data::color::{Color, BLACK, BLUE, CYAN, GREEN, ORANGE, RED, WHITE, YELLOW};
use ofws_core::data::map::generation::io::read_map_generator;
use ofws_core::data::map::Map2d;
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
    path: String,
    map: Option<Map2d>,
    attribute_renderer: CellRenderer,
    texture_id: TextureId,
}

impl BiomeExample {
    pub fn new(path: String) -> BiomeExample {
        BiomeExample {
            path,
            map: None,
            attribute_renderer: create_elevation_renderer(),
            texture_id: 0,
        }
    }
}

fn create_map(path: &str) -> Option<Map2d> {
    match read_map_generator(path) {
        Ok(map_generation) => {
            info!("Loaded map generator from '{}'", path);
            Some(map_generation.generate())
        }
        Err(error) => {
            error!("Failed loading '{}' with {:?}", path, error);
            None
        }
    }
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
        self.map = create_map(&self.path);
    }

    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLACK);

        if let Some(map) = &self.map {
            let mut tile_renderer = renderer.get_tile_renderer(self.texture_id, Size2d::new(2, 2));
            let tiles = tile_renderer.get_tiles().get_area();

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
        } else if key == KeyCode::Space {
            self.map = create_map(&self.path);
        }
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Example with biomes");
    let app = Rc::new(RefCell::new(BiomeExample::new(
        "resources/map_generation/biome.yaml".to_string(),
    )));

    window.run(app.clone());
}
