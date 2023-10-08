use crate::structures::{config_structure::City, config_structure::ConfigStructure};
use rustc_serialize::json::{BuilderError, Json};
use std::fs::{File};
use std::io;
use std::io::{BufReader, BufWriter, Error, Read};


const NAME_CONFIG_FILE: &str = "config.json";

pub fn save_config(payload: City) -> Result<(), io::Error> {
    let config_structure = ConfigStructure {
        city_data: &payload
    };

    let file = File::create(&NAME_CONFIG_FILE)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &config_structure)?;
    Ok(())
}

pub fn read_config() -> Result<(), Box<Error>>  {
    match File::open(&NAME_CONFIG_FILE) {
        Ok(file) => {
            let mut reader = BufReader::new(file);

            let json: Result<Json, BuilderError> = Json::from_reader(&mut reader);

            if json.ok().is_some() {
                Ok(())
            } else {
                Err(Box::from(Error::new(io::ErrorKind::Other, "oh no!")))
            }
        },
        Err(err) => Err(Box::from(err))
    }
}

pub fn get_config() -> Result<Json, BuilderError> {
    match read_config() {
        Ok(_) => {
            let mut file = File::open(&NAME_CONFIG_FILE).expect("Error Read");
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();

            Json::from_str(&data)
        }
        Err(err) => Err(BuilderError::from(*err))
    }
}