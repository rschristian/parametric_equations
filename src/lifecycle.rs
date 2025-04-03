extern crate rand;

use rand::Rng;

use crate::models::state::State;
use crate::models::vertex::create_vertex_slice;
use crate::models::vertex::Vertex;

pub type Parameters = ([i8; 9], [i8; 9]);

pub fn initialize(state: &mut State) -> (Parameters, (String, String), Vec<Vertex>) {
    state.reset_t();

    let params = (rand_params(), rand_params());

    let mut x_prime = "x' = ".to_owned();
    let mut y_prime = "y' = ".to_owned();
    x_prime.push_str(&build_equation_string(params.0));
    y_prime.push_str(&build_equation_string(params.1));

    let equation_strings = (x_prime, y_prime);

    let vertex_vector = create_vertex_slice();

    (params, equation_strings, vertex_vector)
}

const NUMBER_OF_PARAMS: usize = 9;

fn rand_params() -> [i8; NUMBER_OF_PARAMS] {
    let mut rng = rand::thread_rng();

    core::array::from_fn(|_| rng.gen_range(-1, 2))
}

fn build_equation_string(params: [i8; 9]) -> String {
    let mut equation_string = String::new();
    let mut is_first = true;

    let equation_variables = [
        "x\u{b2}", "y\u{b2}", "t\u{b2}", "xy", "xt", "yt", "x", "y", "t",
    ];

    for i in 0..equation_variables.len() {
        if params[i] != 0 {
            if is_first {
                if params[i] == -1 {
                    equation_string.push('-');
                }
            } else if params[i] == -1 {
                equation_string.push_str(" - ");
            } else {
                equation_string.push_str(" + ");
            }

            equation_string.push_str(equation_variables[i]);
            is_first = false;
        }
    }
    equation_string.push('\n');
    equation_string
}
