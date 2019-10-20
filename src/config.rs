#![allow(dead_code)]

use std::ops::Range;

pub const WINDOW_WIDTH: u32 = 1600;
pub const WINDOW_HEIGHT: u32 = 900;

pub const SCREEN_RANGE: Range<f64> = -1.0..1.00000001;

pub const ITERATIONS: i32 = 8;
pub const STEPS: i32 = 5;
pub const DELTA_PER_STEP: f64 = 1e-5;
pub const DELTA_MINIMUM: f64 = 1e-7;
pub const T_START: f64 = -3.0;
pub const T_END: f64 = 3.0;
