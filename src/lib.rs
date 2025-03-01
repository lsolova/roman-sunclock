use svg_clock_generator::generate_svg_clock;
use time_info::TimeInfo;
use types::GeoLocation;
use wasm_bindgen::prelude::*;
use wasm_types::RomanSunclockResult;

mod julian_converters;
mod rise_set;
mod sun_calculator;
mod sun_time_calculator;
mod svg_clock_generator;
mod time_info;
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
    let time_info = TimeInfo::from(requested_epoch).set_timezone_offset(timezone_offset);
    let location = GeoLocation::new(lat, lon, alt);
    let roman_time_details = sun_time_calculator::calculate_roman_sun_time(&time_info, &location);

    let clock_svg = generate_svg_clock(&time_info, &roman_time_details);
    RomanSunclockResult {
        time_details: roman_time_details,
        clock_svg,
    }
}
