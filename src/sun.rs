use std::fmt::Debug;

use crate::{
    earth::EarthState,
    time_info::{TimeInfo, JULIAN_DAY_Y2K},
    types::GeoLocation,
    utils::normalize_to_circle_radians,
};

static ARGUMENT_OF_PERIAPSIS: f64 = 102.9372;
static CENTER_EQUATION_COEFFICIENT: f64 = 1.9148;
static EARTH_AXIAL_TILT: f64 = 23.4397;
pub static SUN_GEOMETRIC_ALTITUDE_OF_BODY_CENTER: f64 = -0.833; // In degree

pub struct SunState {
    pub declination: f64, // In radians
    pub local_solar_noon_time: f64,
    pub right_ascension: f64, // In radians
}

impl Debug for SunState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SunState (δ {:?}, α {:?})",
            self.declination, self.right_ascension
        )
    }
}

pub fn calculate_sun_state(
    time_info: &TimeInfo,
    location: &GeoLocation,
    earth_state: &EarthState,
) -> SunState {
    let current_julian_day = time_info.current_julian_day;
    let mean_solar_time = current_julian_day - f64::from(location.lon() / 360.0);
    let solar_mean_anomaly = (357.5291 + (0.98560028 * mean_solar_time)).rem_euclid(360.0);
    let solar_mean_anomaly_rad = solar_mean_anomaly.to_radians();
    let equation_of_center = CENTER_EQUATION_COEFFICIENT * solar_mean_anomaly_rad.sin()
        + 0.02 * (2.0 * solar_mean_anomaly_rad).sin()
        + 0.0003 * (3.0 * solar_mean_anomaly_rad).sin();
    let ecliptic_longitude =
        (solar_mean_anomaly + equation_of_center + 180.0 + ARGUMENT_OF_PERIAPSIS).rem_euclid(360.0);
    let ecliptic_longitude_rad = ecliptic_longitude.to_radians();
    let right_ascension = normalize_to_circle_radians(
        (earth_state.obliquity_of_the_ecliptic.cos() * ecliptic_longitude_rad.sin())
            .atan2(ecliptic_longitude_rad.cos()),
    );
    let equation_of_time =
        0.0053 * solar_mean_anomaly_rad.sin() - 0.0069 * (2.0 * ecliptic_longitude_rad).sin();
    let local_solar_noon_time = JULIAN_DAY_Y2K + mean_solar_time + equation_of_time;
    let declination_of_sun_sin = ecliptic_longitude_rad.sin() * EARTH_AXIAL_TILT.to_radians().sin();
    let declination_of_sun_rad = declination_of_sun_sin.asin();

    SunState {
        declination: declination_of_sun_rad,
        local_solar_noon_time,
        right_ascension,
    }
}
