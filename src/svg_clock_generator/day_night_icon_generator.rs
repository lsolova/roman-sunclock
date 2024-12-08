use core::f32;
use svg::{
    node::element::{path::Data, Circle, Group, Line, Path},
    Node,
};

use crate::wasm_types::DayType;

use super::constants::{
    CANVAS_X_CENTER, FIFTEEN_DEGREE_IN_RAD, FORTYFIVE_DEGREE_IN_RAD, NINETY_DEGREE_IN_RAD,
};
use super::point_utils::calculate_point_on_circle_with_center;

fn generate_sun_icon() -> Group {
    let mut sun_group = Group::new()
        .set("stroke-width", 3.75)
        .set("stroke-linecap", "round")
        .set("stroke", "var(--main-color)");
    let sun_circle = Circle::new()
        .set("cx", CANVAS_X_CENTER)
        .set("cy", 96)
        .set("r", 11);
    for i in 0..8 {
        let point_one = calculate_point_on_circle_with_center(
            &(i as f32 * FORTYFIVE_DEGREE_IN_RAD),
            18.0,
            CANVAS_X_CENTER,
            96.0,
        );
        let point_two = calculate_point_on_circle_with_center(
            &(i as f32 * FORTYFIVE_DEGREE_IN_RAD),
            22.0,
            CANVAS_X_CENTER,
            96.0,
        );
        let sun_shine_line = Line::new()
            .set("x1", point_one.0)
            .set("y1", point_one.1)
            .set("x2", point_two.0)
            .set("y2", point_two.1);
        sun_group.append(sun_shine_line);
    }
    sun_group.append(sun_circle);
    sun_group
}

fn generate_moon_icon() -> Group {
    let moon_point_1 = calculate_point_on_circle_with_center(
        &(-FIFTEEN_DEGREE_IN_RAD),
        17.0,
        CANVAS_X_CENTER,
        96.0,
    );
    let moon_point_2 = calculate_point_on_circle_with_center(
        &(NINETY_DEGREE_IN_RAD + FIFTEEN_DEGREE_IN_RAD),
        17.0,
        CANVAS_X_CENTER,
        96.0,
    );
    let moon_path_data = Data::new()
        .move_to(moon_point_1)
        .elliptical_arc_to((17, 17, 0, 1, 0, moon_point_2.0, moon_point_2.1))
        .elliptical_arc_to((17, 17, 0, 0, 1, moon_point_1.0, moon_point_1.1));
    let moon_path = Path::new().set("d", moon_path_data);
    let moon_group = Group::new()
        .set("stroke-width", 3.75)
        .set("stroke-linecap", "round")
        .set("stroke-linejoin", "round")
        .set("stroke", "var(--main-color)")
        .add(moon_path);
    return moon_group;
}

pub fn generate_icon(day_type: &DayType) -> Group {
    match day_type {
        DayType::FullDay => generate_sun_icon(),
        DayType::FullNight => generate_moon_icon(),
        DayType::NormalDay => generate_sun_icon(),
        DayType::NormalNight => generate_moon_icon(),
        DayType::__Invalid => Group::new(),
    }
}
