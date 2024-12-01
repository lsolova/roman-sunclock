use std::f32;

use crate::time_calculators::get_day_start;

const FULL_CIRCLE: f32 = f32::consts::PI * 2.0;
const DAY_LENGTH: f32 = 86400000.0;

pub fn calculate_angle_of_timestamp(timestamp: i64) -> f32 {
    let day_start = get_day_start(timestamp);
    let ts_diff = (timestamp - day_start) as f32;
    (ts_diff / DAY_LENGTH) * FULL_CIRCLE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_for_day_start() {
        assert_eq!(calculate_angle_of_timestamp(1733011200000), 0.0);
    }

    #[test]
    fn calculate_for_day_end() {
        assert_eq!(calculate_angle_of_timestamp(1733097599999), FULL_CIRCLE);
    }

    #[test]
    fn calculate_for_6am() {
        assert_eq!(
            calculate_angle_of_timestamp(1733032800000),
            FULL_CIRCLE / 4.0
        );
    }
}
