use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigStructure {
    pub cities: Vec<City>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct City {
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
}