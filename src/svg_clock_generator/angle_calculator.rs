use std::f32;

const FULL_CIRCLE: f32 = f32::consts::PI * 2.0;
const DAY_LENGTH: f32 = 86400000.0;

pub fn calculate_angle_of_timestamp(timestamp: i64, day_start: i64) -> f32 {
    let ts_diff = (timestamp - day_start) as f32;
    (ts_diff / DAY_LENGTH) * FULL_CIRCLE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_for_day_start() {
        assert_eq!(
            calculate_angle_of_timestamp(1733011200000, 1733011200000),
            0.0
        );
    }

    #[test]
    fn calculate_for_day_end() {
        assert_eq!(
            calculate_angle_of_timestamp(1733097599999, 1733011200000),
            FULL_CIRCLE
        );
    }

    #[test]
    fn calculate_for_6am() {
        assert_eq!(
            calculate_angle_of_timestamp(1733032800000, 1733011200000),
            FULL_CIRCLE / 4.0
        );
    }
}
