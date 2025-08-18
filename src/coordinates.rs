#[derive(PartialEq)]
enum CoordinateType {
    HelioEcliptic,
    GeoEcliptic,
    GeoEquatorial,
}

struct Coordinate {
    c_type: CoordinateType,
    x: f64,
    y: f64,
    z: f64,
}

pub fn from_ecliptic_to_equatorial(coord: Coordinate, earth_obliquity: f64) -> Coordinate {
    if coord.c_type != CoordinateType::GeoEcliptic {
        panic!("This coordinate conversion might fail")
    }
    Coordinate {
        c_type: CoordinateType::GeoEquatorial,
        x: coord.x,
        y: coord.y * earth_obliquity.cos() - coord.z * earth_obliquity.sin(),
        z: coord.y * earth_obliquity.sin() - coord.z * earth_obliquity.cos(),
    }
}
