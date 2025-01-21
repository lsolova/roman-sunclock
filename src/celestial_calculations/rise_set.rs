use crate::{
    celestial_calculations::{
        julian_time::julian_day_to_unix_milliseconds, utils::{consolidate_to_0_360, consolidate_transit_time},
    },
    types::{NormalDayAndNight, RiseSetInfo},
};

pub const GEOMETRIC_ALTITUDE_OF_BODY_CENTER_OF_PLANETS: f64 = -0.5667;
pub const GEOMETRIC_ALTITUDE_OF_BODY_CENTER_OF_SUN: f64 = -0.8333;

pub fn calculate_rise_set(
    apparent_right_ascension_deg: f64,
    apparent_declination_deg: f64,
    mean_sidereal_time: f64,
    lat_deg: f32,
    lon_deg: f32,
    geometric_altitude_of_body_center: f64,
    day_start_unix_epoch: i64,
) -> RiseSetInfo {
    let right_ascension_rad = apparent_right_ascension_deg.to_radians();
    let declination_rad = apparent_declination_deg.to_radians();
    let lat_rad = (lat_deg as f64).to_radians();
    let lon_rad = (lon_deg as f64).to_radians();
    // H
    let hour_angle_cos_rad = (geometric_altitude_of_body_center.to_radians().sin()
        - (lat_rad.sin() * declination_rad.sin()))
        / (lat_rad.cos() * declination_rad.cos());

    println!("h0 {:?}", geometric_altitude_of_body_center);
    println!("lat {:?}", lat_rad);
    println!("lon {:?}", lon_rad);
    println!("δ {:?} {:?}", apparent_declination_deg, declination_rad);
    println!("α {:?} {:?}", apparent_right_ascension_deg, right_ascension_rad);
    println!("cosH {:?}", hour_angle_cos_rad);
    println!("Θ {:?}", mean_sidereal_time);

    if hour_angle_cos_rad <= -1.0 {
        RiseSetInfo::CircumpolarDay
        // >1 night
    } else if hour_angle_cos_rad >= 1.0 {
        RiseSetInfo::CircumpolarNight
    } else {
        let hour_angle_rad = hour_angle_cos_rad.acos();
        let hour_angle_deg = hour_angle_rad.to_degrees();
        // m0
        let transit_time =
            consolidate_transit_time((apparent_right_ascension_deg + (lon_deg as f64) - mean_sidereal_time) / 360.0);
        // m1
        let rise_time = consolidate_transit_time(transit_time - (hour_angle_deg / 360.0));
        // m2
        let set_time = consolidate_transit_time(transit_time + (hour_angle_deg / 360.0));

        let rise_time_hour = consolidate_to_0_360(rise_time) * 24.0;
        // let transit_time_hour = consolidate_to_0_360(transit_time) * 24.0;
        let set_time_hour = consolidate_to_0_360(set_time) * 24.0;

        println!("H {:?}", hour_angle_deg);
        println!("m0 {:?}", transit_time);
        println!("m1 {:?}", rise_time);
        println!("m2 {:?}", set_time);
        println!("rise {:?}", consolidate_to_0_360(rise_time) * 24.0);
        println!("transit {:?}", consolidate_to_0_360(transit_time) * 24.0);
        println!("set {:?}", consolidate_to_0_360(set_time) * 24.0);

        RiseSetInfo::NormalDayAndNight(NormalDayAndNight {
            sunrise_epoch:((rise_time_hour * 3600000.0).round() as i64) + day_start_unix_epoch,
            sunset_epoch: ((set_time_hour * 3600000.0).round() as i64) + day_start_unix_epoch,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::celestial_calculations::{earth, julian_time::init_time_info, moon, sun};

    use super::*;

    #[test]
    fn meeus_example() {
        let lat: f32 = 42.3333;
        let lon: f32 = -71.0833; // GPS negated in Meeus
        let unix_epoch = 574819200000; // 1988-03-20 UTC
        let current_time_info = init_time_info(unix_epoch);
        let rise_set_info = calculate_rise_set(
            41.73129,
            18.44092,
            current_time_info.mean_sidereal_time,
            lat,
            lon,
            GEOMETRIC_ALTITUDE_OF_BODY_CENTER_OF_PLANETS,
            current_time_info.day_start_unix_epoch,
        );
        println!("{:?}", rise_set_info);
    }

    #[test]
    fn celestial_programming_sun_example_1() {
        let lat: f32 = 36.695063085114654;
        let lon: f32 = 4.4538146350563546; // GPS negated in Meeus
        let unix_epoch = 1737317257979;
        let current_time_info = init_time_info(unix_epoch);
        let earth_state = earth::calculate_earth_state(current_time_info.julian_century);
        let sun_state = sun::calculate_sun_state(current_time_info.julian_century, &earth_state);
        let rise_set_info = calculate_rise_set(
            sun_state.right_ascension, //302.7852410043696,
            sun_state.declination, //-20.022844800277305,
            current_time_info.mean_sidereal_time,
            lat,
            lon,
            GEOMETRIC_ALTITUDE_OF_BODY_CENTER_OF_SUN,
            current_time_info.day_start_unix_epoch
        );
        /* jd 2460695.3346420047
        T 0.25052250902134665
        Θ 119.62686795415357
        ε 23.436033272282497
        e 0.016698094833402732
        h0 -0.8333
        lat 0.6676355768391254
        lon -1.496768171657855
        dec -0.3495979659416806
        RA 5.283955225545689
        cosH 0.2677219084583606
        H 74.47124800944448
        Θ 119.62686795415357
        m0 0.746888790975609
        m1 0.5400242131715965
        m2 0.9537533687796215
        rise 12.960581116118316
        transit 17.925330983414618
        set 22.890080850710916 */
        println!("{:?}", rise_set_info);
    }

    #[test]
    fn celestial_programming_moon_example_1() {
        let lat: f32 = 36.695063085114654;
        let lon: f32 = 4.4538146350563546; // GPS negated in Meeus
        let unix_epoch = 1737317257979;
        let current_time_info = init_time_info(unix_epoch);
        let earth_state = earth::calculate_earth_state(current_time_info.julian_century);
        let sun_state = sun::calculate_sun_state(current_time_info.julian_century, &earth_state);
        let moon_state = moon::calculate_moon_state(current_time_info.julian_century, sun_state.mean_anomaly, earth_state.obliquity_of_the_ecliptic);
        let rise_set_info = calculate_rise_set(
            moon_state.right_ascension, //302.7852410043696,
            moon_state.declination, //-20.022844800277305,
            current_time_info.mean_sidereal_time,
            lat,
            lon,
            GEOMETRIC_ALTITUDE_OF_BODY_CENTER_OF_SUN,
            current_time_info.day_start_unix_epoch
        );
        /* jd 2460695.3346420047
        T 0.25052250902134665
        Θ 119.62686795415357
        ε 23.436033272282497
        e 0.016698094833402732
        h0 -0.8333
        lat 0.6676355768391254
        lon -1.496768171657855
        dec -0.3495979659416806
        RA 5.283955225545689
        cosH 0.2677219084583606
        H 74.47124800944448
        Θ 119.62686795415357
        m0 0.746888790975609
        m1 0.5400242131715965
        m2 0.9537533687796215
        rise 12.960581116118316
        transit 17.925330983414618
        set 22.890080850710916 */
        println!("{:?}", rise_set_info);
    }

}
