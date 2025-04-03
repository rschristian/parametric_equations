use crate::constants::{TOTAL_VERTICES, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::models::coordinate::Coordinate;
use std::cmp;

/// A vertex is defined by an RGB+Opacity color value and an (x,y) coordinate point
#[derive(Copy, Clone)]
pub struct Vertex {
    pub color: [f32; 4],
    pub position: Coordinate,
}

impl Vertex {
    pub fn new_with_color(i: usize) -> Vertex {
        Vertex {
            color: generate_color_from_iterator_position(i),
            position: Coordinate::new(),
        }
    }

    fn set_position(&mut self, new_position: Coordinate) {
        self.position = new_position;
    }

    /// Converts the vertex's position from e.g. 1600 x 900 to -1.0-1.0 in both x and y axis.
    pub fn set_position_within_window_dimensions(&mut self, new_coordinates: Coordinate) {
        let nx = (new_coordinates.x - (WINDOW_WIDTH / 2) as f64) / (WINDOW_WIDTH / 2) as f64;
        let ny = (new_coordinates.y - (WINDOW_HEIGHT / 2) as f64) / (WINDOW_HEIGHT / 2) as f64;
        self.set_position(Coordinate::new_with_values(nx, ny));
    }
}

implement_vertex!(Vertex, position, color);

fn generate_color_from_iterator_position(i: usize) -> [f32; 4] {
    let i = i + 1;
    let red = cmp::min(255, 50 + (i * 11909) % 256) as f32 / 255.0;
    let green = cmp::min(255, 50 + (i * 52973) % 256) as f32 / 255.0;
    let blue = cmp::min(255, 50 + (i * 44111) % 256) as f32 / 255.0;

    [red, green, blue, 1.0]
}

pub type VertexSlice = [Vertex; TOTAL_VERTICES];

pub fn create_vertex_slice() -> VertexSlice {
    //core::array::from_fn(|i| Vertex::new_with_color(i))
    core::array::from_fn(Vertex::new_with_color)
}
