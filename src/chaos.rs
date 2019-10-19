use crate::models::globals::Globals;
use crate::models::parameters::{ParamDimensions, Parameters};

pub fn apply_chaos(globals: Globals, params: Parameters) -> (f64, f64) {
    let nx = calculate_new_coords(globals.t(), params.get_x_dimensions());
    let ny = calculate_new_coords(globals.t(), params.get_y_dimensions());
    (nx, ny)
}

/// Calculates a new coordinate value for a given t and (randomized) parameter values
fn calculate_new_coords(t: f64, params: ParamDimensions) -> f64 {
    let x = t;
    let y = t;

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
