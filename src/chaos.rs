use crate::config::{DELTA_PER_STEP, SCREEN_RANGE};
use crate::models::globals::Globals;
use crate::models::parameters::{ParamDimensions, Parameters};
use crate::models::shape::Shapes;
use crate::visuals::utility::to_screen;

pub fn apply_chaos(globals: &mut Globals, params: Parameters, vertex_vector: &mut Vec<Shapes>) {
    for (i, point) in vertex_vector.iter_mut().enumerate() {
        let nx = calculate_new_coords(globals.t(), params.get_x_dimensions());
        let ny = calculate_new_coords(globals.t(), params.get_y_dimensions());

        let screen_point = to_screen(*globals, nx, ny);
        point[0].convert_to_gl(screen_point);

        if SCREEN_RANGE.contains(&point[0].get_position().x())
            && SCREEN_RANGE.contains(&point[0].get_position().y())
        {
            globals.increase_t(DELTA_PER_STEP * globals.speed_multiplier());
        } else {
            globals.increase_t(0.01 * globals.speed_multiplier());
        }
    }
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
