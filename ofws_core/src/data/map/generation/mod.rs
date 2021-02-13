use crate::data::map::generation::step::GenerationStep;
use crate::data::map::Map2d;
use crate::data::math::size2d::Size2d;
use std::ops::Sub;

pub mod attribute;
pub mod distortion;
pub mod generator;
pub mod modify;
pub mod step;
pub mod transformer;

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
