#![allow(dead_code)]

use glium::vertex::AttributeType;
use glium::CapabilitiesSource;

#[derive(Copy, Clone)]
pub struct Coordinate {
    x: f64,
    y: f64,
}

impl Coordinate {
    pub fn new() -> Coordinate {
        Coordinate { x: 0.0, y: 0.0 }
    }

    pub fn new_with_values(nx: f64, ny: f64) -> Coordinate {
        Coordinate { x: nx, y: ny }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn xy(&self) -> [f64; 2] {
        [self.x, self.y]
    }

    pub fn set_x(&mut self, nx: f64) {
        self.x = nx;
    }

    pub fn set_y(&mut self, ny: f64) {
        self.y = ny;
    }

    pub fn set_xy(&mut self, nxy: [f64; 2]) {
        self.x = nxy[0];
        self.y = nxy[1];
    }
}

unsafe impl glium::vertex::Attribute for Coordinate {
    fn get_type() -> AttributeType {
        glium::vertex::AttributeType::F64F64
    }

    fn is_supported<C: ?Sized>(_caps: &C) -> bool
    where
        C: CapabilitiesSource,
    {
        unimplemented!()
    }
}
