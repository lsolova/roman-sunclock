use crate::types::SunDetails;

mod julian_converters;

static ARGUMENT_OF_PERIAPSIS: f64 = 102.9372;
static JULIAN_DAY_Y2K: f64 = 2451545.0; // Julian day count of 1 January 2000
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
pub fn calculate_sunrise_sunset(epoch: i64, lat: f32, lon: f32, alt: f32) -> SunDetails {
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
    let ecliptic_longitude = (solar_mean_anomaly + equation_of_center + 180.0 + ARGUMENT_OF_PERIAPSIS).rem_euclid(360.0);
    let ecliptic_longitude_rad = ecliptic_longitude.to_radians();
    let equation_of_time = 0.0053 * solar_mean_anomaly_rad.sin() - 0.0069 * (2.0 * ecliptic_longitude_rad).sin();
    let solar_transit = JULIAN_DAY_Y2K + mean_solar_time + equation_of_time; // Local solar noon
    let declination_of_sun_sin = ecliptic_longitude_rad.sin() * EARTH_AXIAL_TILT.to_radians().sin();
    let declination_of_sun_rad = declination_of_sun_sin.asin();
    let elevation_correction_rad = (HOUR_ANGLE_COEFFICIENT_DEGREES - 2.076 * (alt as f64).sqrt() / 60.0).to_radians();
    let hour_angle_cos = (elevation_correction_rad.sin() - (lat_rad.sin() * declination_of_sun_sin)) / ( lat_rad.cos() * declination_of_sun_rad.cos());
    let hour_angle = hour_angle_cos.acos().to_degrees();

    let sunrise = solar_transit - (hour_angle / 360.0);
    let sunset = solar_transit + (hour_angle / 360.0);

    SunDetails {
        sunrise_epoch: julian_converters::julian_date_to_unix_milliseconds(sunrise),
        sunset_epoch: julian_converters::julian_date_to_unix_milliseconds(sunset),
    }
}
