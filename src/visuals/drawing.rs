use crate::models::globals::Globals;
use crate::models::vertex::Vertex;
use glium::{Display, DrawParameters, Frame, Surface};

/// Vertex shader required by Glium
/// Both color and position are passed in for each vertex,
/// with color being passed through to be used in the fragment shader
fn vertex_shader() -> &'static str {
    r#"
        #version 140

        in vec4 color;
        in vec2 position;

        out vec4 v_color;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            v_color = color;
        }
    "#
}

/// Fragment shader required by Glium
fn fragment_shader() -> &'static str {
    r#"
        #version 140

        in vec4 v_color;

        out vec4 color_final;

        void main() {
            color_final = vec4(v_color);
        }
    "#
}

fn setup_draw_params<'a>(point_size: usize) -> DrawParameters<'a> {
    // Determines pixel size
    const POINT_SIZES: [f32; 3] = [1.0, 3.0, 10.0];
    glium::DrawParameters {
        point_size: Some(POINT_SIZES[point_size]),
        ..Default::default()
    }
}

/// Draws the vertices on the canvas.
///
/// # Arguments
///
/// * `globals` - Global variables for the app
/// * `vertex_vector` - A reference to a vector of shapes, and each shape is a vector of vertices
/// * `display` - A reference to the GL context with a facade for drawing upon
/// * `target` - A reference to the current frame buffer
///
pub fn draw_vertices(
    globals: Globals,
    vertex_vector: &mut Vec<Vertex>,
    display: &Display,
    target: &mut Frame,
) {
    let draw_parameters = setup_draw_params(globals.point_size());

    // Buffer containing pixel data
    let vertex_buffer = glium::VertexBuffer::new(display, &vertex_vector).unwrap();
    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::Points);

    // Combines the different shaders into the display for OpenGL
    let program =
        glium::Program::from_source(display, vertex_shader(), fragment_shader(), None).unwrap();

    // Finally draws everything to the screen
    target
        .draw(
            &vertex_buffer,
            &index_buffer,
            &program,
            &glium::uniforms::EmptyUniforms,
            &draw_parameters,
        )
        .unwrap();
}
