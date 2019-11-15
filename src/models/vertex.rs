use crate::config::{ITERATIONS, STEPS, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::models::coordinate::Coordinate;
use std::cmp;

/// A vertex is defined by an RGB+Opacity color value and an (x,y) coordinate point
#[derive(Copy, Clone)]
pub struct Vertex {
    color: [f32; 4],
    position: Coordinate,
}

impl Vertex {
    /// Returns a new vertex with a transparent color and coord point that is just offscreen
    /// (otherwise you'd be drawing some points to the center of the screen on the first few loops)
    pub fn new() -> Vertex {
        Vertex {
            color: [0.0, 0.0, 0.0, 0.0],
            position: Coordinate::new(),
        }
    }

    /// Sets the current current color based on index in vector
    ///
    /// # Arguments
    ///
    /// * `i` - The index of the vertex in a vector
    ///
    pub fn set_color(&mut self, i: i32) {
        let i = i + 1;
        let red = cmp::min(255, 50 + (i * 11909) % 256) as f32 / 255.0;
        let green = cmp::min(255, 50 + (i * 52973) % 256) as f32 / 255.0;
        let blue = cmp::min(255, 50 + (i * 44111) % 256) as f32 / 255.0;
        self.color = [red, green, blue, 1.0];
    }

    /// Returns the current coord location of a vertex
    pub fn position(&self) -> Coordinate {
        self.position
    }

    /// Sets the current coord location of a vertex
    ///
    /// # Arguments
    ///
    /// * `new_position` - The new coordinate object to be used as this vertex's position
    ///
    fn set_position(&mut self, new_position: Coordinate) {
        self.position = new_position;
    }

    /// Converts the vertex's position from e.g. 1600 x 900 to -1.0-1.0 in both x and y axis.
    ///
    /// # Arguments
    ///
    /// * `new_coordinates` - Coordinate struct that contains the new values for this particular vertex
    ///
    pub fn set_position_within_window_dimensions(&mut self, new_coordinates: Coordinate) {
        let nx = (new_coordinates.x() - (WINDOW_WIDTH / 2) as f64) / (WINDOW_WIDTH / 2) as f64;
        let ny = (new_coordinates.y() - (WINDOW_HEIGHT / 2) as f64) / (WINDOW_HEIGHT / 2) as f64;
        self.set_position(Coordinate::new_with_values(nx, ny));
    }
}

implement_vertex!(Vertex, position, color);

/// Creates a vector of vertices with default coordinates and random colors
pub fn create_vertex_vector() -> Vec<Vertex> {
    let mut vertex_array = vec![Vertex::new(); (ITERATIONS * STEPS) as usize];
    for (i, vertex) in vertex_array.iter_mut().enumerate() {
        vertex.set_color((i + 1) as i32 % ITERATIONS);
    }
    vertex_array
}
