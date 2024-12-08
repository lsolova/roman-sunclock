use svg::{
    node::element::{
        path::{Command, Data, Parameters, Position},
        Circle, Group, Line, Path, Text,
    },
    Node,
};

use crate::wasm_types::DayType;

use super::{
    constants::{
        CANVAS_X_CENTER, CANVAS_Y_CENTER, FULL_CIRCLE_IN_RAD, NINETY_DEGREE_IN_RAD,
        THIRTY_DEGREE_IN_RAD,
    },
    point_utils::calculate_point_on_circle,
};

fn format_time_to_hours_minutes(hours: &i32, minutes: &i32) -> String {
    format!("{:0>2}:{:0>2}", hours, minutes)
}

pub fn generate_roman_clock_lines_group(
    initial_angle: &f32,
    step: f32,
    last_item: i32,
    stroke: &str,
) -> Group {
    let mut roman_clock_lines_group = Group::new().set("stroke", stroke);
    for i in 0..last_item {
        let w_angle = i as f32 * step + initial_angle;
        let to_radius = if i % 3 == 0 { 107.0 } else { 113.0 };
        let from_point = calculate_point_on_circle(&w_angle, 119.0);
        let to_point = calculate_point_on_circle(&w_angle, to_radius);
        let clock_line = Line::new()
            .set("x1", from_point.0)
            .set("y1", from_point.1)
            .set("x2", to_point.0)
            .set("y2", to_point.1);
        roman_clock_lines_group.append(clock_line);
    }
    roman_clock_lines_group
}

fn generate_full_clock(circle_stroke: &str, lines_stroke: &str) -> (Circle, Group) {
    let roman_clock_full_circle = Circle::new()
        .set("cx", CANVAS_X_CENTER)
        .set("cy", CANVAS_Y_CENTER)
        .set("r", 105)
        .set("stroke", circle_stroke)
        .set("stroke-width", "30");
    let roman_clock_lines_group =
        generate_roman_clock_lines_group(&0.0, THIRTY_DEGREE_IN_RAD, 13, lines_stroke);
    (roman_clock_full_circle, roman_clock_lines_group)
}

fn generate_normal_clock(
    day_type: &DayType,
    last_change_angle: &f32,
    next_change_angle: &f32,
) -> (Group, Group) {
    let last_change_point = calculate_point_on_circle(last_change_angle, 105.0);
    let next_change_point = calculate_point_on_circle(next_change_angle, 105.0);
    let next_change_end_angle = last_change_angle + FULL_CIRCLE_IN_RAD;
    let last_stroke = if day_type == &DayType::NormalDay {
        "var(--day-color)"
    } else {
        "var(--night-color)"
    };
    let next_stroke = if day_type == &DayType::NormalDay {
        "var(--night-color)"
    } else {
        "var(--day-color)"
    };

    let mut last_change_arc_data = Data::new().move_to(last_change_point);
    let mut next_change_arc_data = Data::new().move_to(next_change_point);

    for i in 0..8 {
        let segment_angle = i as f32 * NINETY_DEGREE_IN_RAD;
        if last_change_angle < &segment_angle && &segment_angle < next_change_angle {
            // Add to last change arc
            let segment_angle_point = calculate_point_on_circle(&segment_angle, 105.0);
            last_change_arc_data.append(Command::EllipticalArc(
                Position::Absolute,
                Parameters::from((
                    105,
                    105,
                    0,
                    0,
                    1,
                    segment_angle_point.0,
                    segment_angle_point.1,
                )),
            ));
        } else if next_change_angle < &segment_angle && segment_angle < next_change_end_angle {
            // Add to next change arc
            let segment_angle_point = calculate_point_on_circle(&segment_angle, 105.0);
            next_change_arc_data.append(Command::EllipticalArc(
                Position::Absolute,
                Parameters::from((
                    105,
                    105,
                    0,
                    0,
                    1,
                    segment_angle_point.0,
                    segment_angle_point.1,
                )),
            ));
        }
    }

    last_change_arc_data.append(Command::EllipticalArc(
        Position::Absolute,
        Parameters::from((105, 105, 0, 0, 1, next_change_point.0, next_change_point.1)),
    ));
    next_change_arc_data.append(Command::EllipticalArc(
        Position::Absolute,
        Parameters::from((105, 105, 0, 0, 1, last_change_point.0, last_change_point.1)),
    ));

    let last_change_arc_path = Path::new()
        .set("stroke", last_stroke)
        .set("stroke-width", "30")
        .set("d", last_change_arc_data);
    let next_change_arc_path = Path::new()
        .set("stroke", next_stroke)
        .set("stroke-width", "30")
        .set("d", next_change_arc_data);

    let last_change_lines_step = (next_change_angle - last_change_angle) / 12.0;
    let last_change_lines_group = generate_roman_clock_lines_group(
        last_change_angle,
        last_change_lines_step,
        12,
        &next_stroke,
    );
    let next_change_lines_step = (next_change_end_angle - next_change_angle) / 12.0;
    let next_change_lines_group = generate_roman_clock_lines_group(
        next_change_angle,
        next_change_lines_step,
        12,
        &last_stroke,
    );

    let last_change_group = Group::new()
        .add(last_change_arc_path)
        .add(last_change_lines_group);
    let next_change_group = Group::new()
        .add(next_change_arc_path)
        .add(next_change_lines_group);
    (last_change_group, next_change_group)
}

pub fn generate_roman_clock_group(
    day_type: &DayType,
    hours: &i32,
    minutes: &i32,
    last_change_angle: &f32,
    next_change_angle: &f32,
) -> Group {
    let mut roman_clock_group = Group::new();
    match day_type {
        DayType::FullDay => {
            let (day_circle, day_lines) =
                generate_full_clock("var(--day-color)", "var(--night-color)");

            roman_clock_group.append(day_circle);
            roman_clock_group.append(day_lines);
        }
        DayType::FullNight => {
            let (night_circle, night_lines) =
                generate_full_clock("var(--night-color)", "var(--day-color)");

            roman_clock_group.append(night_circle);
            roman_clock_group.append(night_lines);
        }
        _ => {
            let (last_change_group, next_change_group) =
                generate_normal_clock(day_type, last_change_angle, next_change_angle);
            roman_clock_group.append(last_change_group);
            roman_clock_group.append(next_change_group);
        }
    }

    let roman_time_str = format_time_to_hours_minutes(hours, minutes);
    let roman_time_text = Text::new(roman_time_str)
        .set("x", 125)
        .set("y", 152)
        .set("text-anchor", "middle")
        .set("fill", "var(--main-color)")
        .set("stroke", "none");
    roman_clock_group.append(roman_time_text);

    roman_clock_group
}
