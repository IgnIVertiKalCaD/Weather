use serde_derive::{Serialize, Deserialize};

pub type RootGeoCodeStruct = Vec<GeoCodeStruct>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoCodeStruct {
    #[serde(rename = "place_id")]
    pub place_id: i64,
    pub licence: String,
    #[serde(rename = "powered_by")]
    pub powered_by: String,
    #[serde(rename = "osm_type")]
    pub osm_type: String,
    #[serde(rename = "osm_id")]
    pub osm_id: i64,
    pub boundingbox: Vec<String>,
    pub lat: String,
    pub lon: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    pub class: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub importance: f64,
}
