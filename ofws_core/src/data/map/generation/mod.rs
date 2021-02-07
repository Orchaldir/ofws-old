use crate::data::map::Map2d;

pub mod biome;
pub mod distortion;
pub mod generator;
pub mod modify;

/// A trait to handle a step of the map generation.
pub trait GenerationStep {
    // Runs the step.
    fn run(&self, map: &mut Map2d);
}
