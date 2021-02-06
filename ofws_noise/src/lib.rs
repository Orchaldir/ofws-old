use noise::{NoiseFn, SuperSimplex};
use ofws_core::data::generator1d::Generator1d;
use ofws_core::data::generator2d::Generator2d;

pub struct NoiseGenerator1d {
    algo: SuperSimplex,
    scale: f64,
    factor: f64,
}

impl NoiseGenerator1d {
    pub fn new(scale: f64, max_value: u8) -> NoiseGenerator1d {
        NoiseGenerator1d {
            algo: SuperSimplex::new(),
            scale,
            factor: max_value as f64 / 2.0,
        }
    }
}

impl Generator1d for NoiseGenerator1d {
    fn generate(&self, input: u32) -> u8 {
        let input = input as f64 / self.scale;
        let positive_value = self.algo.get([input, 0.0]) + 1.0;
        (positive_value * self.factor) as u8
    }
}

pub struct NoiseGenerator2d {
    algo: SuperSimplex,
    scale: f64,
    factor: f64,
}

impl NoiseGenerator2d {
    pub fn new(scale: f64, max_value: u8) -> NoiseGenerator2d {
        NoiseGenerator2d {
            algo: SuperSimplex::new(),
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
