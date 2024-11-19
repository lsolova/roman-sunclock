static DAY_MILLISECONDS: f64 = 86400000.0;
/** Source: https://en.wikipedia.org/wiki/Julian_day */
static JULIAN_DATE_DIFFERENCE: f64 = 2440587.5;

pub fn julian_date_to_unix_milliseconds(juldate: f64) -> i64 {
    ((juldate - JULIAN_DATE_DIFFERENCE) * DAY_MILLISECONDS).ceil() as i64
}

pub fn unix_milliseconds_to_julian_date(millisecs: i64) -> f64 {
    millisecs as f64 / DAY_MILLISECONDS + JULIAN_DATE_DIFFERENCE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_julian_date_to_unix_milliseconds_normal() {
        assert_eq!(julian_date_to_unix_milliseconds(2440587.5), 0);
        assert_eq!(julian_date_to_unix_milliseconds(2440588.0), 43200000);
        assert_eq!(julian_date_to_unix_milliseconds(2440588.5), 86400000);
    }

    #[test]
    fn test_julian_date_to_unix_milliseconds_negative() {
        // Negative Julian dates
        assert_eq!(julian_date_to_unix_milliseconds(2440586.5), -86400000);
    }

    #[test]
    fn test_unix_milliseconds_to_julian_date_normal() {
        assert_eq!(unix_milliseconds_to_julian_date(0), 2440587.5);
        assert_eq!(unix_milliseconds_to_julian_date(43200000), 2440588.0);
        assert_eq!(unix_milliseconds_to_julian_date(86400000), 2440588.5);
    }

    #[test]
    fn test_unix_milliseconds_to_julian_date_negative() {
        assert_eq!(unix_milliseconds_to_julian_date(-86400000), 2440586.5);
    }

    #[test]
    fn test_conversion_consistency() {
        let original_millisecs: i64 = 1609459200000; // January 1, 2021
        let julian_date = unix_milliseconds_to_julian_date(original_millisecs);
        let converted_millisecs = julian_date_to_unix_milliseconds(julian_date);
        assert_eq!(converted_millisecs, original_millisecs);

        let original_julian: f64 = 2459215.50; // Approx. January 1, 2021
        let millisecs = julian_date_to_unix_milliseconds(original_julian);
        let converted_julian = unix_milliseconds_to_julian_date(millisecs);
        // Use a reasonable precision for floating-point comparison
        assert!((converted_julian - original_julian).abs() < 1e-10);
    }
}
