use crate::data::map::generation::attribute::CreateAttribute;
use crate::data::map::generation::distortion::distortion2d::{Distortion2d, Distortion2dData};
use crate::data::map::generation::distortion::{Distortion1d, Distortion1dData};
use crate::data::map::generation::generator::{GeneratorStep, GeneratorStepData};
use crate::data::map::generation::modify::ModifyWithAttribute;
use crate::data::map::generation::transformer::{TransformAttribute2d, TransformAttribute2dData};
use crate::data::map::Map2d;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

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

/// For serializing, deserializing & validating [`GenerationStep`].
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum GenerationStepData {
    CreateAttribute(CreateAttribute),
    DistortAlongX(Distortion1dData),
    DistortAlongY(Distortion1dData),
    Distortion2d(Distortion2dData),
    GeneratorAdd(GeneratorStepData),
    GeneratorSub(GeneratorStepData),
    ModifyWithAttribute(ModifyWithAttribute),
    TransformAttribute2d(TransformAttribute2dData),
}

impl TryFrom<GenerationStepData> for GenerationStep {
    type Error = &'static str;

    fn try_from(data: GenerationStepData) -> Result<Self, Self::Error> {
        match data {
            GenerationStepData::CreateAttribute(step) => Ok(GenerationStep::CreateAttribute(step)),
            GenerationStepData::DistortAlongX(step) => {
                Ok(GenerationStep::DistortAlongX(step.try_into()?))
            }
            GenerationStepData::DistortAlongY(step) => {
                Ok(GenerationStep::DistortAlongY(step.try_into()?))
            }
            GenerationStepData::Distortion2d(step) => {
                Ok(GenerationStep::Distortion2d(step.try_into()?))
            }
            GenerationStepData::GeneratorAdd(step) => {
                Ok(GenerationStep::GeneratorAdd(step.try_into()?))
            }
            GenerationStepData::GeneratorSub(step) => {
                Ok(GenerationStep::GeneratorSub(step.try_into()?))
            }
            GenerationStepData::ModifyWithAttribute(step) => {
                Ok(GenerationStep::ModifyWithAttribute(step))
            }
            GenerationStepData::TransformAttribute2d(step) => {
                Ok(GenerationStep::TransformAttribute2d(step.try_into()?))
            }
        }
    }
}

impl From<GenerationStep> for GenerationStepData {
    fn from(generator: GenerationStep) -> Self {
        match generator {
            GenerationStep::CreateAttribute(data) => GenerationStepData::CreateAttribute(data),
            GenerationStep::DistortAlongX(data) => GenerationStepData::DistortAlongX(data.into()),
            GenerationStep::DistortAlongY(data) => GenerationStepData::DistortAlongY(data.into()),
            GenerationStep::Distortion2d(data) => GenerationStepData::Distortion2d(data.into()),
            GenerationStep::GeneratorAdd(data) => GenerationStepData::GeneratorAdd(data.into()),
            GenerationStep::GeneratorSub(data) => GenerationStepData::GeneratorSub(data.into()),
            GenerationStep::ModifyWithAttribute(data) => {
                GenerationStepData::ModifyWithAttribute(data)
            }
            GenerationStep::TransformAttribute2d(data) => {
                GenerationStepData::TransformAttribute2d(data.into())
            }
        }
    }
}

pub fn assert_eq(data: GenerationStepData) {
    let generator: GenerationStep = data.clone().try_into().unwrap();
    let result: GenerationStepData = generator.into();
    assert_eq!(result, data)
}
