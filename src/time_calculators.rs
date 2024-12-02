use chrono::{DateTime, NaiveTime};

pub fn get_day_start(timestamp_millis: i64) -> i64 {
    let day_time = DateTime::from_timestamp_millis(timestamp_millis).unwrap();
    day_time
        .with_time(NaiveTime::MIN)
        .unwrap()
        .timestamp_millis()
}

pub fn get_day_end(timestamp_millis: i64) -> i64 {
    let day_time = DateTime::from_timestamp_millis(timestamp_millis).unwrap();
    day_time
        .with_time(NaiveTime::from_hms_milli_opt(23, 59, 59, 999).unwrap())
        .unwrap()
        .timestamp_millis()
}

#[cfg(test)]
mod day_start_tests {
    use super::*;

    #[test]
    fn test_get_day_with_day_start_timestamp() {
        let day_start = get_day_start(1654819200000);
        assert_eq!(day_start, 1654819200000);
    }

    #[test]
    fn test_get_day_with_within_day_timestamp() {
        let day_start = get_day_start(1654848000000);
        assert_eq!(day_start, 1654819200000);
    }

    #[test]
    fn test_get_day_with_day_end_timestamp() {
        let day_start = get_day_start(1654905599999);
        assert_eq!(day_start, 1654819200000);
    }
}

#[cfg(test)]
mod day_end_tests {
    use super::*;

    #[test]
    fn test_get_day_with_day_start_timestamp() {
        let day_end = get_day_end(1654819200000);
        assert_eq!(day_end, 1654905599999);
    }

    #[test]
    fn test_get_day_with_within_day_timestamp() {
        let day_end = get_day_end(1654848000000);
        assert_eq!(day_end, 1654905599999);
    }

    #[test]
    fn test_get_day_with_day_end_timestamp() {
        let day_end = get_day_end(1654905599999);
        assert_eq!(day_end, 1654905599999);
    }
}
