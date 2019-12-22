use crate::models::{globals::Globals, vertex::Vertex};
use glium::Surface;

pub mod objects;
pub mod text;
pub mod utility;

/// Draws everything to the screen
///
/// * `equation_text` - A reference to a tuple that contains the strings for the x & y prime equations
/// * `globals` - A struct of global values that includes the current t value, speed multiplier, and text properties
/// * `vertex_vector` - A reference to a vector of vertices that are drawn into the lines
///
pub fn draw_all(
    equation_text: &(String, String),
    globals: &mut Globals,
    vertex_vector: &mut Vec<Vertex>,
) {
    let mut target = globals.display().draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

    objects::draw_vertices(globals, &mut target, &vertex_vector);

    text::draw_equation_text(equation_text, globals, &mut target);
    text::draw_time_text(globals, &mut target);
    text::draw_speed_multiplier_text(globals, &mut target);
    text::draw_point_size_text(globals, &mut target);

    target.finish().unwrap();
}
