use std::fmt::Display;
use wasm_bindgen::prelude::*;

pub struct SunDetails {
    pub sunrise_epoch: i64,
    pub sunset_epoch: i64
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct RomanTimeDetails {
    pub hours: i32,
    pub minutes: i32,
    pub last_sun_change: i64,
    pub next_sun_change: i64,
    pub total_minutes: i32,
    pub minute_length: f32,
    pub is_day: bool,
}
impl Display for RomanTimeDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.hours, self.minutes)
    }
}
