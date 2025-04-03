use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::models::{coordinate::Coordinate, state::State};

pub fn to_screen(state: &State, x: f64, y: f64) -> Coordinate {
    let s = state.scale_factor * (WINDOW_HEIGHT / 2) as f64;
    let nx = WINDOW_WIDTH as f64 * 0.5 + x * s;
    let ny = WINDOW_HEIGHT as f64 * 0.5 + y * s;
    Coordinate::new_with_values(nx, ny)
}
