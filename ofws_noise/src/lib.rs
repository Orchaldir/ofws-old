use noise::{NoiseFn, Seedable, SuperSimplex};
use ofws_core::data::generator2d::Generator2d;

pub struct NoiseGenerator2d {
    algo: SuperSimplex,
    scale: f64,
    factor: f64,
}

impl NoiseGenerator2d {
    pub fn new(seed: u32, scale: f64, max_value: u8) -> NoiseGenerator2d {
        NoiseGenerator2d {
            algo: SuperSimplex::new().set_seed(seed),
            scale,
            factor: max_value as f64 / 2.0,
        }
    }
}

impl Generator2d for NoiseGenerator2d {
    fn generate(&self, x: u32, y: u32) -> u8 {
        let x = x as f64 / self.scale;
        let y = y as f64 / self.scale;
        let positive_value = self.algo.get([x, y]) + 1.0;
        (positive_value * self.factor) as u8
    }
}
