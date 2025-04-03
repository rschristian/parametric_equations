pub const WINDOW_WIDTH: u32 = 1600;
pub const WINDOW_HEIGHT: u32 = 900;

pub const STEPS: u32 = 500;
pub const ITERATIONS: u32 = 800;
pub const TOTAL_VERTICES: usize = (ITERATIONS * STEPS) as usize;
pub const DELTA_PER_STEP: f64 = 1e-5;
pub const T_START: f64 = -3.0;
pub const T_END: f64 = 3.0;

// Magic number value that looks about correct for a time value with 3 decimal places.
// Keeps the text static in size
pub const TITLE_TEXT_WIDTH: f32 = 16.2;

// Rather arbitrary secondary text size
pub const SUBTITLE_TEXT_WIDTH: f32 = 24.0;
