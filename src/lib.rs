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

    let sunrise_epoch = if time_details.is_day {
        time_details.last_sun_change
    } else {
        time_details.next_sun_change
    };
    let sunset_epoch = if time_details.is_day {
        time_details.next_sun_change
    } else {
        time_details.last_sun_change
    };

    let clock_svg = generate_svg_clock(current_epoch, sunrise_epoch, sunset_epoch, timezone_offset);
    RomanSunclockResult {
        time_details,
        clock_svg,
    }
}
