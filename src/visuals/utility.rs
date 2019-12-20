use crate::config::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::models::{
    coordinate::Coordinate,
    globals::Globals,
    parameters::{ParamDimensions, Parameters},
};

pub fn reset_and_generate_new(
    globals: &mut Globals,
    equation_parameters: &Parameters,
) -> (String, String) {
    globals.reset_t();
    let mut x_prime = "x' = ".to_owned();
    let mut y_prime = "y' = ".to_owned();
    x_prime.push_str(&create_equation_string(equation_parameters.x_dimensions()));
    y_prime.push_str(&create_equation_string(equation_parameters.y_dimensions()));
    (x_prime, y_prime)
}

fn create_equation_string(params: &ParamDimensions) -> String {
    let mut equation_string = "".to_owned();

    let equation_variables = [
        "x\u{b2}", "y\u{b2}", "t\u{b2}", "xy", "xt", "yt", "x", "y", "t",
    ];
    let iter_equation_values = vec![
        params.xx(),
        params.yy(),
        params.tt(),
        params.xy(),
        params.xt(),
        params.yt(),
        params.x(),
        params.y(),
        params.t(),
    ];

    let mut is_first = true;
    for i in 0..equation_variables.len() {
        if iter_equation_values[i] as i32 != 0 {
            if is_first {
                if iter_equation_values[i] as i32 == -1 {
                    equation_string.push_str("-");
                }
            } else if iter_equation_values[i] as i32 == -1 {
                equation_string.push_str(" - ");
            } else {
                equation_string.push_str(" + ");
            }

            equation_string.push_str(equation_variables[i]);
            is_first = false;
        }
    }
    equation_string.push_str("\n");
    equation_string
}

pub fn to_screen(globals: &Globals, x: f64, y: f64) -> Coordinate {
    let s = globals.scale_factor() * (WINDOW_HEIGHT / 2) as f64;
    let nx = WINDOW_WIDTH as f64 * 0.5 + x * s;
    let ny = WINDOW_HEIGHT as f64 * 0.5 + y * s;
    Coordinate::new_with_values(nx, ny)
}
