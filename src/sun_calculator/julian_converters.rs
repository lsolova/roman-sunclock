static DAY_MILLISECONDS: f64 = 86400000.0;
static UNIX_TIMESTAMP_SHIFT: f64 = 2440587.5;

pub fn julian_date_to_unix_milliseconds(juldate: f64) -> i64 {
    ((juldate - UNIX_TIMESTAMP_SHIFT) * DAY_MILLISECONDS).ceil() as i64
}

pub fn unix_milliseconds_to_julian_date(millisecs: i64) -> f64 {
    millisecs as f64 / DAY_MILLISECONDS + UNIX_TIMESTAMP_SHIFT
}
