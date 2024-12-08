use crate::sun_calculator::calculate_sunrise_sunset;
use crate::time_calculators::get_day_start;
use crate::types::{PointOfTime, SunMovementResult, TimeType, Timeline};
use crate::wasm_types::{DayType, RomanTimeDetails};
use std::cmp::{max, min};

static DAY_MILLISECONDS: i64 = 86400000;

fn build_timeline(requested_epoch: i64, lat: f32, lon: f32, alt: f32) -> Timeline {
    let mut day_type;

    let day_start_epoch = get_day_start(requested_epoch);
    let mut last_sun_change: Option<PointOfTime> = None;
    let mut next_sun_change: Option<PointOfTime> = None;

    match calculate_sunrise_sunset(requested_epoch, lat, lon, alt) {
        SunMovementResult::NormalDayAndNight(n) => {
            // Get the closest to requested (max)
            let mut last_sun_change_epoch = i64::MIN;
            let mut last_sun_change_time_type = TimeType::Sunrise;
            if n.sunrise_epoch < requested_epoch && last_sun_change_epoch < n.sunrise_epoch {
                last_sun_change_epoch = n.sunrise_epoch;
            }
            if n.sunset_epoch < requested_epoch && last_sun_change_epoch < n.sunset_epoch {
                last_sun_change_epoch = n.sunset_epoch;
                last_sun_change_time_type = TimeType::Sunset;
            }

            if i64::MIN < last_sun_change_epoch {
                last_sun_change = Some(PointOfTime {
                    time_type: last_sun_change_time_type,
                    epoch: last_sun_change_epoch,
                })
            }

            // Get the closest (min)
            let mut next_sun_change_epoch = i64::MAX;
            let mut next_sun_change_time_type = TimeType::Sunrise;
            if requested_epoch < n.sunrise_epoch && n.sunrise_epoch < next_sun_change_epoch {
                next_sun_change_epoch = n.sunrise_epoch;
            }
            if requested_epoch < n.sunset_epoch && n.sunset_epoch < next_sun_change_epoch {
                next_sun_change_epoch = n.sunset_epoch;
                next_sun_change_time_type = TimeType::Sunset;
            }
            if next_sun_change_epoch < i64::MAX {
                next_sun_change = Some(PointOfTime {
                    time_type: next_sun_change_time_type,
                    epoch: next_sun_change_epoch,
                })
            }

            day_type = DayType::NormalDay;
        }
        SunMovementResult::FullDay => {
            day_type = DayType::FullDay;
        }
        SunMovementResult::FullNight => {
            day_type = DayType::FullNight;
        }
    };

    // Read yesterday if needed
    let check_yesterday = if let Some(v) = last_sun_change {
        requested_epoch < v.epoch
    } else {
        true
    };
    if check_yesterday {
        match calculate_sunrise_sunset(requested_epoch - DAY_MILLISECONDS, lat, lon, alt) {
            SunMovementResult::NormalDayAndNight(n) => {
                last_sun_change = Some(PointOfTime {
                    time_type: if n.sunrise_epoch < n.sunset_epoch {
                        TimeType::Sunset
                    } else {
                        TimeType::Sunrise
                    },
                    epoch: max(n.sunrise_epoch, n.sunset_epoch),
                });
            }
            SunMovementResult::FullDay => {
                day_type = DayType::FullDay;
            }
            SunMovementResult::FullNight => {
                day_type = DayType::FullNight;
            }
        };
    }

    // Read tomorrow if needed
    let check_tomorrow = if let Some(v) = next_sun_change {
        v.epoch < requested_epoch
    } else {
        true
    };
    if check_tomorrow {
        match calculate_sunrise_sunset(requested_epoch + DAY_MILLISECONDS, lat, lon, alt) {
            SunMovementResult::NormalDayAndNight(n) => {
                next_sun_change = Some(PointOfTime {
                    time_type: if n.sunrise_epoch < n.sunset_epoch {
                        TimeType::Sunrise
                    } else {
                        TimeType::Sunset
                    },
                    epoch: min(n.sunrise_epoch, n.sunset_epoch),
                });
            }
            SunMovementResult::FullDay => {
                day_type = DayType::FullDay;
            }
            SunMovementResult::FullNight => {
                day_type = DayType::FullNight;
            }
        };
    }

    Timeline {
        day_type,
        day_start_epoch,
        last_sun_change,
        next_sun_change,
    }
}

