use chrono::{DateTime, FixedOffset, Local};
use svg::{
    node::element::{Circle, Group, Line, Text},
    Node,
};

use super::{
    constants::{CANVAS_X_CENTER, CANVAS_Y_CENTER, FIFTEEN_DEGREE_IN_RAD},
    point_utils::calculate_point_on_circle,
};

fn generate_clock_lines() -> Vec<Line> {
    let mut clock_lines = Vec::new();
    for i in 0..24 {
        let w_angle = i as f32 * FIFTEEN_DEGREE_IN_RAD;
        let to_radius = if i % 3 == 0 { 80.0 } else { 85.0 };
        let from_point = calculate_point_on_circle(&w_angle, 89.0);
        let to_point = calculate_point_on_circle(&w_angle, to_radius);
        let clock_line = Line::new()
            .set("x1", from_point.0)
            .set("y1", from_point.1)
            .set("x2", to_point.0)
            .set("y2", to_point.1);
        clock_lines.push(clock_line);
    }
    clock_lines
}

pub fn generate_base_clock(requested_epoch: i64, timezone_offset: i32) -> Group {
    let inner_circle = Circle::new()
        .set("cx", CANVAS_X_CENTER)
        .set("cy", CANVAS_Y_CENTER)
        .set("r", 90);
    let outer_circle = Circle::new()
        .set("cx", CANVAS_X_CENTER)
        .set("cy", CANVAS_Y_CENTER)
        .set("r", 120);
    let clock_lines = generate_clock_lines();
    let local_timezone_offset = FixedOffset::west_opt(timezone_offset * 60).unwrap();
    let local_time_str = format!(
        "{}",
        DateTime::<Local>::from_naive_utc_and_offset(
            DateTime::from_timestamp_millis(requested_epoch)
                .unwrap()
                .naive_utc(),
            local_timezone_offset
        )
        .format("%H:%M")
    );
    let local_time_text = Text::new(local_time_str)
        .set("x", 125)
        .set("y", 180)
        .set("text-anchor", "middle")
        .set("fill", "var(--secondary-color)")
        .set("stroke", "none");
    let mut main_group = Group::new()
        .set("stroke", "var(--main-color)")
        .add(inner_circle)
        .add(outer_circle)
        .add(local_time_text);

    for c_line in clock_lines {
        main_group.append(c_line);
    }

    main_group
}
