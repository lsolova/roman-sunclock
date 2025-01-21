mod earth;
mod julian_time;
mod moon;
mod moon_constants;
mod rise_set;
mod sun;
mod utils;

pub fn collectSunAndMoonInfo(unix_epoch: i64, lat: f32, lon: f32, alt: f32) {
    if lat < -90.0 || lat > 90.0 {
        panic!("Invalid latitude");
    }
    if lon < -180.0 || lon > 180.0 {
        panic!("Invalid longitude");
    }

    let current_time_info = julian_time::init_time_info(unix_epoch);
    let earth_state = earth::calculate_earth_state(current_time_info.julian_century);
    let sun_state = sun::calculate_sun_state(current_time_info.julian_century, &earth_state);
    let sun_rise_set = rise_set::calculate_rise_set(
        sun_state.right_ascension,
        sun_state.declination,
        current_time_info.mean_sidereal_time,
        lat,
        lon,
        0.0,
        current_time_info.day_start_unix_epoch,
    );
    let moon_state = moon::calculate_moon_state(
        current_time_info.julian_century,
        sun_state.mean_anomaly,
        earth_state.obliquity_of_the_ecliptic,
    );
    let moon_rise_set = rise_set::calculate_rise_set(
        moon_state.right_ascension,
        moon_state.declination,
        current_time_info.mean_sidereal_time,
        lat,
        lon,
        0.0,
        current_time_info.day_start_unix_epoch,
    );
    println!("EarthInfo {:?}", earth_state);
    println!("SunInfo {:?}", sun_state);
    println!("MoonInfo {:?}", moon_state);
}
