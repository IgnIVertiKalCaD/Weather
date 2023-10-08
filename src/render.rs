use crate::structures::weather_info::WeatherInfo;
use crate::utils::{unixtime_convertor::time, select_condition::select_condition};
pub fn render(weather: &WeatherInfo, condition: &String) {
    let condition: [&str; 6] = select_condition(condition);

    print!("{}", condition[0]);
    print!("Weather: {} \n", weather.fact.condition);

    print!("{}", condition[1]);
    print!("Temperature: {} \n", weather.fact.temp);

    print!("{}", condition[2]);
    print!("Temperature feels like: {} \n", weather.fact.feels_like);

    print!("{}", condition[3]);
    print!("Sunrise: {} \n", weather.forecasts[0].sunrise);

    print!("{}", condition[4]);
    print!("Sunset: {} \n", weather.forecasts[0].sunset);

    print!("{}", condition[5]);
    print!("Time: {} \n", time(weather.now, weather.info.tzinfo.offset));
}
