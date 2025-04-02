use crate::config::{DELTA_PER_STEP, ITERATIONS, STEPS};
use crate::models::{
    globals::Globals,
    parameters::{ParamDimensions, Parameters},
    vertex::Vertex,
};
use crate::visuals::{self, utility};
use std::ops::Range;

// Technically the upper bound is exclusive, while the lower is inclusive, so while I could make the upper
// bound something like 1.0000000000001, it doesn't seem necessary
pub const SCREEN_RANGE: Range<f64> = -1.0..1.0;

/// Applies the logic used to move the points, then renders the screen
///
/// * `equation_text` - A reference to a tuple that contains the strings for the x & y prime equations
/// * `globals` - A struct of global values that includes the current t value and text properties
/// * `params` - A reference to the parameters (-1,0,1) that decide the random makeup of the equations
/// * `vertex_vector` - A reference to a vector of vertices that are drawn into the lines
pub fn apply_chaos(
    equation_text: &(String, String),
    globals: &mut Globals,
    params: &Parameters,
    vertex_vector: &mut [Vertex],
) {
    for step in 0..STEPS {
        let mut x = globals.t();
        let mut y = globals.t();
        let mut is_on_screen = false;

        for iter in 0..ITERATIONS {
            x = calculate_new_coords((x, y), globals.t(), params.x_dimensions());
            y = calculate_new_coords((x, y), globals.t(), params.y_dimensions());

            let screen_point = utility::to_screen(globals, x, y);
            let current_index = step * ITERATIONS + iter;
            vertex_vector[current_index as usize]
                .set_position_within_window_dimensions(screen_point);

            if SCREEN_RANGE.contains(&vertex_vector[current_index as usize].position().x())
                && SCREEN_RANGE.contains(&vertex_vector[current_index as usize].position().y())
            {
                is_on_screen = true;
            }
        }

        if is_on_screen {
            globals.increase_t(DELTA_PER_STEP * globals.speed_multiplier());
        } else {
            globals.increase_t(0.01 * globals.speed_multiplier());
        }
    }

    visuals::draw_all(equation_text, globals, vertex_vector);
}

/// Calculates a new coordinate value for a given t and (randomized) parameter values
fn calculate_new_coords(xy: (f64, f64), t: f64, params: &ParamDimensions) -> f64 {
    let x: f64 = xy.0;
    let y: f64 = xy.1;

    // Sets variables as humans would understand them in a mathematical context.
    let xx: f64 = x * x;
    let yy: f64 = y * y;
    let tt: f64 = t * t;
    let xy: f64 = x * y;
    let xt: f64 = x * t;
    let yt: f64 = y * t;

    // Creates the new coordinate using the random equations. Since the params are either -1, 0, or 1,
    // this creates either -xx, 0*xx (which is 0) or xx for each item, making random parametric equations
    xx * params.xx()
        + yy * params.yy()
        + tt * params.tt()
        + xy * params.xy()
        + xt * params.xt()
        + yt * params.yt()
        + x * params.x()
        + y * params.y()
        + t * params.t()
}
