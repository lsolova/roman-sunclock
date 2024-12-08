use std::fmt::{Debug, Display};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DayType {
    FullDay = "fullDay",
    FullNight = "fullNight",
    NormalDay = "normalDay",
    NormalNight = "normalNight",
}

impl Default for DayType {
    fn default() -> Self {
        DayType::NormalDay
    }
}

impl Display for DayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct RomanTimeDetails {
    pub hours: i32,
    pub minutes: i32,
    pub last_sun_change: Option<i64>,
    pub next_sun_change: Option<i64>,
    pub minute_length: f32,
    pub day_type: DayType,
}

impl Display for RomanTimeDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.hours, self.minutes)
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, PartialEq)]
pub struct RomanSunclockResult {
    pub time_details: RomanTimeDetails,
    pub clock_svg: String,
}
