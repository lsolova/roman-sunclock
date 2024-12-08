use crate::{
    time_calculators::get_day_start,
    wasm_types::{DayType, RomanTimeDetails},
};
use base_clock_generator::generate_base_clock;
use constants::{CANVAS_HEIGHT, CANVAS_WIDTH};
use day_night_icon_generator::generate_icon;
use point_utils::calculate_point_on_circle;
use roman_clock_generator::generate_roman_clock_group;
use std::string::String;
use svg::{node::element::Circle, Document};

mod angle_calculator;
mod base_clock_generator;
mod constants;
mod day_night_icon_generator;
mod point_utils;
mod roman_clock_generator;

pub fn generate_svg_clock(
    requested_epoch: i64,
    timezone_offset: i32,
    roman_time_details: &RomanTimeDetails,
) -> String {
    let timezone_offset_millis = match roman_time_details.day_type {
        DayType::FullDay => 0,
        DayType::FullNight => 0,
        _ => timezone_offset * 60000,
    };
    let day_start = get_day_start(requested_epoch);
    let last_change_epoch = match roman_time_details.last_sun_change {
        Some(lsc) => lsc,
        None => 0,
    };
    let next_change_epoch = match roman_time_details.next_sun_change {
        Some(nsc) => nsc,
        None => 0,
    };

    let now_angle = angle_calculator::calculate_angle_of_timestamp(
        requested_epoch - timezone_offset_millis as i64,
        day_start,
    );
    let last_change_angle = angle_calculator::calculate_angle_of_timestamp(
        last_change_epoch - timezone_offset_millis as i64,
        day_start,
    );
    let next_change_angle = angle_calculator::calculate_angle_of_timestamp(
        next_change_epoch - timezone_offset_millis as i64,
        day_start,
    );

    let now_point = calculate_point_on_circle(&now_angle, 102.0);

    let base_clock_group = generate_base_clock(requested_epoch, timezone_offset);

    let now_circle = Circle::new()
        .set("cx", now_point.0)
        .set("cy", now_point.1)
        .set("r", 4)
        .set("fill", "var(--now-color)");

    let roman_clock_group = generate_roman_clock_group(
        &roman_time_details.day_type,
        &roman_time_details.hours,
        &roman_time_details.minutes,
        &last_change_angle,
        &next_change_angle,
    );
    let icon_group = generate_icon(&roman_time_details.day_type);
    let document = Document::new()
        .set(
            "viewBox",
            format!("0 0 {:?} {:?}", CANVAS_WIDTH, CANVAS_HEIGHT),
        )
        .set("fill", "transparent")
        .add(roman_clock_group)
        .add(now_circle)
        .add(base_clock_group)
        .add(icon_group);

    let mut output: Vec<u8> = Vec::new();
    svg::write(&mut output, &document).unwrap();
    String::from_utf8(output).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::wasm_types::DayType;

    use super::*;

    #[test]
    fn simple_test() {
        println!(
            "{:?}",
            generate_svg_clock(
                1733162814475,
                -60,
                &RomanTimeDetails {
                    hours: 15,
                    minutes: 45,
                    last_sun_change: Some(1733158980000),
                    next_sun_change: Some(1733209920000),
                    day_type: DayType::NormalDay,
                    minute_length: 49.8
                }
            )
        );
    }
}
