use crate::constants::{DELTA_PER_STEP, ITERATIONS, STEPS};
use crate::lifecycle::Parameters;
use crate::models::{state::State, vertex::Vertex};
use crate::visuals::utility;
use std::ops::Range;

// Technically the upper bound is exclusive, while the lower is inclusive, so while I could make the upper
// bound something like 1.0000000000001, it doesn't seem necessary
pub const SCREEN_RANGE: Range<f64> = -1.0..1.0;

pub fn apply_chaos(state: &mut State, params: &Parameters, vertex_vector: &mut [Vertex]) {
    for step in 0..STEPS {
        let mut x = state.t;
        let mut y = state.t;
        let mut is_on_screen = false;

        for iter in 0..ITERATIONS {
            (x, y) = calculate_new_coords((x, y), state.t, params);

            let screen_point = utility::to_screen(state, x, y);
            let current_index = step * ITERATIONS + iter;
            vertex_vector[current_index as usize]
                .set_position_within_window_dimensions(screen_point);

            if SCREEN_RANGE.contains(&vertex_vector[current_index as usize].position.x)
                && SCREEN_RANGE.contains(&vertex_vector[current_index as usize].position.y)
            {
                is_on_screen = true;
            }
        }

        if is_on_screen {
            state.increase_t(DELTA_PER_STEP * state.speed_multiplier);
        } else {
            state.increase_t(0.01 * state.speed_multiplier);
        }
    }
}

fn calculate_new_coords(xy: (f64, f64), t: f64, params: &Parameters) -> (f64, f64) {
    let (x, y) = xy;
    let (params_x, params_y) = params;

    let xx = x * x;
    let yy = y * y;
    let tt = t * t;
    let xy = x * y;
    let xt = x * t;
    let yt = y * t;

    let nx = xx * (params_x[0] as f64)
        + yy * (params_x[1] as f64)
        + tt * (params_x[2] as f64)
        + xy * (params_x[3] as f64)
        + xt * (params_x[4] as f64)
        + yt * (params_x[5] as f64)
        + x * (params_x[6] as f64)
        + y * (params_x[7] as f64)
        + t * (params_x[8] as f64);

    let ny = xx * (params_y[0] as f64)
        + yy * (params_y[1] as f64)
        + tt * (params_y[2] as f64)
        + xy * (params_y[3] as f64)
        + xt * (params_y[4] as f64)
        + yt * (params_y[5] as f64)
        + x * (params_y[6] as f64)
        + y * (params_y[7] as f64)
        + t * (params_y[8] as f64);

    (nx, ny)
}
