use crate::models::{globals::Globals, vertex::Vertex};
use glium::{DrawParameters, Frame, Surface};

pub mod objects;
pub mod text;
pub mod utility;

fn setup_draw_params<'a>() -> DrawParameters<'a> {
    glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::Overwrite,
            write: true,
            ..Default::default()
        },
        blend: glium::Blend {
            color: glium::BlendingFunction::ReverseSubtraction {
                source: glium::LinearBlendingFactor::One,
                destination: glium::LinearBlendingFactor::One,
            },
            alpha: glium::BlendingFunction::ReverseSubtraction {
                source: glium::LinearBlendingFactor::One,
                destination: glium::LinearBlendingFactor::One,
            },
            ..Default::default()
        },
        line_width: None,
        ..Default::default()
    }
}

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

    draw_fade_overlay(globals, &mut target);
    objects::draw_vertices(globals, &mut target, &vertex_vector);

    text::draw_equation_text(equation_text, globals, &mut target);
    text::draw_time_text(globals, &mut target);
    text::draw_speed_multiplier_text(globals, &mut target);
    text::draw_point_size_text(globals, &mut target);

    target.finish().unwrap();
}

fn draw_fade_overlay(globals: &Globals, target: &mut Frame) {
    let draw_parameters = setup_draw_params();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    implement_vertex!(Vertex, position);
    let shape = vec![
        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [1.0, -1.0] },
        Vertex { position: [1.0, 1.0] },
        Vertex { position: [1.0, 1.0] },
        Vertex { position: [-1.0, 1.0] },
        Vertex { position: [-1.0, -1.0] },
    ];

    // Buffer containing pixel data
    let vertex_buffer = glium::VertexBuffer::new(globals.display(), &shape).unwrap();
    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.1, 0.1, 0.1, 0.0);
        }
    "#;

    // Combines the different shaders into the display for OpenGL
    //let program =
    //    glium::Program::new(globals.display());
    let program =
        glium::Program::from_source(globals.display(), vertex_shader, fragment_shader, None)
            .unwrap();

    // Draws the 'shape' to the screen
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
