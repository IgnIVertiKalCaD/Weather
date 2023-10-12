use crate::structures::config_structure::{City, ConfigStructure};

use std::{fs, io};
use std::fs::File;
use std::io::{BufReader, BufWriter};

use crate::storage::constants::{NAME_CONFIG_FILE, NAME_CONFIG_FOLDER};

pub fn restore(payload: &Vec<City>) {
    let path: String = create_path_to_dir_for_save_config_file();

    match fs::create_dir(&path) {
        _ => save(payload, &path).expect("Error save on restore (Ok)"),
    }
}

pub fn read_config() -> Result<ConfigStructure, io::Error> {
    let path_to_config_dir = create_path_to_dir_for_save_config_file();
    let full_path = format!("{path_to_config_dir}/{NAME_CONFIG_FILE}");

    match File::open(full_path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let json: Result<ConfigStructure, serde_json::Error> = serde_json::from_reader(reader);

            match json {
                Ok(json) => Ok(json),
                Err(err) => {Err(io::Error::from(err))}
            }
        }
        Err(err) => Err(err)
    }
}

pub fn save(payload: &Vec<City>, path: &String) -> serde_json::Result<()> {
    let config_structure = ConfigStructure {
        cities: payload.clone()
    };

    let file = File::create(format!("{path}/{NAME_CONFIG_FILE}")).unwrap();
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &config_structure)?;
    Ok(())
}


pub fn create_path_to_dir_for_save_config_file() -> String {
    let config_path = std::env::var("XDG_CONFIG_HOME")
        .or_else(|_| std::env::var("HOME").map(|home| format!("{}/.config", home))).unwrap();

    return format!("{config_path}/{NAME_CONFIG_FOLDER}");
}