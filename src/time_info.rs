use chrono::{DateTime, NaiveTime};

/** One day length in milliseconds */
static DAY_MILLISECONDS: f64 = 86400000.0;
/** 1 January 2000, 12:00 UTC */
pub static JULIAN_DAY_Y2K: f64 = 2451545.0;
/** Julian century length in Julian days */
static JULIAN_CENTURY_IN_JULIAN_DAYS: f64 = 36525.0;
/** Current leap seconds and terrestial time by Wikipedia
 *  Source: https://en.wikipedia.org/wiki/Sunrise_equation#Calculate_current_Julian_day
 */
static LEAP_SECONDS_AND_TERRESTRIAL_TIME: f64 = 69.184;
/** 1 January 1970, 00:00 UTC */
static UNIX_CALENDAR_START_JULIAN_DAY: f64 = 2440587.5;

fn get_day_start(current_unix_epoch: i64) -> i64 {
    let day_time = DateTime::from_timestamp_millis(current_unix_epoch).unwrap();
    day_time
        .with_time(NaiveTime::MIN)
        .unwrap()
        .timestamp_millis()
}

fn unix_milliseconds_to_julian_day(unix_epoch: i64) -> f64 {
    /* This fixes an issue with calculation of milliseconds on day ends
     * If a timestamp is 1654905599999 then it will be converted to the next day,
     * because of some rounding during the process. Therefore we are rounding up
     * to seconds, this will prevent jumping to the next day. */
    let cleaned_milliseconds = ((unix_epoch as f64 / 1000.0).floor()) * 1000.0;
    cleaned_milliseconds / DAY_MILLISECONDS + UNIX_CALENDAR_START_JULIAN_DAY
}

fn julian_day_to_century(j_date: f64) -> f64 {
    (j_date - JULIAN_DAY_Y2K) / JULIAN_CENTURY_IN_JULIAN_DAYS
}

pub struct TimeInfo {
    pub current_julian_day: f64,
    pub current_julian_century: f64,
    pub current_unix_epoch: i64,
    pub day_start_julian_day: f64,
    pub day_start_unix_epoch: i64,
    pub timezone_offset: i32,
}

impl TimeInfo {
    pub fn set_timezone_offset(&self, timezone_offset: i32) -> Self {
        TimeInfo {
            current_julian_century: self.current_julian_century,
            current_julian_day: self.current_julian_day,
            current_unix_epoch: self.current_unix_epoch,
            day_start_julian_day: self.day_start_julian_day,
            day_start_unix_epoch: self.day_start_unix_epoch,
            timezone_offset
        }
    }
}

/** Create TimeInfo from unix epoch. */
impl From<i64> for TimeInfo {
    fn from(current_unix_epoch: i64) -> Self {
        let current_julian_date = unix_milliseconds_to_julian_day(current_unix_epoch);
        let fractional_julian_day = LEAP_SECONDS_AND_TERRESTRIAL_TIME / DAY_MILLISECONDS;
        let current_j2000_julian_day = (current_julian_date - JULIAN_DAY_Y2K + fractional_julian_day).round();
        let current_j2000_julian_century = julian_day_to_century(current_j2000_julian_day);
        let day_start_julian_day = current_j2000_julian_day.floor() + 0.5;
        let day_start_unix_epoch = get_day_start(current_unix_epoch);

        TimeInfo {
            current_julian_century: current_j2000_julian_century,
            current_julian_day: current_j2000_julian_day,
            current_unix_epoch,
            day_start_julian_day,
            day_start_unix_epoch,
            timezone_offset: 0
        }
    }
}

#[cfg(test)]
mod day_start_tests {
    use super::*;

    #[test]
    fn test_get_day_with_day_start_timestamp() {
        let day_start = TimeInfo::from(1654819200000).day_start_unix_epoch;
        assert_eq!(day_start, 1654819200000);
    }

    #[test]
    fn test_get_day_with_within_day_timestamp() {
        let day_start = TimeInfo::from(1654848000000).day_start_unix_epoch;
        assert_eq!(day_start, 1654819200000);
    }

    #[test]
    fn test_get_day_with_day_end_timestamp() {
        let day_start = TimeInfo::from(1654905599999).day_start_unix_epoch;
        assert_eq!(day_start, 1654819200000);
    }

    #[test]
    fn test_unix_milliseconds_to_julian_date_normal() {
        assert_eq!(TimeInfo::from(0).current_julian_day, 2440587.5);
        assert_eq!(TimeInfo::from(43200000).current_julian_day, 2440588.0);
        assert_eq!(TimeInfo::from(86400000).current_julian_day, 2440588.5);
        assert_eq!(
            TimeInfo::from(1654905599999).current_julian_day,
            2459741.499988426
        );
    }

    #[test]
    fn test_unix_milliseconds_to_julian_date_negative() {
        assert_eq!(TimeInfo::from(-86400000).current_julian_day, 2440586.5);
    }
}
