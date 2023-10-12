use crate::structures::{config_structure::City, config_structure::ConfigStructure};
use crate::storage::helpers::helpers::*;
use std::{io};
use std::path::Path;
use serde::de::Error;

pub fn remove_city_by_index(city_index: usize) -> Result<(), ()> {
    match get_config() {
        Ok(config) => {
            let city_name = &config.cities[city_index].city;

            remove_city_by_name(city_name).expect("Error remove city by index");

            Ok(())
        }
        Err(_) => Err(())
    }
}


pub fn remove_city_by_name(city_name: &str) -> Result<(), ()> {
    match get_config() {
        Ok(mut config) => {
            let index = config.cities.iter().position(|el| {
                el.city == city_name
            }).expect("Not found city");

            config.cities.remove(index);

            save_config(&config.cities).expect("Error remove city. (Error save config)");

            println!("City: {} removed", city_name);

            Ok(())
        }
        Err(_) => Err(())
    }
}

pub fn add_city(city: City) {
    match get_config() {
        Ok(mut config) => {
            config.cities.push(city);

            save_config(&config.cities).expect("Error add city. (Error save config)")
        }
        Err(_) => {
            restore(&vec![city])
        }
    }
}

pub fn save_config(payload: &Vec<City>) -> Result<(), io::Error> {
    let path_to_config_dir: String = create_path_to_dir_for_save_config_file();

    if Path::read_dir(path_to_config_dir.as_ref()).is_ok() {
        save(payload, &path_to_config_dir).expect("TODO: panic message");
        Ok(())
    } else {
        restore(payload);
        Ok(())
    }
}

pub fn get_config() -> serde_json::Result<ConfigStructure> {
    match read_config() {
        Ok(config_file) => Ok(config_file),
        Err(err) => Err(serde_json::Error::custom(err))
    }
}
