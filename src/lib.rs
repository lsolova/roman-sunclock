use wasm_bindgen::prelude::*;

mod sun_calculator;
mod sun_time_calculator;
mod types;

#[wasm_bindgen]
pub fn roman_sun_time(current_epoch: i64, lat: f32, lon: f32, alt: f32) -> types::RomanTimeDetails {
    sun_time_calculator::calculate_roman_sun_time(current_epoch, lat, lon, alt)
}
