use crate::celestial_calculations::{
    moon_constants::{LATITUDE_TABLE, LONGITUDE_RADIUS_TABLE, NUTATION_TABLE},
    utils::consolidate_to_0_360,
};

const DEGREE_CALC: f64 = 1.0 / 3600.0 * 0.0001;

#[derive(Debug, PartialEq)]
pub struct MoonState {
    /** Declination in degrees.
     *  https://en.wikipedia.org/wiki/Declination */
    pub declination: f64,
    /** Right ascension in degrees. Consolidated between 0 - 360°.
     *  https://en.wikipedia.org/wiki/Right_ascension */
    pub right_ascension: f64,
}

pub fn calculate_moon_state(
    current_julian_century: f64,
    sun_mean_anomaly_deg: f64,
    earth_obliquity_deg: f64,
) -> MoonState {
    //L'
    let moon_mean_longitude_deg = consolidate_to_0_360(
        218.3164477 + 481267.88123421 * current_julian_century
            - 0.0015786 * current_julian_century * current_julian_century
            + 1.0 / 538841.0
                * current_julian_century
                * current_julian_century
                * current_julian_century
            - 1.0 / 65194000.0
                * current_julian_century
                * current_julian_century
                * current_julian_century
                * current_julian_century,
    );
    // D
    let moon_mean_elongation_deg = consolidate_to_0_360(
        297.8501921 + 445267.1114034 * current_julian_century
            - 0.0018819 * current_julian_century * current_julian_century
            + 1.0 / 545868.0
                * current_julian_century
                * current_julian_century
                * current_julian_century
            - 1.0 / 113065000.0
                * current_julian_century
                * current_julian_century
                * current_julian_century
                * current_julian_century,
    );
    // M'
    let moon_mean_anomaly_deg = consolidate_to_0_360(
        134.9633964
            + 477198.8675055 * current_julian_century
            + 0.0087414 * current_julian_century * current_julian_century
            + 1.0 / 69699.0
                * current_julian_century
                * current_julian_century
                * current_julian_century
            - 1.0 / 14712000.0
                * current_julian_century
                * current_julian_century
                * current_julian_century
                * current_julian_century,
    );
    // F
    let moon_arg_of_latitude_deg = consolidate_to_0_360(
        93.2720950 + 483202.0175233 * current_julian_century
            - 0.0036539 * current_julian_century * current_julian_century
            - 1.0 / 3526000.0
                * current_julian_century
                * current_julian_century
                * current_julian_century
            + 1.0 / 863310000.0
                * current_julian_century
                * current_julian_century
                * current_julian_century
                * current_julian_century,
    );
    // Further arguments
    let e = 1.0
        - 0.002516 * current_julian_century
        - 0.0000074 * current_julian_century * current_julian_century;
    let a1_deg = consolidate_to_0_360(119.75 + 131.849 * current_julian_century);
    let a2_deg = consolidate_to_0_360(53.09 + 479264.290 * current_julian_century);
    let a3_deg = consolidate_to_0_360(313.45 + 481266.484 * current_julian_century);

    // Σb
    let mut latitude_sum = 0.0;
    // Σl
    let mut longitude_sum = 0.0;
    // Σr
    let mut radius_sum = 0.0;
    let moon_mean_elongation_rad = moon_mean_elongation_deg.to_radians();
    let sun_mean_anomaly_rad = sun_mean_anomaly_deg.to_radians();
    let moon_mean_anomaly_rad = moon_mean_anomaly_deg.to_radians();
    let moon_arg_of_latitude_rad = moon_arg_of_latitude_deg.to_radians();

    for row in LATITUDE_TABLE {
        let argsum = moon_mean_elongation_rad * row[0]
            + sun_mean_anomaly_rad * row[1]
            + moon_mean_anomaly_rad * row[2]
            + moon_arg_of_latitude_rad * row[3];
        let coefficient_modifier = if row[1] == 1.0 || row[1] == -1.0 {
            e
        } else if row[1] == 2.0 || row[1] == -2.0 {
            e * e
        } else {
            1.0
        };
        latitude_sum += coefficient_modifier * row[4] * argsum.sin();
    }

    for row in LONGITUDE_RADIUS_TABLE {
        let argsum = moon_mean_elongation_rad * row[0]
            + sun_mean_anomaly_rad * row[1]
            + moon_mean_anomaly_rad * row[2]
            + moon_arg_of_latitude_rad * row[3];
        let coefficient_modifier = if row[1] == 1.0 || row[1] == -1.0 {
            e
        } else if row[1] == 2.0 || row[1] == -2.0 {
            e * e
        } else {
            1.0
        };
        longitude_sum += coefficient_modifier * row[4] * argsum.sin();
        radius_sum += coefficient_modifier * row[5] * argsum.cos    ();
    }

    longitude_sum += 3958.0 * a1_deg.to_radians().sin()
        + 1962.0
            * (moon_mean_longitude_deg - moon_arg_of_latitude_deg)
                .to_radians()
                .sin()
        + 318.0 * a2_deg.to_radians().sin();

    latitude_sum += -2235.0 * moon_mean_longitude_deg.to_radians().sin()
        + 382.0 * a3_deg.to_radians().sin()
        + 175.0 * (a1_deg - moon_arg_of_latitude_deg).to_radians().sin()
        + 175.0 * (a1_deg + moon_arg_of_latitude_deg).to_radians().sin()
        + 127.0
            * (moon_mean_longitude_deg - moon_mean_anomaly_deg)
                .to_radians()
                .sin()
        - 115.0
            * (moon_mean_longitude_deg + moon_mean_anomaly_deg)
                .to_radians()
                .sin();

    let obliquity_rad = earth_obliquity_deg.to_radians();
    // Ω
    let mean_equinox_deg = 125.04452 - 1934.136261 * current_julian_century
        + 0.0020708 * current_julian_century * current_julian_century
        + (current_julian_century * current_julian_century * current_julian_century) / 450000.0;
    let mean_equinox_rad = mean_equinox_deg.to_radians();

    // Δψ
    let mut nutation_diff_deg = 0.0;
    // Δε
    let mut obliquity_diff_deg = 0.0;
    for row in NUTATION_TABLE {
        let argsum = moon_mean_elongation_rad * row[0]
            + sun_mean_anomaly_rad * row[1]
            + moon_mean_anomaly_rad * row[2]
            + moon_arg_of_latitude_rad * row[3]
            + mean_equinox_rad * row[4];

        nutation_diff_deg += argsum.sin() * (row[5] + row[6] * current_julian_century);
        obliquity_diff_deg += argsum.cos() * (row[7] + row[8] * current_julian_century);
    }

    // β
    let final_latitude_deg = latitude_sum / 1000000.0;
    // λ
    let final_longitude_deg = moon_mean_longitude_deg + (longitude_sum / 1000000.0) + (nutation_diff_deg * DEGREE_CALC);
    // Δ
    let final_radius = 385000.56 + radius_sum / 1000.0;
    // ε
    let true_obliquity_rad = obliquity_rad + (obliquity_diff_deg * DEGREE_CALC).to_radians();

    // α
    let right_ascension = ((final_longitude_deg.to_radians().sin() * true_obliquity_rad.cos())
        - (final_latitude_deg.to_radians().tan() * true_obliquity_rad.cos()))
        / final_longitude_deg.to_radians().cos();
    // δ
    let declination = final_latitude_deg.to_radians().sin() * true_obliquity_rad.cos()
        + final_latitude_deg.to_radians().cos()
            * final_longitude_deg.to_radians().sin()
            * true_obliquity_rad.sin();

    println!("Moon ---------------");
    println!("ε0 {:?}", earth_obliquity_deg);
    println!("L' {:?}", moon_mean_longitude_deg);
    println!("D  {:?}", moon_mean_elongation_deg);
    println!("M  {:?}", sun_mean_anomaly_deg);
    println!("M' {:?}", moon_mean_anomaly_deg);
    println!("F  {:?}", moon_arg_of_latitude_deg);
    println!("A* {:?} {:?} {:?}", a1_deg, a2_deg, a3_deg);
    println!("E  {:?}", e);
    println!("Σb {:?}", latitude_sum);
    println!("Σl {:?}", longitude_sum);
    println!("Σr {:?}", radius_sum);
    println!("Δψ {:?}", nutation_diff_deg * DEGREE_CALC);
    println!("Δε {:?}", obliquity_diff_deg * DEGREE_CALC);
    println!("ε  {:?}", true_obliquity_rad.to_degrees());
    println!("β  {:?}", final_latitude_deg);
    println!("λ  {:?}", final_longitude_deg);
    println!("Δ  {:?}", final_radius);
    println!("Ω  {:?}", mean_equinox_deg);
    println!("α  {:?}", consolidate_to_0_360(right_ascension.to_degrees()));
    println!("δ  {:?}", declination.to_degrees());

    MoonState {
        declination,
        right_ascension,
    }
}

#[cfg(test)]
mod test {
    use crate::celestial_calculations::{
        earth::calculate_earth_state, julian_time::init_time_info, sun::calculate_sun_state,
    };

    use super::*;

    #[test]
    fn meeus_example() {
        let julian_century = -0.077221081451;
        let earth_state = calculate_earth_state(julian_century);
        let sun_state = calculate_sun_state(julian_century, &earth_state);
        let moon_state = calculate_moon_state(
            julian_century,
            sun_state.mean_anomaly,
            earth_state.obliquity_of_the_ecliptic,
        );
    }
}
