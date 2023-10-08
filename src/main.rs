mod structures;
mod patterns;
mod storage;
mod config;
mod render;

use std::env;
mod utils;

use reqwest::{Client, Response};
use rustc_serialize::json::Json;
use crate::storage::storage_manager::{get_config, read_config, save_config};
use crate::structures::{weather_info::WeatherInfo};
use crate::structures::config_structure::{City};
use crate::structures::geocode_structure::RootGeoCodeStruct;

#[tokio::main]
async fn main() {
    let arg: Vec<String> = env::args().collect();
    let mut enter_city: String = String::new();

    //
    // if arg.len() >= 2  {
    //     if arg[1] == "switch-city" {
    //        return get_geocode(&enter_city).await;
    //     }
    // }

    match read_config() {
        Ok(_) => {
            get_config();
            // let cached_city: Json = get_config();
            //
            // let lat: f64 = cached_city.find_path(&["city_data", "latitude"]).unwrap().as_f64().unwrap();
            // let lon: f64 = cached_city.find_path(&["city_data", "longitude"]).unwrap().as_f64().unwrap();
            //
            // get_weather(&lat, &lon).await;
        }
        Err(_) => {
            println!("Enter city: ");

            std::io::stdin().read_line(&mut enter_city).unwrap();

            get_geocode(&enter_city).await;
        }
    }
}

async fn get_geocode(city: &str) {
    let url: String = format!("https://geocode.maps.co/search?q={city}");

    let http: Client = Client::new();

    let response: Response = http.get(&url).send().await.unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<RootGeoCodeStruct>().await {
                Ok(parsed) => {
                    choice_city(&parsed, &city).await;
                }
                Err(err) => println!("Hm, the response didn't match the shape we expected. {}", err),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    }
}

async fn choice_city(cities: &RootGeoCodeStruct, search_city: &str) {
    println!("{} cities have been found for your query.\nSelect the one that matches your query.\n\nExample: 1\n", cities.len());

    let mut enter_city: String = String::new();

    for (i, _city) in cities.iter().enumerate() {
        println!("[{}] City: {} | Description: {}", i + 1, search_city, cities[i].display_name);
    }

    println!("Enter city: ");

    std::io::stdin().read_line(&mut enter_city).unwrap();

    let selected_city = &cities[enter_city.trim().parse::<usize>().expect("Error number") - 1];

    let object_for_config = City {
        city: search_city.to_string(),
        longitude: selected_city.lon.parse().unwrap(),
        latitude: selected_city.lat.parse().unwrap()
    };
    save_config(object_for_config).expect("Error on save config file");

    get_weather(&selected_city.lat.parse::<f64>().unwrap(), &selected_city.lon.parse::<f64>().unwrap()).await;
}

async fn get_weather(lat: &f64, lon: &f64) {
    let url: String = format!("https://api.weather.yandex.ru/v2/forecast?lat={lat}&lon={lon}");

    let http: Client = Client::new();

    let response: Response = http.get(&url).header("X-Yandex-API-Key", config::API_KEY).send().await.unwrap();

    return match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<WeatherInfo>().await {
                Ok(parsed) => {
                    render::render(&parsed, &parsed.fact.condition);
                }
                Err(err) => println!("Hm, the response didn't match the shape we expected. {}", err),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    }
}

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
