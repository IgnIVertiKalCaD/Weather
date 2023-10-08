use crate::patterns::condition_patterns::{CONDITION_CLEAR, CONDITION_CLOUDY, CONDITION_RAIN, CONDITION_SNOW, CONDITION_THUNDERSTORM};

pub fn select_condition(condition: &str) -> [&str; 6] {
    match condition {
        "clear" => CONDITION_CLEAR,

        "overcast" => CONDITION_CLOUDY,
        "cloudy" => CONDITION_CLOUDY,

        "rain" => CONDITION_RAIN,

        "snow" => CONDITION_SNOW,

        "thunderstorm" => CONDITION_THUNDERSTORM,

        _ => CONDITION_CLOUDY,
    }
}
