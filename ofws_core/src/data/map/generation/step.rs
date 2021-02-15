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
use GenerationStep::*;

#[derive(Debug, Eq, PartialEq)]
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
            CreateAttribute(step) => step.run(map),
            DistortAlongX(step) => step.distort_along_x(map),
            DistortAlongY(step) => step.distort_along_y(map),
            Distortion2d(step) => step.run(map),
            GeneratorAdd(step) => step.add(map),
            GeneratorSub(step) => step.sub(map),
            ModifyWithAttribute(step) => step.run(map),
            TransformAttribute2d(step) => step.run(map),
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

type Data = GenerationStepData;

impl GenerationStepData {
    pub fn try_convert(
        self,
        attributes: &mut Vec<String>,
    ) -> Result<GenerationStep, GenerationStepError> {
        match self {
            Data::CreateAttribute(step) => {
                attributes.push(step.get_attribute().to_string());
                Ok(CreateAttribute(step))
            }
            Data::DistortAlongX(step) => Ok(DistortAlongX(step.try_convert(attributes)?)),
            Data::DistortAlongY(step) => Ok(DistortAlongY(step.try_convert(attributes)?)),
            Data::Distortion2d(step) => Ok(Distortion2d(step.try_convert(attributes)?)),
            Data::GeneratorAdd(step) => Ok(GeneratorAdd(step.try_convert(attributes)?)),
            Data::GeneratorSub(step) => Ok(GeneratorSub(step.try_convert(attributes)?)),
            Data::ModifyWithAttribute(step) => {
                Ok(ModifyWithAttribute(step.try_convert(attributes)?))
            }
            Data::TransformAttribute2d(step) => {
                Ok(TransformAttribute2d(step.try_convert(attributes)?))
            }
        }
    }
}

impl GenerationStep {
    pub fn convert(&self, attributes: &mut Vec<String>) -> GenerationStepData {
        match self {
            CreateAttribute(data) => {
                attributes.push(data.get_attribute().to_string());
                Data::CreateAttribute(data.clone())
            }
            DistortAlongX(data) => Data::DistortAlongX(data.convert(attributes)),
            DistortAlongY(data) => Data::DistortAlongY(data.convert(attributes)),
            Distortion2d(data) => Data::Distortion2d(data.convert(attributes)),
            GeneratorAdd(data) => Data::GeneratorAdd(data.convert(attributes)),
            GeneratorSub(data) => Data::GeneratorSub(data.convert(attributes)),
            ModifyWithAttribute(data) => Data::ModifyWithAttribute(data.convert(attributes)),
            TransformAttribute2d(data) => Data::TransformAttribute2d(data.convert(attributes)),
        }
    }
}

pub fn get_attribute_id(
    attribute: &str,
    attributes: &[String],
) -> Result<usize, GenerationStepError> {
    attributes
        .iter()
        .position(|name| name.eq(attribute))
        .ok_or_else(|| GenerationStepError::AttributeUnknown(attribute.to_string()))
}
