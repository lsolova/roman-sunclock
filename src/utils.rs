/** Astronomical unit in km. */
pub const AU: f64 = 149597870.0;
/** π */
pub const PI: f64 = std::f64::consts::PI;
/** 2π */
pub const PI2: f64 = 2.0 * PI;

pub fn normalize_to_circle_radians(original_radian: f64) -> f64 {
    let remaining = original_radian % PI2;
    if remaining < 0.0 {
        remaining + PI2
    } else {
        remaining
    }
}
