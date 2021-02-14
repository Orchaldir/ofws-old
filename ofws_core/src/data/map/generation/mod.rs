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
    GenerationStep(GenerationStepError),
}

impl From<GenerationStepError> for MapGenerationError {
    fn from(error: GenerationStepError) -> Self {
        MapGenerationError::GenerationStep(error)
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
///# use ofws_core::data::map::generation::step::GenerationStepData;
///# use ofws_core::data::math::size2d::Size2d;
/// let steps = vec![GenerationStepData::CreateAttribute(CreateAttribute::new("attribute", 42))];
/// let data = MapGenerationData::new("map".to_string(), Size2d::new(4, 5), steps);
/// let step: MapGeneration = data.clone().try_into().unwrap();
/// let result: MapGenerationData = (&step).into();
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

    fn try_from(data: MapGenerationData) -> Result<Self, Self::Error> {
        let mut attributes: Vec<String> = Vec::new();
        let steps: Result<Vec<_>, _> = data
            .steps
            .into_iter()
            .map(|data| data.try_convert(&mut attributes))
            .collect();
        let steps = steps?;
        Ok(MapGeneration::new(data.name, data.size, steps))
    }
}

impl From<&MapGeneration> for MapGenerationData {
    fn from(map_generation: &MapGeneration) -> Self {
        let steps: Vec<GenerationStepData> = map_generation
            .steps
            .iter()
            .map(|data| data.into())
            .collect();
        MapGenerationData::new(map_generation.name.clone(), map_generation.size, steps)
    }
}
