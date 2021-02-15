use std::convert::TryFrom;
use std::ops::Sub;

use serde::{Deserialize, Serialize};

use crate::data::map::generation::step::{GenerationStep, GenerationStepData, GenerationStepError};
use crate::data::map::Map2d;
use crate::data::math::size2d::Size2d;

pub mod attributes;
pub mod io;
pub mod step;

#[derive(Debug)]
pub enum MapGenerationError {
    GenerationStep(usize, GenerationStepError),
    IoError(std::io::Error),
    SerdeError(serde_yaml::Error),
}

impl From<std::io::Error> for MapGenerationError {
    fn from(error: std::io::Error) -> Self {
        MapGenerationError::IoError(error)
    }
}

impl From<serde_yaml::Error> for MapGenerationError {
    fn from(error: serde_yaml::Error) -> Self {
        MapGenerationError::SerdeError(error)
    }
}

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
        let start = std::time::Instant::now();

        info!(
            "Generate the map '{}' with {:?} in {} steps:",
            self.name,
            self.size,
            self.steps.len(),
        );

        let mut start_step = start;
        let mut map = Map2d::with_name(self.name.clone(), self.size);

        self.steps.iter().for_each(|step| {
            step.run(&mut map);
            let end_step = std::time::Instant::now();
            let duration = end_step.sub(start_step);
            debug!("Step took {:?}", duration);
            start_step = end_step;
        });

        let end = std::time::Instant::now();
        let duration = end.sub(start);

        info!("Finished generation of '{}' in {:?}", self.name, duration);

        map
    }
}

// For serializing, deserializing & validating [`MapGeneration`].
///
///```
///# use std::convert::TryInto;
///# use ofws_core::data::map::generation::{MapGenerationData, MapGeneration};
///# use ofws_core::data::map::generation::attributes::create::CreateAttribute;
///# use ofws_core::data::map::generation::attributes::modify::ModifyWithAttributeData;
///# use ofws_core::data::map::generation::step::GenerationStepData;
///# use ofws_core::data::math::size2d::Size2d;
/// let step0 = GenerationStepData::CreateAttribute(CreateAttribute::new("a0", 42));
/// let step1 = GenerationStepData::CreateAttribute(CreateAttribute::new("a1", 200));
/// let modify = ModifyWithAttributeData::new("a0".to_string(), "a1".to_string(), 100, 10);
/// let step2 = GenerationStepData::ModifyWithAttribute(modify);
/// let steps = vec![step0, step1, step2];
/// let data = MapGenerationData::new("map".to_string(), Size2d::new(4, 5), steps);
///
/// let generation: MapGeneration = data.clone().try_into().unwrap();
/// let result: MapGenerationData = (&generation).into();
///
/// assert_eq!(data, result)
///```
#[derive(new, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct MapGenerationData {
    name: String,
    size: Size2d,
    steps: Vec<GenerationStepData>,
}

impl TryFrom<MapGenerationData> for MapGeneration {
    type Error = MapGenerationError;

    /// The conversion from [`MapGenerationData`] to [`MapGeneration`] can fail,
    /// if a [`GenerationStep`] uses an unknown [`Attribute`].
    ///
    ///```
    ///# use std::convert::TryInto;
    ///# use ofws_core::data::map::generation::{MapGenerationData, MapGeneration, MapGenerationError};
    ///# use ofws_core::data::map::generation::attributes::create::CreateAttribute;
    ///# use ofws_core::data::map::generation::attributes::modify::ModifyWithAttributeData;
    ///# use ofws_core::data::map::generation::step::GenerationStepData;
    ///# use ofws_core::data::map::generation::step::GenerationStepError::AttributeUnknown;
    ///# use ofws_core::data::math::size2d::Size2d;
    /// let create = GenerationStepData::CreateAttribute(CreateAttribute::new("a0", 0));
    /// let modify = ModifyWithAttributeData::new("a0".to_string(), "a1".to_string(), 100, 10);
    /// let modify = GenerationStepData::ModifyWithAttribute(modify);
    /// let steps = vec![create, modify];
    /// let data = MapGenerationData::new("map".to_string(), Size2d::new(4, 5), steps);
    ///
    /// let result: Result<MapGeneration, MapGenerationError> = data.try_into();
    ///
    /// match result {
    ///    Ok(_) => panic!("Wrong!"),
    ///    Err(error) => match error {
    ///        MapGenerationError::GenerationStep(step, error) => {
    ///            assert_eq!(step, 1);
    ///            assert_eq!(error, AttributeUnknown("a1".to_string()));
    ///        },
    ///        MapGenerationError::IoError(_) => panic!("Wrong!"),
    ///        MapGenerationError::SerdeError(_) => panic!("Wrong!"),
    ///    }
    /// }
    ///```
    fn try_from(data: MapGenerationData) -> Result<Self, Self::Error> {
        let mut attributes: Vec<String> = Vec::new();
        let steps: Result<Vec<_>, _> = data
            .steps
            .into_iter()
            .enumerate()
            .map(|(index, data)| {
                data.try_convert(&mut attributes)
                    .map_err(|error| MapGenerationError::GenerationStep(index, error))
            })
            .collect();
        let steps = steps?;
        Ok(MapGeneration::new(data.name, data.size, steps))
    }
}

impl From<&MapGeneration> for MapGenerationData {
    fn from(map_generation: &MapGeneration) -> Self {
        let mut attributes: Vec<String> = Vec::new();
        let steps: Vec<GenerationStepData> = map_generation
            .steps
            .iter()
            .map(|data| data.convert(&mut attributes))
            .collect();
        MapGenerationData::new(map_generation.name.clone(), map_generation.size, steps)
    }
}
