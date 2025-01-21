pub fn consolidate_to_0_360(original_deg: f64) -> f64 {
    let remaining = original_deg % 360.0;
    if remaining < 0.0 {
        remaining + 360.0
    } else {
        remaining
    }
}

pub fn consolidate_transit_time(transit_time_raw: f64) -> f64 {
    if transit_time_raw < 0.0 {
        transit_time_raw + 1.0
    } else if transit_time_raw > 1.0 {
        transit_time_raw - 1.0
    } else {
        transit_time_raw
    }
}
