use crate::patterns::condition_patterns::*;

pub fn select_condition(condition: &str) -> [&str; 6] {
    match condition {
        "clear" => CONDITION_CLEAR,

        "partly-cloudy" => CONDITION_CLOUDY,
        "cloudy" => CONDITION_CLOUDY,
        "overcast" => CONDITION_CLOUDY,

        "light-rain" => LIGHT_CONDITION_RAIN,
        "rain" => CONDITION_RAIN,
        "heavy-rain" => HEAVY_CONDITION_RAIN,
        "showers" => SHOWERS_CONDITION_RAIN,

        "wet-snow" => CONDITION_WET_SNOW,

        "light-snow" => LIGHT_CONDITION_SNOW,
        "snow" => CONDITION_SNOW,
        "snow-showers" => HEAVY_CONDITION_SNOW,

        "hail" => HAIL_CONDITION_SNOW,

        "thunderstorm" => CONDITION_THUNDERSTORM,
        "thunderstorm-with-rain " => CONDITION_THUNDERSTORM_WITH_RAIN,
        "thunderstorm-with-hail" => CONDITION_THUNDERSTORM_WITH_HAIL,

        _ => CONDITION_CLOUDY,
    }
}
