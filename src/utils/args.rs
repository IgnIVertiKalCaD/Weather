use crate::{choice_city, get_geocode};
use crate::dialog::dialog::{show_dialog_for_deleting_city, show_dialog_for_entering_city};
use crate::storage::storage_manager::{add_city, remove_city_by_name};
use crate::structures::config_structure::City;
use crate::structures::geocode_structure::RootGeoCodeStruct;

pub async fn args() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1) {
        None => {}
        Some(arg) => {
            if arg == "remove-city" {
                return match args.get(2) {
                    Some(city) => {
                        remove_city_by_name(&city).expect("Error remove city");
                    }
                    None => {
                        show_dialog_for_deleting_city()
                    }
                };
            }
            if arg == "add-city" {
                return match args.get(2) {
                    Some(city) => {
                        let geo_code: RootGeoCodeStruct = get_geocode(&city).await.unwrap();
                        let selected_city = choice_city(geo_code, &city).await;
                        add_city(selected_city)
                    }
                    None => {
                        let selected_city: City = show_dialog_for_entering_city().await;
                        add_city(selected_city)
                    }
                };
            }
        }
    }
}