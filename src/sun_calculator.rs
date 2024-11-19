use crate::types::{FullDayOrNight, PartialOrFullDayNight, SunDetails};

mod julian_converters;

static ARGUMENT_OF_PERIAPSIS: f64 = 102.9372;
/** Julian day count of 1 January 2000 */
static JULIAN_DAY_Y2K: f64 = 2451545.0;
static CENTER_EQUATION_COEFFICIENT: f64 = 1.9148;
static DAY_MILLISECONDS: f64 = 86400000.0;
static EARTH_AXIAL_TILT: f64 = 23.4397;
static HOUR_ANGLE_COEFFICIENT_DEGREES: f64 = -0.833;
static LEAP_SECONDS_AND_TERRESTRIAL_TIME: f64 = 69.184;

/**
 * Sources:
 * - https://en.wikipedia.org/wiki/Julian_day
 * - https://en.wikipedia.org/wiki/Sunrise_equation
 */
pub fn calculate_sunrise_sunset(epoch: i64, lat: f32, lon: f32, alt: f32) -> PartialOrFullDayNight {
    if lat < -90.0 || lat > 90.0 {
        panic!("Invalid latitude");
    }
    if lon < -180.0 || lon > 180.0 {
        panic!("Invalid longitude");
    }

    let lat_rad = (lat as f64).to_radians();

    let current_julian_date = julian_converters::unix_milliseconds_to_julian_date(epoch);
    let fractional_julian_day = LEAP_SECONDS_AND_TERRESTRIAL_TIME / DAY_MILLISECONDS;
    let current_julian_day = (current_julian_date - JULIAN_DAY_Y2K + fractional_julian_day).round();

    let mean_solar_time = current_julian_day - f64::from(lon / 360.0);
    let solar_mean_anomaly = (357.5291 + (0.98560028 * mean_solar_time)).rem_euclid(360.0);
    let solar_mean_anomaly_rad = solar_mean_anomaly.to_radians();
    let equation_of_center = CENTER_EQUATION_COEFFICIENT * solar_mean_anomaly_rad.sin()
        + 0.02 * (2.0 * solar_mean_anomaly_rad).sin()
        + 0.0003 * (3.0 * solar_mean_anomaly_rad).sin();
    let ecliptic_longitude =
        (solar_mean_anomaly + equation_of_center + 180.0 + ARGUMENT_OF_PERIAPSIS).rem_euclid(360.0);
    let ecliptic_longitude_rad = ecliptic_longitude.to_radians();
    let equation_of_time =
        0.0053 * solar_mean_anomaly_rad.sin() - 0.0069 * (2.0 * ecliptic_longitude_rad).sin();
    let solar_transit = JULIAN_DAY_Y2K + mean_solar_time + equation_of_time; // Local solar noon
    let declination_of_sun_sin = ecliptic_longitude_rad.sin() * EARTH_AXIAL_TILT.to_radians().sin();
    let declination_of_sun_rad = declination_of_sun_sin.asin();
    let elevation_correction_rad =
        (HOUR_ANGLE_COEFFICIENT_DEGREES - 2.076 * (alt as f64).sqrt() / 60.0).to_radians();
    let hour_angle_cos = (elevation_correction_rad.sin()
        - (lat_rad.sin() * declination_of_sun_sin))
        / (lat_rad.cos() * declination_of_sun_rad.cos());

    // <-1 day
    if hour_angle_cos <= -1.0 {
        PartialOrFullDayNight::FullDayNight(FullDayOrNight { is_day: true })
    // >1 night
    } else if hour_angle_cos >= 1.0 {
        PartialOrFullDayNight::FullDayNight(FullDayOrNight { is_day: false })
    } else {
        let hour_angle = hour_angle_cos.acos().to_degrees();

        let sunrise = solar_transit - (hour_angle / 360.0);
        let sunset = solar_transit + (hour_angle / 360.0);

        PartialOrFullDayNight::PartialDayNight(SunDetails {
            sunrise_epoch: julian_converters::julian_date_to_unix_milliseconds(sunrise),
            sunset_epoch: julian_converters::julian_date_to_unix_milliseconds(sunset),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arctic_circle_in_summer() {
        let epoch = 1656576000000; // Thu, 30 Jun 2022 08:00:00 GMT
        let lat = 67.5; // Inside the Arctic Circle
        let lon = 24.6657;
        let alt = 10.0;

        let result = calculate_sunrise_sunset(epoch, lat, lon, alt);
        assert!(matches!(
            result,
            PartialOrFullDayNight::FullDayNight(FullDayOrNight { is_day: true })
        ));
    }

    #[test]
    fn test_arctic_circle_in_winter() {
        let epoch = 1640159200000; // Wed, 22 Dec 2021 07:46:40 GMT
        let lat = 67.5; // Inside the Arctic Circle
        let lon = 0.0;
        let alt = 0.0;

        let result = calculate_sunrise_sunset(epoch, lat, lon, alt);
        assert!(matches!(
            result,
            PartialOrFullDayNight::FullDayNight(FullDayOrNight { is_day: false })
        ));
    }

    #[test]
    fn test_equator_almost_12_hours() {
        let epoch = 1648932200000; // Sat, 02 Apr 2022 20:43:20 GMT
        let lat = 0.0; // Equator
        let lon = 0.0;
        let alt = 0.0;
        let result = calculate_sunrise_sunset(epoch, lat, lon, alt);
        let sun_details = match result {
            PartialOrFullDayNight::FullDayNight(_) => unreachable!(),
            PartialOrFullDayNight::PartialDayNight(sd) => sd,
        };
        println!(
            "{:?} - {:?}",
            sun_details.sunrise_epoch, sun_details.sunset_epoch
        );
        assert_eq!(sun_details.sunrise_epoch, 1648879207642); // Sat, 02 Apr 2022 06:00:07 GMT / SunCalc.org: 06:00:03
        assert_eq!(sun_details.sunset_epoch, 1648922808939); // Sat, 02 Apr 2022 18:06:48 GMT / SunCalc.org: 18:07:05
    }

    #[test]
    fn test_high_latitude_in_summer() {
        let epoch = 1656576000000; // Thu, 30 Jun 2022 08:00:00 GMT
        let lat = 80.0; // High in the northern hemisphere
        let lon = 0.0;
        let alt = 0.0;
        let result = calculate_sunrise_sunset(epoch, lat, lon, alt);
        assert!(matches!(
            result,
            PartialOrFullDayNight::FullDayNight(FullDayOrNight { is_day: true })
        ));
    }

    #[test]
    fn test_invalid_latitude() {
        let epoch = 1656576000000;
        let lat = 100.0; // Invalid latitude, should be within [-90, 90]
        let lon = 0.0;
        let alt = 0.0;
        let result = std::panic::catch_unwind(|| calculate_sunrise_sunset(epoch, lat, lon, alt));
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
        let result = std::panic::catch_unwind(|| calculate_sunrise_sunset(epoch, lat, lon, alt));
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
        let result = calculate_sunrise_sunset(epoch, lat, lon, alt);
        println!("{:?}", result);
        assert!(matches!(
            result,
            PartialOrFullDayNight::PartialDayNight(SunDetails {
                sunrise_epoch: 1731913152254,
                sunset_epoch: 1731949632505
            })
        ));
    }
}
