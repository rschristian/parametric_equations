extern crate rand;

use glium::{vertex::AttributeType, CapabilitiesSource};
use rand::Rng;

/// Standard RGB color value, with an opacity.
#[derive(Copy, Clone)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
    opacity: f32,
}

impl Color {
    /// Returns a blank color object that is black and transparent
    pub fn new() -> Color {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            opacity: 0.0,
        }
    }

    /// Sets the color to a random RGB value, but full opacity
    pub fn set_color(&mut self) {
        let mut rng = rand::thread_rng();
        self.red = rng.gen();
        self.green = rng.gen();
        self.blue = rng.gen();
        self.opacity = 1.0;
    }
}

unsafe impl glium::vertex::Attribute for Color {
    fn get_type() -> AttributeType {
        AttributeType::F32F32F32F32
    }

    fn is_supported<C: ?Sized>(_caps: &C) -> bool
    where
        C: CapabilitiesSource,
    {
        unimplemented!()
    }
}
