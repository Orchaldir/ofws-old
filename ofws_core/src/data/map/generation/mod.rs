use crate::data::map::generation::attribute::CreateAttribute;
use crate::data::map::generation::biome::{BiomeSelector, SetValueIfBelowThreshold};
use crate::data::map::generation::distortion::{Distortion1d, Distortion2d};
use crate::data::map::generation::generator::GeneratorStep;
use crate::data::map::generation::modify::ModifyWithAttribute;
use crate::data::map::Map2d;
use crate::data::size2d::Size2d;

pub mod attribute;
pub mod biome;
pub mod distortion;
pub mod generator;
pub mod modify;

/// Generates a map based on a number of steps.
pub struct MapGeneration {
    name: String,
    size: Size2d,
    steps: Vec<GenerationStep>,
}

impl MapGeneration {
    pub fn new<S: Into<String>>(
        name: S,
        size: Size2d,
        steps: Vec<GenerationStep>,
    ) -> MapGeneration {
        MapGeneration {
            name: name.into(),
            size,
            steps,
        }
    }

    /// Generates the map.
    pub fn generate(&self) -> Map2d {
        info!(
            "Generate the map '{}' with {:?} in {} steps:",
            self.name,
            self.size,
            self.steps.len(),
        );

        let mut map = Map2d::with_name(self.name.clone(), self.size);

        self.steps.iter().for_each(|step| step.run(&mut map));

        info!("Finished generation of '{}'", self.name);

        map
    }
}

/// A step during map generation.
pub enum GenerationStep {
    CreateAttribute(CreateAttribute),
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
            GenerationStep::CreateAttribute(step) => step.run(map),
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
