#![allow(dead_code)]

use crate::config::{DELTA_PER_STEP, T_START};

#[derive(Clone, Copy)]
pub struct Globals {
    t: f64,
    rolling_delta: f64,
    speed_multiplier: f64,
    scale_factor: f64,
    point_size: usize,
}

impl Globals {
    pub fn new() -> Globals {
        Globals {
            t: T_START,
            rolling_delta: DELTA_PER_STEP,
            speed_multiplier: 1.0,
            scale_factor: 0.25,
            point_size: 1,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn increase_t(&mut self, additional_value: f64) {
        self.t += additional_value;
    }

    pub fn reset_t(&mut self) {
        self.t = T_START;
    }

    pub fn rolling_delta(&self) -> f64 {
        self.rolling_delta
    }

    pub fn set_rolling_delta(&mut self, new_rolling_delta: f64) {
        self.rolling_delta = new_rolling_delta;
    }

    pub fn speed_multiplier(&self) -> f64 {
        self.speed_multiplier
    }

    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    pub fn point_size(&self) -> usize {
        self.point_size
    }
}
