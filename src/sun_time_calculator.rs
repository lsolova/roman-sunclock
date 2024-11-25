use crate::sun_calculator;
use crate::types::{PartialOrFullDayNight, RomanTimeDetails};
use std::cmp::{max, min};

mod time_calculators;

struct DayDetails {
    today_start: i64,
    today_sunrise_epoch: i64,
    today_sunset_epoch: i64,
    today_end: i64,
    last_sun_change: i64,
    next_sun_change: i64,
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
    let day_start = time_calculators::get_day_start(current_epoch);
    let day_end = time_calculators::get_day_end(current_epoch);
    let calculated_sun_changes = sun_calculator::calculate_sunrise_sunset(current_epoch, lat, lon, alt);
    match calculated_sun_changes {
        PartialOrFullDayNight::FullDayNight(fdn) => DayDetails {
            last_sun_change: day_start,
            next_sun_change: day_end,
            today_start: day_start,
            today_end: day_end,
            today_sunrise_epoch: if fdn.is_day { day_start } else { day_end },
            today_sunset_epoch: if fdn.is_day { day_start } else { day_end },
            is_day: fdn.is_day,
            is_morning: !fdn.is_day,
        },
        PartialOrFullDayNight::PartialDayNight(sd) => {
            let inclusive_sunrise_epoch = if !follow_day || current_epoch <= sd.sunset_epoch {
                sd.sunrise_epoch
            } else {
                let tomorrow_epoch = current_epoch + DAY_MILLISECONDS;
                get_closest_sun_change(tomorrow_epoch, lat, lon, alt, false).today_sunrise_epoch
            };
            let inclusive_sunset_epoch = if !follow_day || current_epoch >= sd.sunrise_epoch {
                sd.sunset_epoch
            } else {
                let yesterday_epoch = current_epoch - (DAY_MILLISECONDS / 2);
                get_closest_sun_change(yesterday_epoch, lat, lon, alt, false).today_sunset_epoch
            };
            DayDetails {
                today_start: day_start,
                today_sunrise_epoch: sd.sunrise_epoch,
                today_sunset_epoch: sd.sunset_epoch,
                today_end: day_end,
                last_sun_change: min(inclusive_sunrise_epoch, inclusive_sunset_epoch),
                next_sun_change: max(inclusive_sunrise_epoch, inclusive_sunset_epoch),
                is_day: inclusive_sunrise_epoch <= current_epoch
                    && current_epoch <= inclusive_sunset_epoch,
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
        (today_details.last_sun_change - today_details.next_sun_change).abs() / 720;
    let duration_since_sunrise =
        current_epoch - min(today_details.last_sun_change, today_details.next_sun_change);
    let minutes_since_sunrise = (duration_since_sunrise / roman_minute_length) as i32;
    let time_of_day_adjustment = if today_details.is_morning {
        -6
    } else if today_details.is_day {
        6
    } else {
        18
    };
    let roman_hours = ((minutes_since_sunrise) / 60) + time_of_day_adjustment;
    let roman_hours_adjustment = if roman_hours < 0 {
        24
    } else if roman_hours >= 24 {
        -24
    } else {
        0
    };

    RomanTimeDetails {
        hours: roman_hours + roman_hours_adjustment,
        minutes: (minutes_since_sunrise % 60),
        last_sun_change: today_details.last_sun_change,
        next_sun_change: today_details.next_sun_change,
        today_start: today_details.today_start,
        today_end: today_details.today_end,
        today_sunrise: today_details.today_sunrise_epoch,
        today_sunset: today_details.today_sunset_epoch,
        total_minutes: minutes_since_sunrise + (time_of_day_adjustment * 60),
        minute_length: roman_minute_length as f32 / 1000.0,
        is_day: today_details.is_day,
        is_morning: today_details.is_morning,
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
                last_sun_change: 1654803323788, // Thu, 09 Jun 2022 19:35:23 GMT
                next_sun_change: 1654837126628, // Fri, 10 Jun 2022 04:58:46 GMT
                today_start: 1654819200000, // Fri, 10 Jun 2022 00:00:00 GMT
                today_end: 1654905599999, // Fri, 10 Jun 2022 23:59:59 GMT
                today_sunrise: 1654837126628, // Fri, 10 Jun 2022 04:58:46 GMT
                today_sunset: 1654889753237, // Fri, 10 Jun 2022 19:35:53 GMT
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
                today_start: 1654819200000, // Fri, 10 Jun 2022 00:00:00 GMT
                today_end: 1654905599999, // Fri, 10 Jun 2022 23:59:59 GMT
                today_sunrise: 1654837126628, // Fri, 10 Jun 2022 04:58:46 GMT
                today_sunset: 1654889753237, // Fri, 10 Jun 2022 19:35:53 GMT
                total_minutes: 1417,
                minute_length: 46.9,
                is_day: false,
                is_morning: false
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
                today_start: 1668038400000, // Thu, 10 Nov 2022 00:00:00 GMT
                today_end: 1668124799999, // Thu, 10 Nov 2022 23:59:59 GMT
                today_sunrise: 1668058844950, // Thu, 10 Nov 2022 05:40:44 GMT
                today_sunset: 1668093290048, // Thu, 10 Nov 2022 15:14:50 GMT
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
                today_start: 1654819200000, // Fri, 10 Jun 2022 00:00:00 GMT
                today_end: 1654905599999, // Fri, 10 Jun 2022 23:59:59 GMT
                today_sunrise: 1654837126628, // Fri, 10 Jun 2022 04:58:46 GMT
                today_sunset: 1654889753237, // Fri, 10 Jun 2022 19:35:53 GMT
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
        // Mon, 18 Nov 2024 18:05:50 GMT, Málaga (spec)
        assert_eq!(
            calculate_roman_sun_time(1731953150610, 36.6952287, -4.4538448, 0.0),
            RomanTimeDetails {
                hours: 18,
                minutes: 50,
                last_sun_change: 1731949632505, // Mon, 18 Nov 2024 17:07:12 GMT
                next_sun_change: 1731999614212, // Tue, 19 Nov 2024 07:00:14 GMT
                today_start: 1731888000000, // Mon, 18 Nov 2024 00:00:00 GMT
                today_end: 1731974399999, // Mon, 18 Nov 2024 23:59:59 GMT
                today_sunrise: 1731913152254, // Mon, 18 Nov 2024 06:59:12 GMT
                today_sunset: 1731949632505, // Mon, 18 Nov 2024 17:07:12 GMT
                total_minutes: 1130,
                minute_length: 69.419,
                is_day: false,
                is_morning: false
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
                last_sun_change: 1732035998446, // Tue, 19 Nov 2024 18:06:38 GMT+01:00
                next_sun_change: 1732086075901, // Wed, 20 Nov 2024 08:01:15 GMT+01:00
                today_start: 1731974400000, // Tue, 19 Nov 2024 01:00:00 GMT+01:00
                today_end: 1732060799999, // Tue, 20 Nov 2024 00:59:59 GMT+01:00
                today_sunrise: 1731999614212, // Tue, 19 Nov 2024 08:00:14 GMT+01:00
                today_sunset: 1732035998446, // Tue, 19 Nov 2024 18:06:38 GMT+01:00
                total_minutes: 1406,
                minute_length: 69.552,
                is_day: false,
                is_morning: false
            }
        );
    }
}
