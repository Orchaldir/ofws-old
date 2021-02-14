use crate::data::map::generation::attributes::create::CreateAttribute;
use crate::data::map::generation::attributes::distortion1d::{Distortion1d, Distortion1dData};
use crate::data::map::generation::attributes::distortion2d::{Distortion2d, Distortion2dData};
use crate::data::map::generation::attributes::generator::{GeneratorStep, GeneratorStepData};
use crate::data::map::generation::attributes::modify::{
    ModifyWithAttribute, ModifyWithAttributeData,
};
use crate::data::map::generation::attributes::transformer::{
    TransformAttribute2d, TransformAttribute2dData,
};
use crate::data::map::Map2d;
use crate::data::math::generator::generator1d::Generator1dError;
use crate::data::math::generator::generator2d::Generator2dError;
use crate::data::math::transformer::transformer2d::Transformer2dError;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum GenerationStepError {
    AttributeUnknown(String),
    Generator1d(Generator1dError),
    Generator2d(Generator2dError),
    Transformer2d(Transformer2dError),
}

impl From<Generator1dError> for GenerationStepError {
    fn from(error: Generator1dError) -> Self {
        GenerationStepError::Generator1d(error)
    }
}

impl From<Generator2dError> for GenerationStepError {
    fn from(error: Generator2dError) -> Self {
        GenerationStepError::Generator2d(error)
    }
}

impl From<Transformer2dError> for GenerationStepError {
    fn from(error: Transformer2dError) -> Self {
        GenerationStepError::Transformer2d(error)
    }
}

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
    ModifyWithAttribute(ModifyWithAttributeData),
    TransformAttribute2d(TransformAttribute2dData),
}

impl GenerationStepData {
    pub fn try_convert(
        self,
        attributes: &mut Vec<String>,
    ) -> Result<GenerationStep, GenerationStepError> {
        match self {
            GenerationStepData::CreateAttribute(step) => {
                attributes.push(step.get_attribute().to_string());
                Ok(GenerationStep::CreateAttribute(step))
            }
            GenerationStepData::DistortAlongX(step) => {
                Ok(GenerationStep::DistortAlongX(step.try_convert(attributes)?))
            }
            GenerationStepData::DistortAlongY(step) => {
                Ok(GenerationStep::DistortAlongY(step.try_convert(attributes)?))
            }
            GenerationStepData::Distortion2d(step) => {
                Ok(GenerationStep::Distortion2d(step.try_convert(attributes)?))
            }
            GenerationStepData::GeneratorAdd(step) => {
                Ok(GenerationStep::GeneratorAdd(step.try_convert(attributes)?))
            }
            GenerationStepData::GeneratorSub(step) => {
                Ok(GenerationStep::GeneratorSub(step.try_convert(attributes)?))
            }
            GenerationStepData::ModifyWithAttribute(step) => Ok(
                GenerationStep::ModifyWithAttribute(step.try_convert(attributes)?),
            ),
            GenerationStepData::TransformAttribute2d(step) => Ok(
                GenerationStep::TransformAttribute2d(step.try_convert(attributes)?),
            ),
        }
    }
}

impl From<&GenerationStep> for GenerationStepData {
    fn from(generator: &GenerationStep) -> Self {
        match generator {
            GenerationStep::CreateAttribute(data) => {
                GenerationStepData::CreateAttribute(data.clone())
            }
            GenerationStep::DistortAlongX(data) => GenerationStepData::DistortAlongX(data.into()),
            GenerationStep::DistortAlongY(data) => GenerationStepData::DistortAlongY(data.into()),
            GenerationStep::Distortion2d(data) => GenerationStepData::Distortion2d(data.into()),
            GenerationStep::GeneratorAdd(data) => GenerationStepData::GeneratorAdd(data.into()),
            GenerationStep::GeneratorSub(data) => GenerationStepData::GeneratorSub(data.into()),
            GenerationStep::ModifyWithAttribute(data) => {
                GenerationStepData::ModifyWithAttribute(data.into())
            }
            GenerationStep::TransformAttribute2d(data) => {
                GenerationStepData::TransformAttribute2d(data.into())
            }
        }
    }
}

pub fn get_attribute_id(
    attribute: &str,
    attributes: &mut Vec<String>,
) -> Result<usize, GenerationStepError> {
    attributes
        .iter()
        .position(|name| name.eq(attribute))
        .ok_or_else(|| GenerationStepError::AttributeUnknown(attribute.to_string()))
}
