use crate::config::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::models::{color::Color, coordinate::Coordinate};

/// A vertex is defined by an x,y coordinate point and an associated RGB+Opacity value
#[derive(Copy, Clone)]
pub struct Vertex {
    color: Color,
    position: Coordinate,
}

impl Vertex {
    /// Returns a new vertex with a transparent color and (0.0, 0.0) coord location
    pub fn new() -> Vertex {
        Vertex {
            color: Color::new(),
            position: Coordinate::new(),
        }
    }

    /// Returns the current coord location of a vertex
    pub fn get_position(&self) -> Coordinate {
        self.position
    }

    /// Sets the current coord location of a vertex
    ///
    /// # Arguments
    ///
    /// * `new_position` - The new coordinate object to be used as this vertex's position
    ///
    pub fn set_position(&mut self, new_position: Coordinate) {
        self.position = new_position;
    }

    // Converts the vertex's position from e.g. 1600 x 900 to -1.0-1.0 in both x and y axis.
    pub fn convert_to_gl(&mut self, new_coordinates: Coordinate) {
        let nx = (new_coordinates.x() - (WINDOW_WIDTH / 2) as f64) / (WINDOW_WIDTH / 2) as f64;
        let ny = (new_coordinates.y() - (WINDOW_HEIGHT / 2) as f64) / (WINDOW_HEIGHT / 2) as f64;
        self.set_position(Coordinate::new_with_values(nx, ny));
    }
}

implement_vertex!(Vertex, position, color);

/// Creates a vector of vertices with (0,0) coordinates and random colors
///
/// # Arguments
///
/// * `amount` - The amount of vertices to be created in this vector
///
pub fn populate_vertex_vector(amount: usize) -> Vec<Vertex> {
    let mut vertex_array = vec![Vertex::new(); amount];
    for vertex in &mut vertex_array {
        vertex.color.set_color();
    }
    vertex_array
}
