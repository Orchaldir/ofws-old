use crate::data::map::Map2d;

pub mod generator;

/// A trait to handle a step of the map generation.
pub trait GenerationStep {
    // Executes the step.
    fn execute(&self, map: &mut Map2d);
}