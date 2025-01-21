use super::utils::consolidate_to_0_360;

/** One day length in milliseconds */
static DAY_MILLISECONDS: f64 = 86400000.0;
/** 1 January 2000, 12:00 UTC */
static JULIAN_DAY_Y2K: f64 = 2451545.0;
/** Julian century length in Julian days */
static JULIAN_CENTURY_IN_DAYS: f64 = 36525.0;
/** Current leap seconds and terrestial time by Wikipedia */
static LEAP_SECONDS_AND_TERRESTRIAL_TIME: f64 = 69.184;
/** 1 January 1970, 00:00 UTC */
static UNIX_CALENDAR_START_JULIAN_DAY: f64 = 2440587.5;

pub struct TimeInfo {
    pub day_start_unix_epoch: i64,
    /** Julian century based time. */
    pub julian_century: f64,
    /** Mean sidereal time in degrees. Consolidated between 0 - 360°.
     *  https://en.wikipedia.org/wiki/Sidereal_time */
    pub mean_sidereal_time: f64,
}

pub fn julian_day_to_unix_milliseconds(juldate: f64) -> i64 {
    ((juldate - UNIX_CALENDAR_START_JULIAN_DAY) * DAY_MILLISECONDS).ceil() as i64
}

fn unix_milliseconds_to_julian_day(unix_epoch: i64) -> f64 {
    /* This fixes an issue with calculation of milliseconds on day ends
     * If a timestamp is 1654905599999 then it will be converted to the next day,
     * because of some rounding during the process. Therefore we are rounding up
     * to seconds, this will prevent jumping to the next day. */
    let cleaned_milliseconds = ((unix_epoch as f64 / 1000.0).floor()) * 1000.0;
    cleaned_milliseconds / DAY_MILLISECONDS + UNIX_CALENDAR_START_JULIAN_DAY
}

pub fn init_time_info(unix_epoch: i64) -> TimeInfo {
    let current_julian_date = unix_milliseconds_to_julian_day(unix_epoch);
    let fractional_julian_day = LEAP_SECONDS_AND_TERRESTRIAL_TIME / DAY_MILLISECONDS;
    let current_julian_day = current_julian_date + fractional_julian_day;
    let current_julian_day_start = current_julian_date.floor() + 0.5;
    let day_start_unix_epoch = julian_day_to_unix_milliseconds(current_julian_day_start);
    let current_julian_century = (current_julian_date - JULIAN_DAY_Y2K) / JULIAN_CENTURY_IN_DAYS;
    let pow2_current_julian_century = current_julian_century * current_julian_century;
    // Θ
    let earth_mean_sidereal_time_deg = consolidate_to_0_360(
        280.46061837
            + (360.98564736629 * (current_julian_day_start - JULIAN_DAY_Y2K))
            + (0.000387933 * pow2_current_julian_century)
            + ((current_julian_century * pow2_current_julian_century) / 38710000.0),
    );

    println!("unix_epoch {:?}", unix_epoch);
    println!("jd {:?}", current_julian_day);
    println!("T {:?}", current_julian_century);
    println!("Θ {:?}", earth_mean_sidereal_time_deg);
    TimeInfo {
        day_start_unix_epoch,
        julian_century: current_julian_century,
        mean_sidereal_time: earth_mean_sidereal_time_deg,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_julian_date_to_unix_milliseconds_normal() {
        assert_eq!(julian_day_to_unix_milliseconds(2440587.5), 0);
        assert_eq!(julian_day_to_unix_milliseconds(2440588.0), 43200000);
        assert_eq!(julian_day_to_unix_milliseconds(2440588.5), 86400000);
        assert_eq!(julian_day_to_unix_milliseconds(2456177.541666667), 86400000);
    }

    #[test]
    fn test_julian_date_to_unix_milliseconds_negative() {
        // Negative Julian dates
        assert_eq!(julian_day_to_unix_milliseconds(2440586.5), -86400000);
    }

    #[test]
    fn meeus_example_to_milliseconds() {
        assert_eq!(julian_day_to_unix_milliseconds(2448908.5), 718934400000);
    }

    #[test]
    fn test_unix_milliseconds_to_julian_date_normal() {
        assert_eq!(unix_milliseconds_to_julian_day(0), 2440587.5);
        assert_eq!(unix_milliseconds_to_julian_day(43200000), 2440588.0);
        assert_eq!(unix_milliseconds_to_julian_day(86400000), 2440588.5);
        assert_eq!(
            unix_milliseconds_to_julian_day(1654905599999),
            2459741.499988426
        );
    }

    #[test]
    fn test_unix_milliseconds_to_julian_date_negative() {
        assert_eq!(unix_milliseconds_to_julian_day(-86400000), 2440586.5);
    }

    #[test]
    fn test_conversion_consistency() {
        let original_millisecs: i64 = 1609459200000; // January 1, 2021
        let julian_date = unix_milliseconds_to_julian_day(original_millisecs);
        let converted_millisecs = julian_day_to_unix_milliseconds(julian_date);
        assert_eq!(converted_millisecs, original_millisecs);

        let original_julian: f64 = 2459215.50; // Approx. January 1, 2021
        let millisecs = julian_day_to_unix_milliseconds(original_julian);
        let converted_julian = unix_milliseconds_to_julian_day(millisecs);
        // Use a reasonable precision for floating-point comparison
        assert!((converted_julian - original_julian).abs() < 1e-10);
    }
}
