use crate::data::map::generation::attribute::CreateAttribute;
use crate::data::map::generation::distortion::distortion2d::Distortion2d;
use crate::data::map::generation::distortion::Distortion1d;
use crate::data::map::generation::generator::GeneratorStep;
use crate::data::map::generation::modify::ModifyWithAttribute;
use crate::data::map::generation::transformer::TransformAttribute2d;
use crate::data::map::Map2d;

/// A step during [`MapGeneration`].
pub enum GenerationStep {
    CreateAttribute(CreateAttribute),
    DistortAlongX(Distortion1d),
    DistortAlongY(Distortion1d),
    Distortion2d(Distortion2d),
    GeneratorAdd(GeneratorStep),
    GeneratorSub(GeneratorStep),
    ModifyWithAttribute(ModifyWithAttribute),
    TransformAttribute2d(TransformAttribute2d),
}

impl GenerationStep {
    /// Runs the step.
    pub fn run(&self, map: &mut Map2d) {
        match self {
            GenerationStep::CreateAttribute(step) => step.run(map),
            GenerationStep::DistortAlongX(step) => step.distort_along_x(map),
            GenerationStep::DistortAlongY(step) => step.distort_along_y(map),
            GenerationStep::Distortion2d(step) => step.run(map),
            GenerationStep::GeneratorAdd(step) => step.add(map),
            GenerationStep::GeneratorSub(step) => step.sub(map),
            GenerationStep::ModifyWithAttribute(step) => step.run(map),
            GenerationStep::TransformAttribute2d(step) => step.run(map),
        }
    }
}
