mod structures;
mod patterns;
mod storage;
mod config;
mod render;
mod utils;
mod dialog;

use crate::storage::storage_manager::{add_city, get_config};
use crate::structures::{weather_info::WeatherInfo};
use crate::structures::config_structure::{City};
use crate::structures::geocode_structure::{RootGeoCodeStruct};
use std::io::{Error, ErrorKind};
use reqwest::{Client, Response};
use crate::dialog::dialog::{show_dialog_for_choice_city, show_dialog_for_entering_city};
use crate::utils::args::args;

#[tokio::main]
async fn main() {
    args().await;

    match get_config() {
        Ok(config) => {
            let mut handles = vec![];

            match config.cities.len() {
                0 => {
                    let selected_city: City = show_dialog_for_entering_city().await;
                    add_city(selected_city)
                }
                _ => {
                    for city in config.cities {
                        handles.push(
                            tokio::spawn(async move {
                                let weather = get_weather(city.latitude, city.longitude).await.unwrap();
                                render::render(&weather, &weather.fact.condition, &city.city)
                            })
                        )
                    }
                    futures::future::join_all(handles).await;
                }
            }
        }
        Err(_) => {
            let selected_city: City = show_dialog_for_entering_city().await;
            add_city(selected_city)
        }
    }
}


async fn get_geocode(city: &str) -> Result<RootGeoCodeStruct, Error> {
    let url: String = format!("https://geocode.maps.co/search?q={city}");

    let http: Client = Client::new();

    let response: Response = http.get(&url).send().await.unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<RootGeoCodeStruct>().await {
                Ok(parsed) => Ok(parsed),
                Err(err) => {
                    println!("Hm, the response didn't match the shape we expected.");
                    Err(Error::new(ErrorKind::InvalidData, err))
                }
            }
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            Err(Error::new(ErrorKind::ConnectionAborted, "Need to grab a new token"))
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    }
}

async fn choice_city(cities: RootGeoCodeStruct, desired_city: &str) -> City {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

    println!("{} cities have been found for your query.\nSelect the one that matches your query.\n\nExample: 1\n", cities.len());

    for (i, _city) in cities.iter().enumerate() {
        print!("[{}] City: {} \n | Description: {}\n", i + 1, desired_city, cities[i].display_name);
    }

    let selected_city = show_dialog_for_choice_city();
    let city_data = &cities[selected_city];

    City {
        city: desired_city.to_string(),
        longitude: city_data.lon.parse().unwrap(),
        latitude: city_data.lat.parse().unwrap(),
    }
}

async fn get_weather(lat: f64, lon: f64) -> Result<WeatherInfo, Error> {
    let url: String = format!("https://api.weather.yandex.ru/v2/forecast?lat={lat}&lon={lon}");

    let http: Client = Client::new();

    let response: Response = http.get(&url).header("X-Yandex-API-Key", config::API_KEY).send().await.unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<WeatherInfo>().await {
                Ok(parsed) => Ok(parsed),
                Err(err) => {
                    println!("Hm, the response didn't match the shape we expected.");
                    Err(Error::new(ErrorKind::InvalidData, err))
                }
            }
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            Err(Error::new(ErrorKind::ConnectionAborted, "Need to grab a new token"))
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    }
}