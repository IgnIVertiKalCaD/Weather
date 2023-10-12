use std::io;
use std::io::Write;
use crate::{choice_city, get_geocode};
use crate::storage::storage_manager::{get_config, remove_city_by_index};
use crate::structures::config_structure::City;
use crate::structures::geocode_structure::RootGeoCodeStruct;


pub fn show_dialog_for_deleting_city() {
    show_cities();

    let enter_city = dialog("Enter the city number to be deleted >> ");
    remove_city_by_index(enter_city.parse::<usize>().unwrap() - 1).expect("Error remove city");
}

pub async fn show_dialog_for_entering_city() -> City {
    let enter_city = dialog("Enter city >> ");
    let geo_code: RootGeoCodeStruct = get_geocode(&enter_city).await.unwrap();
    choice_city(geo_code, &enter_city).await
}

pub fn show_dialog_for_choice_city() -> usize {
    println!();
    let selected_city = dialog("Select city >> ");
    return selected_city.parse::<usize>().unwrap() - 1;
}
fn dialog(text: &str) -> String {
    let mut buff: String = String::new();

    print!("{}", text);

    let _ = io::stdout().flush();
    io::stdin().read_line(&mut buff).unwrap();
    let _ = buff.pop().unwrap().to_string();

    return buff;
}

fn show_cities() {
    match get_config() {
        Ok(config) => {
            for (i, city) in config.cities.iter().enumerate() {
                print!("[{}] City: {} \n |  Latitude: {} \n +- Longitude {}\n\n", i + 1, city.city, city.latitude, city.longitude);
            }
        }
        Err(_) => {}
    };
}