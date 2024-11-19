use crate::sun_calculator;
use crate::types::{PartialOrFullDayNight, RomanTimeDetails};
use std::cmp::{max, min};

mod time_calculators;

struct DayDetails {
    sunrise_epoch: i64,
    sunset_epoch: i64,
    is_day: bool,
    is_morning: bool,
}

static DAY_MILLISECONDS: i64 = 86400000;

/**
 * @param follow_day Get closest sun change could go to infinite loop, when surrounding days should be always
 *                   calculated. This parameter is used to break the loop and calculate only one level deep.
 */
fn get_closest_sun_change(
    current_epoch: i64,
    lat: f32,
    lon: f32,
    alt: f32,
    follow_day: bool,
) -> DayDetails {
    match sun_calculator::calculate_sunrise_sunset(current_epoch, lat, lon, alt) {
        PartialOrFullDayNight::FullDayNight(fdn) => DayDetails {
            sunrise_epoch: if fdn.is_day {
                time_calculators::get_day_start(current_epoch)
            } else {
                time_calculators::get_day_end(current_epoch)
            },
            sunset_epoch: if fdn.is_day {
                time_calculators::get_day_end(current_epoch)
            } else {
                time_calculators::get_day_start(current_epoch)
            },
            is_day: fdn.is_day,
            is_morning: !fdn.is_day,
        },
        PartialOrFullDayNight::PartialDayNight(sd) => {
            let sunrise_epoch = if !follow_day || current_epoch <= sd.sunset_epoch {
                sd.sunrise_epoch
            } else {
                let tomorrow_epoch = current_epoch + DAY_MILLISECONDS;
                get_closest_sun_change(tomorrow_epoch, lat, lon, alt, false).sunrise_epoch
            };
            let sunset_epoch = if !follow_day || current_epoch >= sd.sunrise_epoch {
                sd.sunset_epoch
            } else {
                let yesterday_epoch = current_epoch - (DAY_MILLISECONDS / 2);
                get_closest_sun_change(yesterday_epoch, lat, lon, alt, false).sunset_epoch
            };
            DayDetails {
                sunrise_epoch,
                sunset_epoch,
                is_day: sunrise_epoch <= current_epoch && current_epoch <= sunset_epoch,
                is_morning: current_epoch <= sd.sunrise_epoch,
            }
        }
    }
}

pub fn calculate_roman_sun_time(
    current_epoch: i64,
    lat: f32,
    lon: f32,
    alt: f32,
) -> RomanTimeDetails {
    let today_details = get_closest_sun_change(current_epoch, lat, lon, alt, true);

    let roman_minute_length =
        (today_details.sunrise_epoch - today_details.sunset_epoch).abs() / 720;
    let duration_since_sunrise =
        current_epoch - min(today_details.sunrise_epoch, today_details.sunset_epoch);
    let minutes_since_sunrise = (duration_since_sunrise / roman_minute_length) as i32;
    let time_of_day_adjustment = if today_details.is_morning {
        -6
    } else if today_details.is_day {
        6
    } else {
        18
    };
    let roman_hours = ((minutes_since_sunrise) / 60) + time_of_day_adjustment;

    RomanTimeDetails {
        hours: roman_hours
            + if roman_hours < 0 {
                24
            } else if roman_hours >= 24 {
                -24
            } else {
                0
            },
        minutes: (minutes_since_sunrise % 60),
        last_sun_change: min(today_details.sunrise_epoch, today_details.sunset_epoch),
        next_sun_change: max(today_details.sunrise_epoch, today_details.sunset_epoch),
        total_minutes: minutes_since_sunrise + (time_of_day_adjustment * 60),
        minute_length: roman_minute_length as f32 / 1000.0,
        is_day: today_details.is_day,
        is_morning: today_details.is_morning,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Malaga
    const LAT: f32 = 36.6952469;
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
                last_sun_change: 1654803323788, // Thu, 09 Jun 2022 19:35:23 GMT
                next_sun_change: 1654837126628, // Fri, 10 Jun 2022 04:58:46 GMT
                total_minutes: -22,
                minute_length: 46.948,
                is_day: false,
                is_morning: true
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
                last_sun_change: 1654889753237, // Fri, 10 Jun 2022 19:35:53 GMT
                next_sun_change: 1654923521349, // Sat, 11 Jun 2022 04:58:41 GMT
                total_minutes: -23,
                minute_length: 46.9,
                is_day: false,
                is_morning: true // Because it is full day
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
                last_sun_change: 1668093290048, // Thu, 10 Nov 2022 15:14:50 GMT
                next_sun_change: 1668145335985, // Fri, 11 Nov 2022 05:42:15 GMT
                total_minutes: 1457,
                minute_length: 72.286,
                is_day: false,
                is_morning: false
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
                last_sun_change: 1654837126628, // Fri, 10 Jun 2022 04:58:46 GMT
                next_sun_change: 1654889753237, // Fri, 10 Jun 2022 19:35:53 GMT
                total_minutes: 508,
                minute_length: 73.092,
                is_day: true,
                is_morning: false
            }
        );
    }

    /* Get closest sun change went to infinite loop, when surrounding days should be
     * calculated. It is changed by adding follow_day boolean. This test is to cover
     * the original case before (and after) fix.
     */
    #[test]
    fn test_evening_bug_of_infinite_loop() {
        // Mon, 18 Nov 2024 18:05:50 GMT
        assert_eq!(
            calculate_roman_sun_time(1731953150610, 36.6952287, -4.4538448, 0.0),
            RomanTimeDetails {
                hours: 18,
                minutes: 50,
                last_sun_change: 1731949632505, // Mon, 18 Nov 2024 17:07:12 GMT
                next_sun_change: 1731999614212, // Tue, 19 Nov 2024 07:00:14 GMT
                total_minutes: 1130,
                minute_length: 69.419,
                is_day: false,
                is_morning: false
            }
        );
    }
}
