use crate::data::map::generation::{MapGeneration, MapGenerationData, MapGenerationError};
use std::convert::TryInto;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn read_map_generator(path: &str) -> Result<MapGeneration, MapGenerationError> {
    let string = fs::read_to_string(path)?;
    let data: MapGenerationData = serde_yaml::from_str(&string)?;
    data.try_into()
}

pub fn write_map_generator(
    map_generator: &MapGeneration,
    path: &str,
) -> Result<(), MapGenerationError> {
    let mut file = File::create(path)?;

    let data: MapGenerationData = map_generator.into();
    let s = serde_yaml::to_string(&data)?;

    file.write_all(s.as_bytes())?;

    Ok(())
}
