use svg_clock_generator::generate_svg_clock;
use wasm_types::RomanSunclockResult;
use wasm_bindgen::prelude::*;

mod celestial_calculations;
mod sun_calculator;
mod sun_time_calculator;
mod svg_clock_generator;
mod time_calculators;
mod types;
mod wasm_types;

#[wasm_bindgen]
pub fn roman_sun_time(
    requested_epoch: i64,
    timezone_offset: i32,
    lat: f32,
    lon: f32,
    alt: f32,
) -> RomanSunclockResult {
    let time_details = sun_time_calculator::calculate_roman_sun_time(requested_epoch, lat, lon, alt);

    let clock_svg = generate_svg_clock(requested_epoch, timezone_offset, &time_details);
    RomanSunclockResult {
        time_details,
        clock_svg,
    }
}
