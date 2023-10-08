use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ConfigStructure<'de> {
    pub city_data: &'de City
}

#[derive(Default, Debug, PartialEq, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct City {
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
}