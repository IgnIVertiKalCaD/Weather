use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct WeatherInfo {
    pub now: i64,
    pub info: Info,
    pub fact: Fact,
    pub forecasts: Vec<Forecast>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Info {
    pub tzinfo: Tzinfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tzinfo {
    pub name: String,
    pub abbr: String,
    pub dst: bool,
    pub offset: i64,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Fact {
    #[serde(rename = "obs_time")]
    pub obs_time: i64,
    pub uptime: i64,
    pub temp: i64,
    #[serde(rename = "feels_like")]
    pub feels_like: i64,
    #[serde(rename = "temp_water")]
    pub temp_water: i64,
    pub icon: String,
    pub condition: String,
    pub cloudness: f64,
    #[serde(rename = "prec_type")]
    pub prec_type: i64,
    #[serde(rename = "prec_prob")]
    pub prec_prob: i64,
    #[serde(rename = "prec_strength")]
    pub prec_strength: f64,
    #[serde(rename = "is_thunder")]
    pub is_thunder: bool,
    #[serde(rename = "wind_speed")]
    pub wind_speed: f64,
    #[serde(rename = "wind_dir")]
    pub wind_dir: String,
    #[serde(rename = "pressure_mm")]
    pub pressure_mm: i64,
    #[serde(rename = "pressure_pa")]
    pub pressure_pa: i64,
    pub humidity: i64,
    pub daytime: String,
    pub polar: bool,
    pub season: String,
    pub source: String,
    #[serde(rename = "soil_moisture")]
    pub soil_moisture: f64,
    #[serde(rename = "soil_temp")]
    pub soil_temp: i64,
    #[serde(rename = "uv_index")]
    pub uv_index: i64,
    #[serde(rename = "wind_gust")]
    pub wind_gust: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Forecast {
    pub date: String,
    #[serde(rename = "date_ts")]
    pub date_ts: i64,
    pub week: i64,
    pub sunrise: String,
    pub sunset: String,
    #[serde(rename = "rise_begin")]
    pub rise_begin: String,
    #[serde(rename = "set_end")]
    pub set_end: String,
    #[serde(rename = "moon_code")]
    pub moon_code: i64,
    #[serde(rename = "moon_text")]
    pub moon_text: String,
    pub parts: Parts,
    pub hours: Vec<Hour>,
    pub biomet: Option<Biomet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Parts {
    pub evening: Evening,
    pub night: Night,
    #[serde(rename = "night_short")]
    pub night_short: NightShort,
    #[serde(rename = "day_short")]
    pub day_short: DayShort,
    pub morning: Morning,
    pub day: Day,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Morning {
    #[serde(rename = "_source")]
    pub source: String,
    #[serde(rename = "temp_min")]
    pub temp_min: i64,
    #[serde(rename = "temp_avg")]
    pub temp_avg: i64,
    #[serde(rename = "temp_max")]
    pub temp_max: i64,
    #[serde(rename = "wind_speed")]
    pub wind_speed: f64,
    #[serde(rename = "wind_gust")]
    pub wind_gust: f64,
    #[serde(rename = "wind_dir")]
    pub wind_dir: String,
    #[serde(rename = "pressure_mm")]
    pub pressure_mm: i64,
    #[serde(rename = "pressure_pa")]
    pub pressure_pa: i64,
    pub humidity: i64,
    #[serde(rename = "soil_temp")]
    pub soil_temp: i64,
    #[serde(rename = "soil_moisture")]
    pub soil_moisture: f64,
    #[serde(rename = "prec_mm")]
    pub prec_mm: f64,
    #[serde(rename = "prec_prob")]
    pub prec_prob: i64,
    #[serde(rename = "prec_period")]
    pub prec_period: i64,
    pub cloudness: f64,
    #[serde(rename = "prec_type")]
    pub prec_type: i64,
    #[serde(rename = "prec_strength")]
    pub prec_strength: f64,
    pub icon: String,
    pub condition: String,
    #[serde(rename = "uv_index")]
    pub uv_index: Option<i64>,
    #[serde(rename = "feels_like")]
    pub feels_like: i64,
    pub daytime: String,
    pub polar: bool,
    #[serde(rename = "fresh_snow_mm")]
    pub fresh_snow_mm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DayShort {
    #[serde(rename = "_source")]
    pub source: String,
    pub temp: i64,
    #[serde(rename = "temp_min")]
    pub temp_min: i64,
    #[serde(rename = "wind_speed")]
    pub wind_speed: f64,
    #[serde(rename = "wind_gust")]
    pub wind_gust: f64,
    #[serde(rename = "wind_dir")]
    pub wind_dir: String,
    #[serde(rename = "pressure_mm")]
    pub pressure_mm: i64,
    #[serde(rename = "pressure_pa")]
    pub pressure_pa: i64,
    pub humidity: i64,
    #[serde(rename = "soil_temp")]
    pub soil_temp: i64,
    #[serde(rename = "soil_moisture")]
    pub soil_moisture: f64,
    #[serde(rename = "prec_mm")]
    pub prec_mm: f64,
    #[serde(rename = "prec_prob")]
    pub prec_prob: i64,
    #[serde(rename = "prec_period")]
    pub prec_period: i64,
    pub cloudness: f64,
    #[serde(rename = "prec_type")]
    pub prec_type: i64,
    #[serde(rename = "prec_strength")]
    pub prec_strength: f64,
    pub icon: String,
    pub condition: String,
    #[serde(rename = "uv_index")]
    pub uv_index: Option<i64>,
    #[serde(rename = "feels_like")]
    pub feels_like: i64,
    pub daytime: String,
    pub polar: bool,
    #[serde(rename = "fresh_snow_mm")]
    pub fresh_snow_mm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Evening {
    #[serde(rename = "_source")]
    pub source: String,
    #[serde(rename = "temp_min")]
    pub temp_min: i64,
    #[serde(rename = "temp_avg")]
    pub temp_avg: i64,
    #[serde(rename = "temp_max")]
    pub temp_max: i64,
    #[serde(rename = "wind_speed")]
    pub wind_speed: f64,
    #[serde(rename = "wind_gust")]
    pub wind_gust: f64,
    #[serde(rename = "wind_dir")]
    pub wind_dir: String,
    #[serde(rename = "pressure_mm")]
    pub pressure_mm: i64,
    #[serde(rename = "pressure_pa")]
    pub pressure_pa: i64,
    pub humidity: i64,
    #[serde(rename = "soil_temp")]
    pub soil_temp: i64,
    #[serde(rename = "soil_moisture")]
    pub soil_moisture: f64,
    #[serde(rename = "prec_mm")]
    pub prec_mm: f64,
    #[serde(rename = "prec_prob")]
    pub prec_prob: i64,
    #[serde(rename = "prec_period")]
    pub prec_period: i64,
    pub cloudness: f64,
    #[serde(rename = "prec_type")]
    pub prec_type: i64,
    #[serde(rename = "prec_strength")]
    pub prec_strength: f64,
    pub icon: String,
    pub condition: String,
    #[serde(rename = "uv_index")]
    pub uv_index: Option<i64>,
    #[serde(rename = "feels_like")]
    pub feels_like: i64,
    pub daytime: String,
    pub polar: bool,
    #[serde(rename = "fresh_snow_mm")]
    pub fresh_snow_mm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Day {
    #[serde(rename = "_source")]
    pub source: String,
    #[serde(rename = "temp_min")]
    pub temp_min: i64,
    #[serde(rename = "temp_avg")]
    pub temp_avg: i64,
    #[serde(rename = "temp_max")]
    pub temp_max: i64,
    #[serde(rename = "wind_speed")]
    pub wind_speed: f64,
    #[serde(rename = "wind_gust")]
    pub wind_gust: f64,
    #[serde(rename = "wind_dir")]
    pub wind_dir: String,
    #[serde(rename = "pressure_mm")]
    pub pressure_mm: i64,
    #[serde(rename = "pressure_pa")]
    pub pressure_pa: i64,
    pub humidity: i64,
    #[serde(rename = "soil_temp")]
    pub soil_temp: i64,
    #[serde(rename = "soil_moisture")]
    pub soil_moisture: f64,
    #[serde(rename = "prec_mm")]
    pub prec_mm: f64,
    #[serde(rename = "prec_prob")]
    pub prec_prob: i64,
    #[serde(rename = "prec_period")]
    pub prec_period: i64,
    pub cloudness: f64,
    #[serde(rename = "prec_type")]
    pub prec_type: i64,
    #[serde(rename = "prec_strength")]
    pub prec_strength: f64,
    pub icon: String,
    pub condition: String,
    #[serde(rename = "uv_index")]
    pub uv_index: Option<i64>,
    #[serde(rename = "feels_like")]
    pub feels_like: i64,
    pub daytime: String,
    pub polar: bool,
    #[serde(rename = "fresh_snow_mm")]
    pub fresh_snow_mm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Night {
    #[serde(rename = "_source")]
    pub source: String,
    #[serde(rename = "temp_min")]
    pub temp_min: i64,
    #[serde(rename = "temp_avg")]
    pub temp_avg: i64,
    #[serde(rename = "temp_max")]
    pub temp_max: i64,
    #[serde(rename = "wind_speed")]
    pub wind_speed: f64,
    #[serde(rename = "wind_gust")]
    pub wind_gust: f64,
    #[serde(rename = "wind_dir")]
    pub wind_dir: String,
    #[serde(rename = "pressure_mm")]
    pub pressure_mm: i64,
    #[serde(rename = "pressure_pa")]
    pub pressure_pa: i64,
    pub humidity: i64,
    #[serde(rename = "soil_temp")]
    pub soil_temp: i64,
    #[serde(rename = "soil_moisture")]
    pub soil_moisture: f64,
    #[serde(rename = "prec_mm")]
    pub prec_mm: f64,
    #[serde(rename = "prec_prob")]
    pub prec_prob: i64,
    #[serde(rename = "prec_period")]
    pub prec_period: i64,
    pub cloudness: f64,
    #[serde(rename = "prec_type")]
    pub prec_type: i64,
    #[serde(rename = "prec_strength")]
    pub prec_strength: f64,
    pub icon: String,
    pub condition: String,
    #[serde(rename = "uv_index")]
    pub uv_index: Option<i64>,
    #[serde(rename = "feels_like")]
    pub feels_like: i64,
    pub daytime: String,
    pub polar: bool,
    #[serde(rename = "fresh_snow_mm")]
    pub fresh_snow_mm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct NightShort {
    #[serde(rename = "_source")]
    pub source: String,
    pub temp: i64,
    #[serde(rename = "wind_speed")]
    pub wind_speed: f64,
    #[serde(rename = "wind_gust")]
    pub wind_gust: f64,
    #[serde(rename = "wind_dir")]
    pub wind_dir: String,
    #[serde(rename = "pressure_mm")]
    pub pressure_mm: i64,
    #[serde(rename = "pressure_pa")]
    pub pressure_pa: i64,
    pub humidity: i64,
    #[serde(rename = "soil_temp")]
    pub soil_temp: i64,
    #[serde(rename = "soil_moisture")]
    pub soil_moisture: f64,
    #[serde(rename = "prec_mm")]
    pub prec_mm: f64,
    #[serde(rename = "prec_prob")]
    pub prec_prob: i64,
    #[serde(rename = "prec_period")]
    pub prec_period: i64,
    pub cloudness: f64,
    #[serde(rename = "prec_type")]
    pub prec_type: i64,
    #[serde(rename = "prec_strength")]
    pub prec_strength: f64,
    pub icon: String,
    pub condition: String,
    #[serde(rename = "uv_index")]
    pub uv_index: Option<i64>,
    #[serde(rename = "feels_like")]
    pub feels_like: i64,
    pub daytime: String,
    pub polar: bool,
    #[serde(rename = "fresh_snow_mm")]
    pub fresh_snow_mm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Hour {
    pub hour: String,
    #[serde(rename = "hour_ts")]
    pub hour_ts: i64,
    pub temp: i64,
    #[serde(rename = "feels_like")]
    pub feels_like: i64,
    pub icon: String,
    pub condition: String,
    pub cloudness: f64,
    #[serde(rename = "prec_type")]
    pub prec_type: i64,
    #[serde(rename = "prec_strength")]
    pub prec_strength: f64,
    #[serde(rename = "is_thunder")]
    pub is_thunder: bool,
    #[serde(rename = "wind_dir")]
    pub wind_dir: String,
    #[serde(rename = "wind_speed")]
    pub wind_speed: f64,
    #[serde(rename = "wind_gust")]
    pub wind_gust: f64,
    #[serde(rename = "pressure_mm")]
    pub pressure_mm: i64,
    #[serde(rename = "pressure_pa")]
    pub pressure_pa: i64,
    pub humidity: i64,
    #[serde(rename = "uv_index")]
    pub uv_index: i64,
    #[serde(rename = "soil_temp")]
    pub soil_temp: Option<i64>,
    #[serde(rename = "soil_moisture")]
    pub soil_moisture: Option<f64>,
    #[serde(rename = "prec_mm")]
    pub prec_mm: f64,
    #[serde(rename = "prec_period")]
    pub prec_period: i64,
    #[serde(rename = "prec_prob")]
    pub prec_prob: i64,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Biomet {
    pub index: i64,
    pub condition: String,
}
