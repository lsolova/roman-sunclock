use crate::celestial_calculations::utils::consolidate_to_0_360;

#[derive(Debug, PartialEq)]
pub struct EarthState {
    /** Eccentricity.
     *  https://en.wikipedia.org/wiki/Orbital_eccentricity */
    pub eccentricity: f64,
    /** Obliquity of the ecliptic or axial tilt at the provided time. In degrees.
     *  https://en.wikipedia.org/wiki/Axial_tilt */
    pub obliquity_of_the_ecliptic: f64,
}

pub fn calculate_earth_state(current_julian_century: f64) -> EarthState {
    let pow2_current_julian_century = current_julian_century * current_julian_century;
    let u_time = current_julian_century / 100.0;
    // ε
    let earth_obliquity_of_the_ecliptic_deg = 23.43929111111111
        - (1.300258333333333333 * u_time)
        - (1.55 * u_time * u_time)
        + (1999.25 * u_time * u_time * u_time)
        - 51.38 * u_time * u_time * u_time * u_time
        - 249.67 * u_time * u_time * u_time * u_time * u_time
        - 39.05 * u_time * u_time * u_time * u_time * u_time * u_time
        + 7.12 * u_time * u_time * u_time * u_time * u_time * u_time * u_time
        + 27.87 * u_time * u_time * u_time * u_time * u_time * u_time * u_time * u_time
        + 5.79 * u_time * u_time * u_time * u_time * u_time * u_time * u_time * u_time * u_time
        + 2.45 * u_time * u_time * u_time * u_time * u_time * u_time * u_time * u_time * u_time * u_time;
    // e
    let eccentricity = 0.016708634
        - (0.000042037 * current_julian_century)
        - (0.0000001267 * pow2_current_julian_century);

    println!("Earth --------------");
    println!("ε {:?}", earth_obliquity_of_the_ecliptic_deg);
    println!("e {:?}", eccentricity);

    EarthState {
        eccentricity,
        obliquity_of_the_ecliptic: earth_obliquity_of_the_ecliptic_deg,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn meeus_example() {
        let expected_earth_state = EarthState {
            eccentricity: 0.016711668,
            obliquity_of_the_ecliptic: 23.44023,
        };
        let current_julian_century_time = -0.072183436;
        let earth_state = calculate_earth_state(current_julian_century_time);
        assert_eq!(
            (earth_state.eccentricity * 1000000000.0).round() / 1000000000.0,
            expected_earth_state.eccentricity
        );
        assert_eq!(
            (earth_state.obliquity_of_the_ecliptic * 100000.0).round() / 100000.0,
            expected_earth_state.obliquity_of_the_ecliptic
        );
    }
}
