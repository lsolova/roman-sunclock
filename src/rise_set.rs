use crate::types::{NormalDayAndNight, RiseSetInfo};

/**
# Arguments

* `declination` - `δ` Declination of body, in radians
* `geometric_altitude_of_body_center` - `h0` Geometric altitude of body center, in radians
* `lat` - Latitude, in radians
* `transit_time` - `m0` Transit time, in degrees
*/
pub fn calculate_rise_set(
    declination: f64,
    geometric_altitude_of_body_center: f64,
    lat: f64,
    // local_sidereal_time: f64, Θ
    // right_ascension: f64, α
    local_noon_time: f64
) -> RiseSetInfo {
    let hour_angle_cos = (geometric_altitude_of_body_center.sin()
        - (lat.sin() * declination.sin()))
        / (lat.cos() * declination.cos());

    // < -1 day
    if hour_angle_cos <= -1.0 {
        RiseSetInfo::CircumpolarDay
    // > 1 night
    } else if hour_angle_cos >= 1.0 {
        RiseSetInfo::CircumpolarNight
    } else {
        let hour_angle = hour_angle_cos.acos().to_degrees();
        println!("V1 H {:?}", hour_angle);

        let rise_time = local_noon_time - (hour_angle / 360.0);
        let set_time = local_noon_time + (hour_angle / 360.0);

        RiseSetInfo::NormalDayAndNight(NormalDayAndNight {
            rise_time,
            set_time,
        })
    }
}

fn consolidate_transit_time(transit_time_raw: f64) -> f64 {
    if transit_time_raw < 0.0 {
        transit_time_raw + 1.0
    } else if transit_time_raw > 1.0 {
        transit_time_raw - 1.0
    } else {
        transit_time_raw
    }
}

fn consolidate_to_0_360(original_deg: f64) -> f64 {
    let remaining = original_deg % 360.0;
    if remaining < 0.0 {
        remaining + 360.0
    } else {
        remaining
    }
}

pub fn calculate_rise_set_v2(
    // In radians.
    apparent_right_ascension: f64,
    // In radians.
    apparent_declination: f64,
    // In radians.
    mean_sidereal_time: f64,
    lat_deg: f32,
    lon_deg: f32,
    geometric_altitude_of_body_center: f64,
    // day_start_unix_epoch: i64,
) -> RiseSetInfo {
    // let right_ascension_rad = apparent_right_ascension;
    let declination_rad = apparent_declination;
    let lat_rad = (lat_deg as f64).to_radians();
    // let lon_rad = (lon_deg as f64).to_radians();
    // H
    let hour_angle_cos_rad = (geometric_altitude_of_body_center.to_radians().sin()
        - (lat_rad.sin() * declination_rad.sin()))
        / (lat_rad.cos() * declination_rad.cos());

    // println!("h0 {:?}", geometric_altitude_of_body_center);
    // println!("lat {:?}", lat_rad);
    // println!("lon {:?}", lon_rad);
    // println!("δ {:?}", declination_rad);
    // println!("α {:?}", right_ascension_rad);
    // println!("cosH {:?}", hour_angle_cos_rad);
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
        let transit_time_deg = consolidate_transit_time(
            (apparent_right_ascension.to_degrees() + (lon_deg as f64)
                - mean_sidereal_time.to_degrees())
                / 360.0,
        );
        // m1
        let rise_time = consolidate_transit_time(transit_time_deg - (hour_angle_deg / 360.0));
        // m2
        let set_time = consolidate_transit_time(transit_time_deg + (hour_angle_deg / 360.0));

        let rise_time_hour = consolidate_to_0_360(rise_time) * 24.0;
        // let transit_time_hour = consolidate_to_0_360(transit_time) * 24.0;
        let set_time_hour = consolidate_to_0_360(set_time) * 24.0;

        println!("H {:?}", hour_angle_deg);
        println!("m0 {:?}", transit_time_deg);
        println!("m1 {:?}", rise_time);
        println!("m2 {:?}", set_time);
        println!("rise {:?} h", rise_time_hour);
        println!(
            "transit {:?} h",
            consolidate_to_0_360(transit_time_deg) * 24.0
        );
        println!("set {:?} h", set_time_hour);

        RiseSetInfo::NormalDayAndNight(NormalDayAndNight {
            rise_time,
            set_time,
            // sunrise_epoch: ((rise_time_hour * 3600000.0).round() as i64),
            // sunset_epoch: ((set_time_hour * 3600000.0).round() as i64),
        })
    }
}
