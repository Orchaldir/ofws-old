use noise::{NoiseFn, SuperSimplex};
use ofws_core::data::generator::Generator;

pub struct NoiseGenerator {
    algo: SuperSimplex,
    scale: f64,
    factor: f64,
}

impl NoiseGenerator {
    pub fn new(scale: f64, max_value: u8) -> NoiseGenerator {
        NoiseGenerator {
            algo: SuperSimplex::new(),
            scale,
            factor: max_value as f64 / 2.0,
        }
    }
}

impl Generator for NoiseGenerator {
    fn generate(&self, x: u32, y: u32) -> u8 {
        let x = x as f64 / self.scale;
        let y = y as f64 / self.scale;
        let positive_value = self.algo.get([x, y]) + 1.0;
        (positive_value * self.factor) as u8
    }
}
