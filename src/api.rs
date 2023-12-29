use reqwest;
use serde::{Deserialize, Serialize};

use std::env;



#[derive(Serialize, Deserialize)]
struct City {
    name: String,
    lat: f64,
    lon: f64,
    country: String,
    state: String,
}

#[derive(Serialize, Deserialize)]
pub struct OneCallResponse {
    pub current: CurrentWeather,
    pub minutely: Vec<MinutelyWeather>,
    pub hourly: Vec<HourlyWeather>,
    pub daily: Vec<DailyWeather>,
}

#[derive(Serialize, Deserialize)]
pub struct CurrentWeather {
    dt: u64,
    sunrise: u64,
    sunset: u64,
    temp: f64,
    feels_like: f64,
    pressure: u64,
    humidity: u64,
    dew_point: f64,
    uvi: f64,
    clouds: u64,
    visibility: u64,
    wind_speed: f64,
    wind_deg: u64,
    weather: Vec<Weather>,
}

#[derive(Serialize, Deserialize)]
pub struct MinutelyWeather {
    pub dt: u64,
    pub precipitation: f64,
}

#[derive(Serialize, Deserialize)]
pub struct HourlyWeather {
    dt: u64,
    temp: f64,
    feels_like: f64,
    pressure: u64,
    humidity: u64,
    dew_point: f64,
    uvi: f64,
    clouds: u64,
    visibility: u64,
    wind_speed: f64,
    wind_deg: u64,
    weather: Vec<Weather>,
    pop: f64,
}

#[derive(Serialize, Deserialize)]
pub struct DailyWeather {
    dt: u64,
    sunrise: u64,
    sunset: u64,
    temp: DayTemp,
    feels_like: DayFeelsLike,
    pressure: u64,
    humidity: u64,
    dew_point: f64,
    wind_speed: f64,
    wind_deg: u64,
    weather: Vec<Weather>,
    clouds: u64,
    pop: f64,
    uvi: f64,
}

#[derive(Serialize, Deserialize)]
pub struct DayTemp {
    day: f64,
    min: f64,
    max: f64,
    night: f64,
    eve: f64,
    morn: f64,
}

#[derive(Serialize, Deserialize)]
pub struct DayFeelsLike {
    day: f64,
    night: f64,
    eve: f64,
    morn: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Weather {
    id: u64,
    main: String,
    description: String,
    icon: String,
}




pub fn get_response(city: &str) -> Result<OneCallResponse, reqwest::Error> {
    let api_key = env::var("OPENWEATHERMAP_API_KEY")
        .expect("OPENWEATHERMAP_API_KEY not set");

    let (lat, lon) = city_to_lat_lon(city)?;


    let url = format!("https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&appid={}", 
                        lat, lon, api_key);

    let response = reqwest::blocking::get(&url)?;

    let response: OneCallResponse = response.json()?;

    Ok(response)
}

fn city_to_lat_lon(city: &str) -> Result<(f64, f64), reqwest::Error> {
    let api_key = env::var("OPENWEATHERMAP_API_KEY")
        .expect("OPENWEATHERMAP_API_KEY not set");
    let url = format!("http://api.openweathermap.org/geo/1.0/direct?q={}&limit=3&appid={}",
                        city, api_key);

    let response = reqwest::blocking::get(&url)?;

    let cities: Vec<City> = response.json()?;

    let data = &cities[0];

    Ok((data.lat, data.lon))
}

