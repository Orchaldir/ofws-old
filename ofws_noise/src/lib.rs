use noise::{NoiseFn, SuperSimplex};
use ofws_core::data::generator::Generator;

pub struct NoiseGenerator {
    algo: SuperSimplex,
    scale: f64,
    max_value: f64,
}

impl NoiseGenerator {
    pub fn new(scale: f64, max_value: u8) -> NoiseGenerator {
        NoiseGenerator {
            algo: SuperSimplex::new(),
            scale,
            max_value: max_value as f64 / 2.0,
        }
    }
}

impl Generator for NoiseGenerator {
    fn generate(&self, x: u32, y: u32) -> u8 {
        let x1 = x as f64 / self.scale;
        let x2 = y as f64 / self.scale;
        let positive_value = self.algo.get([x1, x2]) + 1.0;
        (positive_value * self.max_value) as u8
    }
}
