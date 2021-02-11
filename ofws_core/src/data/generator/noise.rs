use noise::{NoiseFn, Seedable, SuperSimplex};

pub struct Noise {
    algo: Box<SuperSimplex>,
    scale: f64,
    factor: f64,
}

impl Noise {
    pub fn new(seed: u32, scale: f64, max_value: u8) -> Noise {
        Noise {
            algo: Box::new(SuperSimplex::new().set_seed(seed)),
            scale,
            factor: max_value as f64 / 2.0,
        }
    }

    /// Generates noise for an input.
    pub fn generate1d(&self, input: u32) -> u8 {
        let input = input as f64 / self.scale;
        let positive_value = self.algo.get([input, 0.0]) + 1.0;
        (positive_value * self.factor) as u8
    }

    /// Generates noise for a 2d point (x,y).
    pub fn generate2d(&self, x: u32, y: u32) -> u8 {
        let x = x as f64 / self.scale;
        let y = y as f64 / self.scale;
        let positive_value = self.algo.get([x, y]) + 1.0;
        (positive_value * self.factor) as u8
    }
}
