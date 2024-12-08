use super::constants::{CANVAS_X_CENTER, CANVAS_Y_CENTER, NINETY_DEGREE_IN_RAD};

pub fn calculate_point_on_circle_with_center(
    angle: &f32,
    radius: f32,
    x_center: f32,
    y_center: f32,
) -> (f32, f32) {
    let x = (angle - NINETY_DEGREE_IN_RAD).cos() * radius + x_center;
    let y = (angle - NINETY_DEGREE_IN_RAD).sin() * radius + y_center;
    (x, y)
}

pub fn calculate_point_on_circle(angle: &f32, radius: f32) -> (f32, f32) {
    calculate_point_on_circle_with_center(angle, radius, CANVAS_X_CENTER, CANVAS_Y_CENTER)
}
