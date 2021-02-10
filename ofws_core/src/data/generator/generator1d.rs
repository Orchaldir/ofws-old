use noise::{NoiseFn, Seedable, SuperSimplex};

/// Generates values for a 1d input.
pub enum Generator1d {
    /// Returns the input as output.
    InputAsOutput,
    /// Generates values with Super Simplex noise.
    Noise1d {
        algo: SuperSimplex,
        seed: u32,
        scale: f64,
        factor: f64,
    },
}

impl Generator1d {
    pub fn new_noise(seed: u32, scale: f64, max_value: u8) -> Generator1d {
        Generator1d::Noise1d {
            algo: SuperSimplex::new().set_seed(seed),
            seed,
            scale,
            factor: max_value as f64 / 2.0,
        }
    }

    /// Generates an output for an input.
    pub fn generate(&self, input: u32) -> u8 {
        match self {
            Generator1d::InputAsOutput => input as u8,
            Generator1d::Noise1d {
                algo,
                seed: _,
                scale,
                factor,
            } => {
                let input = input as f64 / scale;
                let positive_value = algo.get([input, 0.0]) + 1.0;
                (positive_value * factor) as u8
            }
        }
    }
}
