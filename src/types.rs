use std::cmp::Ordering;

use crate::wasm_types::DayType;

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
    pub sunrise_epoch: i64,
    pub sunset_epoch: i64,
}

#[derive(Debug)]
pub enum SunMovementResult {
    NormalDayAndNight(NormalDayAndNight),
    FullDay,
    FullNight,
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
