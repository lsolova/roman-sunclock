use crate::sun_calculator;
use crate::types::RomanTimeDetails;

static DAY_MILLISECONDS: i64 = 86400000;

pub fn calculate_roman_sun_time(current_epoch: i64, lat: f32 ,lon: f32 ,alt: f32) -> RomanTimeDetails {
    let yesterday_sun_details = sun_calculator::calculate_sunrise_sunset(current_epoch - DAY_MILLISECONDS, lat, lon, alt);
    let today_sun_details = sun_calculator::calculate_sunrise_sunset(current_epoch, lat, lon, alt);
    let tomorrow_sun_details = sun_calculator::calculate_sunrise_sunset(current_epoch + DAY_MILLISECONDS, lat, lon, alt);

    let is_morning = current_epoch < today_sun_details.sunrise_epoch;
    let is_day = today_sun_details.sunrise_epoch <= current_epoch && current_epoch <= today_sun_details.sunset_epoch;
    // let is_night = today_sun_details.sunset_epoch < current_epoch;

    let roman_minute_info: (i32, f32) = if is_morning {
        let roman_minute_length = (today_sun_details.sunrise_epoch - yesterday_sun_details.sunset_epoch) / 720;
        println!("Roman minute length: {}", roman_minute_length);
        let time_since_sunrise = current_epoch - yesterday_sun_details.sunset_epoch;
        ((time_since_sunrise / roman_minute_length) as i32, roman_minute_length as f32)
    } else if is_day {
        let roman_minute_length = (today_sun_details.sunset_epoch - today_sun_details.sunrise_epoch) / 720;
        println!("Roman minute length: {}", roman_minute_length);
        let time_since_sunrise = current_epoch - today_sun_details.sunrise_epoch;
        ((time_since_sunrise / roman_minute_length) as i32, roman_minute_length as f32)
    } else {
        let roman_minute_length = (tomorrow_sun_details.sunrise_epoch - today_sun_details.sunset_epoch) / 720;
        println!("Roman minute length: {}", roman_minute_length);
        let time_since_sunrise = current_epoch - today_sun_details.sunset_epoch;
        ((time_since_sunrise / roman_minute_length) as i32, roman_minute_length as f32)
    };

    RomanTimeDetails {
        hours: ((roman_minute_info.0) / 60) + if is_morning { -6 } else if is_day { 6 } else { 18 },
        minutes: (roman_minute_info.0 % 60),
        last_sun_change: today_sun_details.sunrise_epoch,
        next_sun_change: today_sun_details.sunset_epoch,
        total_minutes: roman_minute_info.0 + if is_morning { -360 } else if is_day { 360 } else { 1080 },
        minute_length: roman_minute_info.1 / 1000.0,
        is_day,
    }
}
