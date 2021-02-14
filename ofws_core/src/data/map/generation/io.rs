use crate::data::map::generation::{MapGeneration, MapGenerationData, MapGenerationError};
use std::convert::TryInto;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn read_map_generator(path: &str) -> Result<MapGeneration, MapGenerationError> {
    let string = fs::read_to_string(path)?;
    let data: MapGenerationData = serde_yaml::from_str(&string)?;
    data.try_into()
}

pub fn write_map_generator(map_generator: &MapGeneration, path: &str) {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let data: MapGenerationData = map_generator.into();
    let s = serde_yaml::to_string(&data).unwrap();

    match file.write_all(s.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => info!("Successfully wrote map generator to {}", display),
    }
}
