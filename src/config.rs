use std::ops::Range;

pub const WINDOW_WIDTH: u32 = 1600;
pub const WINDOW_HEIGHT: u32 = 900;

pub const SCREEN_RANGE: Range<f64> = -1.0..1.000_000_01;

pub const STEPS: i32 = 5;
pub const ITERATIONS: i32 = 8;
pub const DELTA_PER_STEP: f64 = 1e-2;
pub const T_START: f64 = -3.0;
pub const T_END: f64 = 3.0;
