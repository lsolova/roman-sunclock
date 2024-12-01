use wasm_bindgen::prelude::*;

mod angle_calculator;
mod sun_calculator;
mod sun_time_calculator;
mod time_calculators;
mod types;

#[wasm_bindgen]
pub fn roman_sun_time(current_epoch: i64, lat: f32, lon: f32, alt: f32) -> types::RomanTimeDetails {
    sun_time_calculator::calculate_roman_sun_time(current_epoch, lat, lon, alt)
}

#[wasm_bindgen]
pub fn angle_of_timestamp(timestamp: i64) -> f32 {
    angle_calculator::calculate_angle_of_timestamp(timestamp)
}
