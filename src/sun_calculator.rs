use crate::{
    earth::EarthState,
    rise_set::{self, calculate_rise_set, calculate_rise_set_v2},
    sun::{calculate_sun_state, SUN_GEOMETRIC_ALTITUDE_OF_BODY_CENTER},
    time_info::TimeInfo,
    types::{GeoLocation, RiseSetInfo},
};

/**
 * Sources:
 * - https://en.wikipedia.org/wiki/Julian_day
 * - https://en.wikipedia.org/wiki/Sunrise_equation
 */
pub fn calculate_sunrise_sunset(time_info: &TimeInfo, location: &GeoLocation) -> RiseSetInfo {
    let lat_rad = (location.lat() as f64).to_radians();
    let alt = location.alt() as f64;
    let earth_state = EarthState::from(time_info.current_julian_century);

    let sun_state = calculate_sun_state(time_info, location, &earth_state);
    println!("{:?}", sun_state);
    let elevation_correction_rad =
        (SUN_GEOMETRIC_ALTITUDE_OF_BODY_CENTER - 2.076 * alt.sqrt() / 60.0).to_radians();

    let rise_set_v1 = calculate_rise_set(
        sun_state.declination,
        elevation_correction_rad,
        lat_rad,
        sun_state.local_solar_noon_time,
    );
    println!("{:?}", rise_set_v1);
    let rise_set_v2 = calculate_rise_set_v2(
        sun_state.right_ascension,
        sun_state.declination,
        sun_state.local_solar_noon_time,
        location.lat(),
        location.lon(),
        SUN_GEOMETRIC_ALTITUDE_OF_BODY_CENTER,
    );
    println!("{:?}", rise_set_v2);
    rise_set_v1
}

#[cfg(test)]
mod tests {
    use crate::types::NormalDayAndNight;

    use super::*;

    #[test]
    fn test_arctic_circle_in_summer() {
        let epoch = 1656576000000; // Thu, 30 Jun 2022 08:00:00 GMT
        let lat = 67.5; // Inside the Arctic Circle
        let lon = 24.6657;
        let alt = 10.0;

        let result =
            calculate_sunrise_sunset(&TimeInfo::from(epoch), &GeoLocation::new(lat, lon, alt));
        assert!(matches!(result, RiseSetInfo::CircumpolarDay));
    }

    #[test]
    fn test_arctic_circle_in_winter() {
        let epoch = 1640159200000; // Wed, 22 Dec 2021 07:46:40 GMT
        let lat = 67.5; // Inside the Arctic Circle
        let lon = 0.0;
        let alt = 0.0;

        let result =
            calculate_sunrise_sunset(&TimeInfo::from(epoch), &GeoLocation::new(lat, lon, alt));
        assert!(matches!(result, RiseSetInfo::CircumpolarNight));
    }

    #[test]
    fn test_equator_almost_12_hours() {
        let epoch = 1648932200000; // Sat, 02 Apr 2022 20:43:20 GMT
        let lat = 0.0; // Equator
        let lon = 0.0;
        let alt = 0.0;
        let result =
            calculate_sunrise_sunset(&TimeInfo::from(epoch), &GeoLocation::new(lat, lon, alt));
        let sun_details = match result {
            RiseSetInfo::NormalDayAndNight(sd) => sd,
            _ => unreachable!(),
        };
        println!("{:?} - {:?}", sun_details.rise_time, sun_details.set_time);
        assert_eq!(sun_details.rise_time, 1648879207642.0); // Sat, 02 Apr 2022 06:00:07 GMT / SunCalc.org: 06:00:03
        assert_eq!(sun_details.set_time, 1648922808939.0); // Sat, 02 Apr 2022 18:06:48 GMT / SunCalc.org: 18:07:05
    }

    #[test]
    fn test_high_latitude_in_summer() {
        let epoch = 1656576000000; // Thu, 30 Jun 2022 08:00:00 GMT
        let lat = 80.0; // High in the northern hemisphere
        let lon = 0.0;
        let alt = 0.0;
        let result =
            calculate_sunrise_sunset(&TimeInfo::from(epoch), &GeoLocation::new(lat, lon, alt));
        assert!(matches!(result, RiseSetInfo::CircumpolarDay));
    }

    #[test]
    fn test_invalid_latitude() {
        let epoch = 1656576000000;
        let lat = 100.0; // Invalid latitude, should be within [-90, 90]
        let lon = 0.0;
        let alt = 0.0;
        let result = std::panic::catch_unwind(|| {
            calculate_sunrise_sunset(&TimeInfo::from(epoch), &GeoLocation::new(lat, lon, alt))
        });
        assert!(
            result.is_err(),
            "Function should panic with invalid latitude."
        );
    }

    #[test]
    fn test_invalid_longitude() {
        let epoch = 1656576000000;
        let lat = 45.0;
        let lon = 200.0; // Invalid longitude, should be within [-180, 180]
        let alt = 0.0;
        let result = std::panic::catch_unwind(|| {
            calculate_sunrise_sunset(&TimeInfo::from(epoch), &GeoLocation::new(lat, lon, alt))
        });
        assert!(
            result.is_err(),
            "Function should panic with invalid longitude."
        );
    }

    #[test]
    fn test_malaga_evening() {
        let epoch = 1731953150610;
        let lat = 36.6952287;
        let lon = -4.4538448;
        let alt = 0.0;
        let result =
            calculate_sunrise_sunset(&TimeInfo::from(epoch), &GeoLocation::new(lat, lon, alt));
        assert!(matches!(
            result,
            RiseSetInfo::NormalDayAndNight(NormalDayAndNight {
                rise_time: 1731913152254.0,
                set_time: 1731949632505.0
            })
        ));
    }

    #[test]
    fn test_almost_full_night() {
        let epoch = 1733500500000; // 2024-12-06T16:55:00
        let lat = 68.2992471;
        let lon = 22.2632669;
        let alt = 0.0;
        let result =
            calculate_sunrise_sunset(&TimeInfo::from(epoch), &GeoLocation::new(lat, lon, alt));
        assert!(matches!(
            result,
            RiseSetInfo::NormalDayAndNight(NormalDayAndNight {
                rise_time: 1733480122290.0,
                set_time: 1733480947740.0
            })
        ))
    }

    #[test]
    fn sunset_seems_earlier() {
        let epoch = 1653317700000; // 2022-05-23T16:55:00
        let lat = 68.574;
        let lon = 23.6077;
        let alt = 0.0;
        let result =
            calculate_sunrise_sunset(&TimeInfo::from(epoch), &GeoLocation::new(lat, lon, alt));
        println!("{:?}", result);
        assert!(matches!(
            result,
            RiseSetInfo::NormalDayAndNight(NormalDayAndNight {
                rise_time: 1653259119108.0, // 2022-05-22T22:38:39.108Z
                set_time: 1653343574951.0   // 2022-05-23T22:06:14.951Z
            })
        ));
    }
}
