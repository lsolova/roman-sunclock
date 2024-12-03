use svg_clock_generator::generate_svg_clock;
use types::RomanSunclockResult;
use wasm_bindgen::prelude::*;

mod sun_calculator;
mod sun_time_calculator;
mod svg_clock_generator;
mod time_calculators;
mod types;

#[wasm_bindgen]
pub fn roman_sun_time(
    current_epoch: i64,
    timezone_offset: i64,
    lat: f32,
    lon: f32,
    alt: f32,
) -> types::RomanSunclockResult {
    let time_details = sun_time_calculator::calculate_roman_sun_time(current_epoch, lat, lon, alt);

    let clock_svg = generate_svg_clock(current_epoch, timezone_offset, &time_details);
    RomanSunclockResult {
        time_details,
        clock_svg,
    }
}
