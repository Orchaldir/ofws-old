use crate::data::map::generation::biome::{BiomeSelector, SetValueIfBelowThreshold};
use crate::data::map::generation::distortion::{Distortion1d, Distortion2d};
use crate::data::map::generation::generator::GeneratorStep;
use crate::data::map::generation::modify::ModifyWithAttribute;
use crate::data::map::Map2d;

pub mod biome;
pub mod distortion;
pub mod generator;
pub mod modify;

/// A trait to handle a step of the map generation.
pub enum GenerationStep {
    BiomeSelector(BiomeSelector),
    DistortAlongX(Distortion1d),
    DistortAlongY(Distortion1d),
    Distortion2d(Distortion2d),
    GeneratorAdd(GeneratorStep),
    GeneratorSub(GeneratorStep),
    ModifyWithAttribute(ModifyWithAttribute),
    SetValueIfBelowThreshold(SetValueIfBelowThreshold),
}

impl GenerationStep {
    /// Runs the step.
    pub fn run(&self, map: &mut Map2d) {
        match self {
            GenerationStep::BiomeSelector(step) => step.run(map),
            GenerationStep::DistortAlongX(step) => step.distort_along_x(map),
            GenerationStep::DistortAlongY(step) => step.distort_along_y(map),
            GenerationStep::Distortion2d(step) => step.run(map),
            GenerationStep::GeneratorAdd(step) => step.add(map),
            GenerationStep::GeneratorSub(step) => step.sub(map),
            GenerationStep::ModifyWithAttribute(step) => step.run(map),
            GenerationStep::SetValueIfBelowThreshold(step) => step.run(map),
        }
    }
}
