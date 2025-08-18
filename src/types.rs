use std::{cmp::Ordering, fmt::Debug};

use crate::wasm_types::DayType;

pub struct GeoLocation {
    lat: f32,
    lon: f32,
    alt: f32, 
}

impl GeoLocation {
    pub fn new(lat: f32, lon: f32, alt: f32) -> GeoLocation {
        if lat < -90.0 || lat > 90.0 {
            panic!("Invalid latitude");
        }
        if lon < -180.0 || lon > 180.0 {
            panic!("Invalid longitude");
        }
        GeoLocation { lat,lon,alt }
    }
    pub fn lat(&self) -> f32 { self.lat }
    pub fn lon(&self) -> f32 { self.lon }
    pub fn alt(&self) -> f32 { self.alt }
}

impl Debug for GeoLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Location: lat {:?}, lon {:?}, alt {:?}", self.lat, self.lon, self.alt)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TimeType {
    Sunrise,
    Sunset,
}

#[derive(Clone, Copy, Debug)]
pub struct PointOfTime {
    pub time_type: TimeType,
    pub epoch: i64,
}

impl Ord for PointOfTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.epoch.cmp(&other.epoch)
    }
}
impl PartialOrd for PointOfTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for PointOfTime {
    fn eq(&self, other: &Self) -> bool {
        self.epoch == other.epoch
    }
}
impl Eq for PointOfTime {}

#[derive(Debug)]
pub struct NormalDayAndNight {
    /// Rise time in Julian day
    pub rise_time: f64,
    /// Set time in Julian day
    pub set_time: f64,
}

#[derive(Debug)]
pub enum RiseSetInfo {
    NormalDayAndNight(NormalDayAndNight),
    CircumpolarDay,
    CircumpolarNight,
}

pub struct Timeline {
    pub day_type: DayType,
    pub day_start_epoch: i64,
    pub last_sun_change: Option<PointOfTime>,
    pub next_sun_change: Option<PointOfTime>,
}
