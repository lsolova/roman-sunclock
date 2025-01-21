use super::earth::EarthState;
use super::utils::consolidate_to_0_360;

#[derive(Debug, PartialEq)]
pub struct SunState {
    /** Declination in degrees.
     *  https://en.wikipedia.org/wiki/Declination */
    pub declination: f64,
    /** Equation of center in degrees.
     *  https://en.wikipedia.org/wiki/Equation_of_the_center */
    equation_of_center: f64,
    /** Mean anomaly in degrees. Consolidated between 0 - 360°.
     *  https://en.wikipedia.org/wiki/Mean_anomaly */
    pub mean_anomaly: f64,
    /** Mean logitude in degrees. Consolidated between 0 - 360°.
     *  https://en.wikipedia.org/wiki/Mean_longitude */
    mean_longitude: f64,
    /** Right ascension in degrees. Consolidated between 0 - 360°.
     *  https://en.wikipedia.org/wiki/Right_ascension */
    pub right_ascension: f64,
}

pub fn calculate_sun_state(current_julian_century: f64, earth_state: &EarthState) -> SunState {
    let pow2_current_julian_century = current_julian_century * current_julian_century;
    let earth_eccentricity = earth_state.eccentricity;
    let earth_obliquity_of_the_ecliptic_deg = earth_state.obliquity_of_the_ecliptic;

    // L
    let sun_mean_longitude_deg = consolidate_to_0_360(
        280.46646
            + (36000.76983 * current_julian_century)
            + (0.0003032 * pow2_current_julian_century),
    );
    // M
    let sun_mean_anomaly_deg = consolidate_to_0_360(
        357.52911 + (35999.05029 * current_julian_century)
            - (0.0001537 * pow2_current_julian_century),
    );
    let sun_mean_anomaly_rad = sun_mean_anomaly_deg.to_radians();
    // C
    let sun_equation_of_center_deg = ((1.914602
        - (0.004817 * current_julian_century)
        - (0.000014 * pow2_current_julian_century))
        * sun_mean_anomaly_rad.sin())
        + ((0.019993 - (0.000101 * current_julian_century)) * (2.0 * sun_mean_anomaly_rad).sin())
        + (0.000289 * (3.0 * sun_mean_anomaly_rad).sin());
    // ⨀
    let sun_true_longitude_deg = sun_mean_longitude_deg + sun_equation_of_center_deg;
    // ν
    let sun_true_anomaly_deg = sun_mean_anomaly_deg + sun_equation_of_center_deg;
    // R
    let sun_radius_vector = (1.000001018 * (1.0 - (earth_eccentricity * earth_eccentricity)))
        / (1.0 + (earth_eccentricity * sun_true_anomaly_deg.to_radians().cos()));
    // Ω
    let omega_value_deg = 125.04 - 1934.136 * current_julian_century;
    let omega_value_rad = omega_value_deg.to_radians();
    // λ
    let apparent_longitude_deg =
        sun_true_longitude_deg - 0.00569 - (0.00478 * omega_value_rad.sin());
    let apparent_longitude_rad = apparent_longitude_deg.to_radians();
    let corrected_earth_obliquity_of_the_ecliptic_deg =
        earth_obliquity_of_the_ecliptic_deg + (0.00256 * omega_value_rad.cos());
    let corrected_earth_obliquity_of_the_ecliptic_rad =
        corrected_earth_obliquity_of_the_ecliptic_deg.to_radians();

    // α
    let sun_right_ascension_rad = (corrected_earth_obliquity_of_the_ecliptic_rad.cos()
        * apparent_longitude_rad.sin())
    .atan2(apparent_longitude_rad.cos());
    let sun_right_ascension_deg = consolidate_to_0_360(sun_right_ascension_rad.to_degrees());
    // δ
    let sun_declination_rad =
        (corrected_earth_obliquity_of_the_ecliptic_rad.sin() * apparent_longitude_rad.sin()).asin();
    let sun_declination_deg = sun_declination_rad.to_degrees();

    println!("T {:?}", current_julian_century);
    println!("L {:?}", sun_mean_longitude_deg);
    println!("M {:?}", sun_mean_anomaly_deg);
    println!("e {:?}", earth_eccentricity);
    println!("C {:?}", sun_equation_of_center_deg);
    println!("⨀ {:?}", sun_true_longitude_deg);
    println!("v {:?}", sun_true_anomaly_deg);
    println!("R {:?}", sun_radius_vector);
    println!("Ω {:?}", omega_value_deg);
    println!("λ {:?}", apparent_longitude_deg);
    println!("ε {:?}", corrected_earth_obliquity_of_the_ecliptic_deg);
    println!("α {:?}", sun_right_ascension_deg);
    println!("δ {:?}", sun_declination_deg);

    SunState {
        declination: sun_declination_deg,
        equation_of_center: sun_equation_of_center_deg,
        mean_anomaly: sun_mean_anomaly_deg,
        mean_longitude: sun_mean_longitude_deg,
        right_ascension: sun_right_ascension_deg,
    }
}

#[cfg(test)]
mod tests {
    use crate::celestial_calculations::earth::calculate_earth_state;

    use super::*;

    #[test]
    fn meeus_example() {
        let expected_sun_state = SunState {
            declination: -7.78507,
            equation_of_center: -1.89732,
            mean_anomaly: 278.99397,
            mean_longitude: 201.80720,
            right_ascension: 198.38083,
        };
        let current_julian_century_time = -0.072183436;
        let earth_state = calculate_earth_state(current_julian_century_time);
        let sun_state = calculate_sun_state(current_julian_century_time, &earth_state);
        assert_eq!(
            (sun_state.declination * 100000.0).round() / 100000.0,
            expected_sun_state.declination
        );
        assert_eq!(
            (sun_state.equation_of_center * 100000.0).round() / 100000.0,
            expected_sun_state.equation_of_center
        );
        assert_eq!(
            (sun_state.mean_anomaly * 100000.0).round() / 100000.0,
            expected_sun_state.mean_anomaly
        );
        assert_eq!(
            (sun_state.mean_longitude * 100000.0).round() / 100000.0,
            expected_sun_state.mean_longitude
        );
        assert_eq!(
            (sun_state.right_ascension * 100000.0).round() / 100000.0,
            expected_sun_state.right_ascension
        );
    }

    #[test]
    fn celestial_programming_example_1() {
        let expected_sun_state = SunState {
            declination: -20.022844800277305,
            equation_of_center: -1.89732,
            mean_anomaly: 278.99397,
            mean_longitude: 201.80720,
            right_ascension: 302.7852410043696,
        };
        // jd 2460695.338623486
        let current_julian_century_time = 0.25052261802836395;
        let earth_state = calculate_earth_state(current_julian_century_time);
        let sun_state = calculate_sun_state(current_julian_century_time, &earth_state);

        println!("{:?}", sun_state);
    }
}
