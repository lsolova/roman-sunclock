use chrono::DateTime;
use std::time::{SystemTime, UNIX_EPOCH};

mod sun_calculator;
mod sun_time_calculator;
mod types;

fn main() {
    let lat = 36.6952469;
    let lon = -4.4538953;
    let alt = 0.0;

    let now = SystemTime::now();
    let current_epoch = now.duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;

    let today_sun_details = sun_calculator::calculate_sunrise_sunset(current_epoch, lat, lon, alt);
    let roman_sun_time = sun_time_calculator::calculate_roman_sun_time(current_epoch, lat, lon, alt);

    let sunrise_time = DateTime::from_timestamp_millis(today_sun_details.sunrise_epoch).expect("Invalid datetime");
    let sunset_time = DateTime::from_timestamp_millis(today_sun_details.sunset_epoch).expect("Invalid datetime");

    println!("Current result {:?} => {:?} - {:?} => {:?}", current_epoch, sunrise_time, sunset_time, roman_sun_time);
}