pub fn calculate_roman_sun_time(
    requested_epoch: i64,
    lat: f32,
    lon: f32,
    alt: f32,
) -> RomanTimeDetails {
    let timeline = build_timeline(requested_epoch, lat, lon, alt);

    let (roman_minute_length, duration_since_lc, clock_start, day_type) =
        match (timeline.last_sun_change, timeline.next_sun_change) {
            (Some(lc), Some(nc)) => {
                let lc_epoch = lc.epoch;
                let roman_minute_length = (nc.epoch - lc_epoch) / 720;
                let duration_since_lc = requested_epoch - min(nc.epoch, lc_epoch);
                let clock_start = if lc.time_type == TimeType::Sunrise {
                    6
                } else {
                    18
                };
                let day_type = if lc.time_type == TimeType::Sunrise {
                    DayType::NormalDay
                } else {
                    DayType::NormalNight
                };
                (
                    roman_minute_length,
                    duration_since_lc,
                    clock_start,
                    day_type,
                )
            }
            _ => {
                let roman_minute_length = 120000;
                let duration_since_lc = requested_epoch - timeline.day_start_epoch;
                println!(
                    "duration since {} in {}, d: {}",
                    requested_epoch, timeline.day_start_epoch, duration_since_lc
                );
                let clock_start = if timeline.day_type == DayType::FullDay {
                    6
                } else {
                    18
                };
                (
                    roman_minute_length,
                    duration_since_lc,
                    clock_start,
                    timeline.day_type,
                )
            }
        };
    let minutes_since_lc = (duration_since_lc / roman_minute_length) as i32;
    let roman_hours = ((minutes_since_lc / 60) + clock_start) % 24;
    let roman_minutes = minutes_since_lc % 60;

    RomanTimeDetails {
        hours: roman_hours,
        minutes: roman_minutes,
        minute_length: roman_minute_length as f32 / 1000.0,
        day_type,
        last_sun_change: match timeline.last_sun_change {
            Some(lsc) => Some(lsc.epoch),
            _ => None,
        },
        next_sun_change: match timeline.next_sun_change {
            Some(nsc) => Some(nsc.epoch),
            _ => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /** Latitude of Málaga. */
    const LAT: f32 = 36.6952469;
    /** Longitude of Málaga. */
    const LON: f32 = -4.4538953;
    const ALT: f32 = 0.0;

    #[test]
    fn test_midnight_day_start() {
        // Fri, 10 Jun 2022 00:00:00.000 GMT
        assert_eq!(
            calculate_roman_sun_time(1654819200000, LAT, LON, ALT),
            RomanTimeDetails {
                hours: 23,
                minutes: 38,
                last_sun_change: Some(1654803323788), // Thu, 09 Jun 2022 19:35:23 GMT
                next_sun_change: Some(1654837126628), // Fri, 10 Jun 2022 04:58:46 GMT
                minute_length: 46.948,
                day_type: DayType::NormalNight,
            }
        );
    }

    #[test]
    fn test_midnight_day_end_summer() {
        // Fri, 10 Jun 2022 23:59:59.999 GMT
        assert_eq!(
            calculate_roman_sun_time(1654905599999, LAT, LON, ALT),
            RomanTimeDetails {
                hours: 23,
                minutes: 37,
                last_sun_change: Some(1654889753237), // Fri, 10 Jun 2022 19:35:53 GMT
                next_sun_change: Some(1654923521349), // Sat, 11 Jun 2022 04:58:41 GMT
                minute_length: 46.9,
                day_type: DayType::NormalNight,
            }
        );
    }

    #[test]
    fn test_close_day_end_winter() {
        // Thu, 10 Nov 2022 22:50:00 GMT, Budapest
        assert_eq!(
            calculate_roman_sun_time(1668120600000, 47.49801, 19.03991, ALT),
            RomanTimeDetails {
                hours: 0,
                minutes: 17,
                last_sun_change: Some(1668093290048), // Thu, 10 Nov 2022 15:14:50 GMT
                next_sun_change: Some(1668145335985), // Fri, 11 Nov 2022 05:42:15 GMT
                minute_length: 72.286,
                day_type: DayType::NormalNight,
            }
        );
    }

    #[test]
    fn test_midday() {
        // Fri, 10 Jun 2022 08:00:00 GMT
        assert_eq!(
            calculate_roman_sun_time(1654848000000, LAT, LON, ALT),
            RomanTimeDetails {
                hours: 8,
                minutes: 28,
                last_sun_change: Some(1654837126628), // Fri, 10 Jun 2022 04:58:46 GMT
                next_sun_change: Some(1654889753237), // Fri, 10 Jun 2022 19:35:53 GMT
                minute_length: 73.092,
                day_type: DayType::NormalDay,
            }
        );
    }

    /* Get closest sun change went to infinite loop, when surrounding days should be
     * calculated. It is changed by adding follow_day boolean. This test is to cover
     * the original case before (and after) fix.
     */
    #[test]
    fn test_evening_bug_of_infinite_loop() {
        // Mon, 18 Nov 2024 18:05:50 GMT, Málaga (spec)
        assert_eq!(
            calculate_roman_sun_time(1731953150610, 36.6952287, -4.4538448, 0.0),
            RomanTimeDetails {
                hours: 18,
                minutes: 50,
                last_sun_change: Some(1731949632505), // Mon, 18 Nov 2024 17:07:12 GMT
                next_sun_change: Some(1731999614212), // Tue, 19 Nov 2024 07:00:14 GMT
                minute_length: 69.419,
                day_type: DayType::NormalNight,
            }
        );
    }

    #[test]
    fn test_evening_bug_of_displayed_hour_24() {
        // Wed, 20 Nov 2024 00:25:00 GMT+0100, Málaga (spec)
        assert_eq!(
            calculate_roman_sun_time(1732058700000, 36.6952287, -4.4538448, 0.0),
            RomanTimeDetails {
                hours: 23,
                minutes: 26,
                last_sun_change: Some(1732035998446), // Tue, 19 Nov 2024 18:06:38 GMT+01:00
                next_sun_change: Some(1732086075901), // Wed, 20 Nov 2024 08:01:15 GMT+01:00
                minute_length: 69.552,
                day_type: DayType::NormalNight,
            }
        );
    }

    #[test]
    fn test_last_day_with_sun_in_the_night() {
        let epoch = 1733500560000; // 2024-12-06T16:56:00
        let lat = 68.2992471;
        let lon = 22.2632669;
        let alt = 0.0;
        let result = calculate_roman_sun_time(epoch, lat, lon, alt);
        assert_eq!(
            result,
            RomanTimeDetails {
                hours: 1,
                minutes: 58,
                last_sun_change: Some(1733480947740), // 2024-12-06T10:29:07.740Z
                next_sun_change: None,
                minute_length: 120.0,
                day_type: DayType::FullNight,
            }
        );
    }
}
