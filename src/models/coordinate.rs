use glium::vertex::AttributeType;

#[derive(Copy, Clone)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

impl Coordinate {
    /// Creates a coordinate at a point just offscreen, to ensure a default point is never shown
    pub fn new() -> Coordinate {
        Coordinate { x: 1.1, y: 1.1 }
    }

    /// Creates a new coordinate point with given x and y values
    ///
    /// # Arguments
    ///
    /// * `nx` - X value for new coordinate
    /// * `ny` - Y value for new coordinate
    ///
    pub fn new_with_values(nx: f64, ny: f64) -> Coordinate {
        Coordinate { x: nx, y: ny }
    }
}

unsafe impl glium::vertex::Attribute for Coordinate {
    const TYPE: AttributeType = AttributeType::F64F64;
    fn get_type() -> AttributeType {
        glium::vertex::AttributeType::F64F64
    }
}
