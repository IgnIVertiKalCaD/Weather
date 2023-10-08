use crate::structures::{config_structure::City, config_structure::ConfigStructure};
use std::fs::*;
use std::io::{BufWriter, Error, Read};
use rustc_serialize::json::Json;

type Result<T> = std::result::Result<T, Error>;

const NAME_CONFIG_FILE: &str = "config.json";

const back_up: &str = r#"{"city_data": {"city": "Revda","latitude": 67.9415427,"longitude": 34.5489702}}"#;


pub fn save_config(payload: City) -> Result<()> {
    let config_structure = ConfigStructure {
        city_data: &payload
    };

    let file = File::create(&NAME_CONFIG_FILE)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &config_structure)?;
    Ok(())
}

pub fn read_config() -> Result<()> {
    let mut file: File = File::open(&NAME_CONFIG_FILE)?;
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    print_type_of(&data);
    println!("{:?}", &data);


    let file_is_trashed = &data;

    // let file_is_trashed: () = match file_is_trashed {
    //     "" => false,
    //     _ => true
    // };

    // println!("{:?}", file_is_trashed);

    if file_is_trashed  {
        let data = Json::from_str(&back_up).unwrap();
        save_config(City {
            city: data.find_path(&["city_data", "city"]).unwrap().to_string(),
            latitude: data.find_path(&["city_data", "latitude"]).unwrap().as_f64().unwrap(),
            longitude: data.find_path(&["city_data", "longitude"]).unwrap().as_f64().unwrap(),
        }).expect("Ultra Error. Write me to help or write in issue on github CODE: 4001. You delete config file?");
    }

    Ok(())
}

pub fn get_config() {
    // match read_config() {
    //     Ok(_) => {
    //         let mut file = File::open(&NAME_CONFIG_FILE).unwrap();
    //         let mut data = String::new();
    //         file.read_to_string(&mut data).unwrap();
    //
    //         Json::from_str(&data).unwrap()
    //     }
    //     Err(_) => {

    //         println!("a {}", data)
    //
    //         save_config(City {
    //             city: data.find_path(&["city_data", "city"]).unwrap().to_string(),
    //             latitude: data.find_path(&["city_data", "latitude"]).unwrap().as_f64().unwrap(),
    //             longitude: data.find_path(&["city_data", "longitude"]).unwrap().as_f64().unwrap(),
    //         }).expect("Ultra Error. Write me to help or write in issue on github CODE: 4001");
    //         //
    //     }
    // }
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
