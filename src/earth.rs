use std::fmt::Debug;

#[derive(PartialEq)]
pub struct EarthState {
    /** e - Eccentricity.
     *  https://en.wikipedia.org/wiki/Orbital_eccentricity */
    pub eccentricity: f64,
    /** ε - Obliquity of the ecliptic or axial tilt at the provided time. In radians.
     *  https://en.wikipedia.org/wiki/Axial_tilt */
    pub obliquity_of_the_ecliptic: f64,
}

impl From<f64> for EarthState {
    fn from(current_julian_century: f64) -> Self {
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
            + 2.45
                * u_time
                * u_time
                * u_time
                * u_time
                * u_time
                * u_time
                * u_time
                * u_time
                * u_time
                * u_time;
        // e
        let eccentricity = 0.016708634
            - (0.000042037 * current_julian_century)
            - (0.0000001267 * pow2_current_julian_century);

        EarthState {
            eccentricity,
            obliquity_of_the_ecliptic: earth_obliquity_of_the_ecliptic_deg.to_radians(),
        }
    }
}

impl Debug for EarthState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EarthState (ε {:?}, e {:?})",
            self.obliquity_of_the_ecliptic, self.eccentricity
        )
    }
}
#[cfg(test)]
mod test {
    use super::*;

    fn rounding(roundee: f64, rounder: f64) -> f64 {
        (roundee * rounder).round() / rounder
    }

    #[test]
    fn test_meeus_example() {
        let expected_earth_state = EarthState {
            eccentricity: 0.016711668,
            obliquity_of_the_ecliptic: rounding((23.44023 as f64).to_radians(), 100000.0),
        };
        let current_julian_century_time = -0.072183436;
        let earth_state = EarthState::from(current_julian_century_time);
        assert_eq!(
            rounding(earth_state.eccentricity, 1000000000.0),
            expected_earth_state.eccentricity
        );
        assert_eq!(
            rounding(earth_state.obliquity_of_the_ecliptic, 100000.0),
            expected_earth_state.obliquity_of_the_ecliptic
        );
    }
}
