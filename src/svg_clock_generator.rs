use core::f32;
use std::string::String;
use svg::{
    node::element::{path::Data, Circle, Group, Line, Path},
    Document, Node,
};

mod angle_calculator;

const CANVAS_WIDTH: f32 = 250.0;
const CANVAS_HEIGHT: f32 = 250.0;
const CANVAS_X_CENTER: f32 = CANVAS_WIDTH / 2.0;
const CANVAS_Y_CENTER: f32 = CANVAS_HEIGHT / 2.0;
const NINETY_DEGREE_IN_RAD: f32 = f32::consts::PI / 2.0;
const FIFTEEN_DEGREE_IN_RAD: f32 = f32::consts::PI / 12.0;
const FULL_CIRCLE_IN_RAD: f32 = f32::consts::PI * 2.0;

fn calculate_point_on_circle(angle: f32, radius: f32) -> (f32, f32) {
    let x = (angle - NINETY_DEGREE_IN_RAD).cos() * radius + CANVAS_X_CENTER;
    let y = (angle - NINETY_DEGREE_IN_RAD).sin() * radius + CANVAS_Y_CENTER;
    (x, y)
}
fn generate_clock_lines() -> Vec<Line> {
    let mut clock_lines = Vec::new();
    for i in 0..24 {
        let w_angle = i as f32 * FIFTEEN_DEGREE_IN_RAD;
        let to_radius = if i % 3 == 0 { 80.0 } else { 85.0 };
        let from_point = calculate_point_on_circle(w_angle, 89.0);
        let to_point = calculate_point_on_circle(w_angle, to_radius);
        let clock_line = Line::new()
            .set("x1", from_point.0)
            .set("y1", from_point.1)
            .set("x2", to_point.0)
            .set("y2", to_point.1);
        clock_lines.push(clock_line);
    }
    clock_lines
}

fn generate_roman_clock_lines(initial_angle: f32, step: f32) -> Vec<Line> {
    let mut roman_clock_lines = Vec::new();
    for i in 1..12 {
        let w_angle = i as f32 * step + initial_angle;
        let to_radius = if i % 3 == 0 { 107.0 } else { 113.0 };
        let from_point = calculate_point_on_circle(w_angle, 119.0);
        let to_point = calculate_point_on_circle(w_angle, to_radius);
        let clock_line = Line::new()
            .set("x1", from_point.0)
            .set("y1", from_point.1)
            .set("x2", to_point.0)
            .set("y2", to_point.1);
        roman_clock_lines.push(clock_line);
    }
    return roman_clock_lines;
}

pub fn generate_svg_clock(
    now_time: i64,
    sunrise_epoch: i64,
    sunset_epoch: i64,
    timezone_offset: i64,
) -> String {
    let timezone_offset_millis = timezone_offset * 60000;
    let now_angle =
        angle_calculator::calculate_angle_of_timestamp((now_time - timezone_offset_millis) as i64);
    let sunrise_angle = angle_calculator::calculate_angle_of_timestamp(
        (sunrise_epoch - timezone_offset_millis) as i64,
    );
    let sunset_angle = angle_calculator::calculate_angle_of_timestamp(
        (sunset_epoch - timezone_offset_millis) as i64,
    );

    let now_point = calculate_point_on_circle(now_angle, 102.0);
    let sunrise_point = calculate_point_on_circle(sunrise_angle, 105.0);
    let sunset_point = calculate_point_on_circle(sunset_angle, 105.0);
    let is_day_longer = f32::consts::PI < sunset_angle - sunrise_angle;

    let day_arc_selector = if is_day_longer { (1, 1) } else { (0, 1) };
    let night_arc_selector = if is_day_longer { (0, 1) } else { (1, 1) };

    let night_path_data = Data::new().move_to(sunset_point).elliptical_arc_to((
        105,
        105,
        0,
        night_arc_selector.0,
        night_arc_selector.1,
        sunrise_point.0,
        sunrise_point.1,
    ));
    let night_path = Path::new()
        .set("stroke", "var(--night-color)")
        .set("stroke-width", "30")
        .set("d", night_path_data);
    let day_path_data = Data::new().move_to(sunrise_point).elliptical_arc_to((
        105,
        105,
        0,
        day_arc_selector.0,
        day_arc_selector.1,
        sunset_point.0,
        sunset_point.1,
    ));
    let day_path = Path::new()
        .set("stroke", "var(--day-color)")
        .set("stroke-width", "30")
        .set("d", day_path_data);

    let inner_circle = Circle::new()
        .set("cx", CANVAS_X_CENTER)
        .set("cy", CANVAS_Y_CENTER)
        .set("r", 90);
    let outer_circle = Circle::new()
        .set("cx", CANVAS_X_CENTER)
        .set("cy", CANVAS_Y_CENTER)
        .set("r", 120);
    let clock_lines = generate_clock_lines();
    let mut main_group = Group::new()
        .set("stroke", "var(--main-color)")
        .add(inner_circle)
        .add(outer_circle);
    for c_line in clock_lines {
        main_group.append(c_line);
    }

    let day_roman_clock_lines =
        generate_roman_clock_lines(sunrise_angle, (sunset_angle - sunrise_angle) / 12.0);
    let mut day_roman_lines_group = Group::new().set("stroke", "var(--night-color)");
    for drc_line in day_roman_clock_lines {
        day_roman_lines_group.append(drc_line);
    }

    let night_roman_clock_lines = generate_roman_clock_lines(
        sunset_angle,
        (FULL_CIRCLE_IN_RAD - (sunset_angle - sunrise_angle)) / 12.0,
    );
    let mut night_roman_lines_group = Group::new().set("stroke", "var(--day-color)");
    for nr_line in night_roman_clock_lines {
        night_roman_lines_group.append(nr_line);
    }

    let now_circle = Circle::new()
        .set("cx", now_point.0)
        .set("cy", now_point.1)
        .set("r", 4)
        .set("fill", "red");

    let document = Document::new()
        .set(
            "viewBox",
            format!("0 0 {:?} {:?}", CANVAS_WIDTH, CANVAS_HEIGHT),
        )
        .set("fill", "transparent")
        .add(night_path)
        .add(day_path)
        .add(main_group)
        .add(day_roman_lines_group)
        .add(night_roman_lines_group)
        .add(now_circle);

    let mut output: Vec<u8> = Vec::new();
    svg::write(&mut output, &document).unwrap();
    String::from_utf8(output).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        println!(
            "{:?}",
            generate_svg_clock(1733162814475, 1733158980000, 1733209920000, -60)
        );
    }
}
