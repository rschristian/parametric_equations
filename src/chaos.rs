use crate::config::{DELTA_PER_STEP, SCREEN_RANGE, ITERATIONS};
use crate::models::globals::Globals;
use crate::models::parameters::{ParamDimensions, Parameters};
use crate::visuals::utility::to_screen;
use crate::models::vertex::Vertex;

pub fn apply_chaos(globals: &mut Globals, params: Parameters, vertex_vector: &mut Vec<Vertex>) {
    let mut x = globals.t();
    let mut y = globals.t();

    for (i, vertex) in vertex_vector.iter_mut().enumerate() {
        x = calculate_new_coords((x,y), globals.t(), params.get_x_dimensions());
        y = calculate_new_coords((x,y), globals.t(), params.get_y_dimensions());

        let screen_point = to_screen(*globals, x, y);
        vertex.convert_to_gl(screen_point);

        if (i as u32 + 1) % ITERATIONS as u32 == 0 {
            if SCREEN_RANGE.contains(&vertex.get_position().x())
                && SCREEN_RANGE.contains(&vertex.get_position().y())
            {
                globals.increase_t(DELTA_PER_STEP * globals.speed_multiplier());
            } else {
                globals.increase_t(0.01 * globals.speed_multiplier());
            }
            x = globals.t();
            y = globals.t();
        }
    }
}

/// Calculates a new coordinate value for a given t and (randomized) parameter values
fn calculate_new_coords(xy: (f64, f64), t: f64, params: ParamDimensions) -> f64 {
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
    xx * params.get_xx()
        + yy * params.get_yy()
        + tt * params.get_tt()
        + xy * params.get_xy()
        + xt * params.get_xt()
        + yt * params.get_yt()
        + x * params.get_x()
        + y * params.get_y()
        + t * params.get_t()
}
