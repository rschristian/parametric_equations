use glium::{Display, DrawParameters, Frame, Surface};

use crate::models::state::State;
use crate::models::vertex::Vertex;

fn vertex_shader() -> &'static str {
    r#"
        #version 140

        in vec2 position;
        in vec4 color;

        out vec4 v_color;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            v_color = color;
        }
    "#
}

fn fragment_shader() -> &'static str {
    r#"
        #version 140

        in vec4 v_color;

        out vec4 color;

        void main() {
            color = v_color;
        }
    "#
}

fn setup_draw_params<'a>(point_size: usize) -> DrawParameters<'a> {
    const POINT_SIZES: [f32; 3] = [2.0, 3.0, 5.0];

    glium::DrawParameters {
        point_size: Some(POINT_SIZES[point_size]),
        multisampling: true,
        ..Default::default()
    }
}

pub fn draw_vertices(
    display: &Display,
    target: &mut Frame,
    state: &State,
    vertex_vector: &[Vertex],
) {
    let vertex_buffer = glium::VertexBuffer::new(display, vertex_vector).unwrap();
    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::Points);

    let program =
        glium::Program::from_source(display, vertex_shader(), fragment_shader(), None).unwrap();

    let draw_parameters = setup_draw_params(state.point_size);

    target
        .draw(
            &vertex_buffer,
            index_buffer,
            &program,
            &glium::uniforms::EmptyUniforms,
            &draw_parameters,
        )
        .unwrap();
}

pub fn draw_fade_overlay(display: &Display, target: &mut Frame) {
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    implement_vertex!(Vertex, position);

    let vertex_vector = [
        Vertex {
            position: [-1.0, -1.0],
        },
        Vertex {
            position: [1.0, -1.0],
        },
        Vertex {
            position: [1.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0],
        },
        Vertex {
            position: [-1.0, 1.0],
        },
        Vertex {
            position: [-1.0, -1.0],
        },
    ];

    let vertex_buffer = glium::VertexBuffer::new(display, &vertex_vector).unwrap();
    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    fn vertex_shader() -> &'static str {
        r#"
            #version 140

            in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#
    }

    fn fragment_shader() -> &'static str {
        r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(0.51, 0.51, 0.51, 1.0);
            }
        "#
    }

    let program =
        glium::Program::from_source(display, vertex_shader(), fragment_shader(), None).unwrap();

    let draw_parameters = glium::DrawParameters {
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
        ..Default::default()
    };

    target
        .draw(
            &vertex_buffer,
            index_buffer,
            &program,
            &glium::uniforms::EmptyUniforms,
            &draw_parameters,
        )
        .unwrap();
}
