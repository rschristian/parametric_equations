use crate::constants::{ITERATIONS, TOTAL_VERTICES, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::models::coordinate::Coordinate;
use std::cmp;

/// A vertex is defined by an RGB+Opacity color value and an (x,y) coordinate point
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: Coordinate,
    pub color: [f32; 4],
}

impl Vertex {
    pub fn new_with_color(i: u32) -> Vertex {
        Vertex {
            position: Coordinate::new(),
            color: generate_color_from_iterator_position(i),
        }
    }

    /// Converts the vertex's position from e.g. 1600 x 900 to -1.0-1.0 in both x and y axis.
    pub fn set_position_within_window_dimensions(&mut self, new_coordinates: Coordinate) {
        let nx = (new_coordinates.x - (WINDOW_WIDTH / 2) as f64) / (WINDOW_WIDTH / 2) as f64;
        let ny = (new_coordinates.y - (WINDOW_HEIGHT / 2) as f64) / (WINDOW_HEIGHT / 2) as f64;
        self.position = Coordinate::new_with_values(nx, ny);
    }
}

implement_vertex!(Vertex, position, color);

fn generate_color_from_iterator_position(i: u32) -> [f32; 4] {
    let i = i + 1;
    let red = cmp::min(255, 50 + (i * 11909) % 256) as f32 / 255.0;
    let green = cmp::min(255, 50 + (i * 52973) % 256) as f32 / 255.0;
    let blue = cmp::min(255, 50 + (i * 44111) % 256) as f32 / 255.0;

    [red, green, blue, 1.0]
}

pub fn create_vertex_slice() -> Vec<Vertex> {
    (0..TOTAL_VERTICES)
        .map(|i| Vertex::new_with_color(i as u32 % ITERATIONS))
        .collect::<Vec<_>>()
}
