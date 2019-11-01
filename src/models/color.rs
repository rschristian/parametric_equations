use glium::{vertex::AttributeType, CapabilitiesSource};
use std::cmp;

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
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            opacity: 0.0,
        }
    }

    /// Sets the color to a RGB value based on index in vertex vector
    pub fn set_color(&mut self, i: i32) {
        let i = i + 1;
        self.red = cmp::min(255, 50 + (i * 11909) % 256) as f32 / 255.0;
        self.green = cmp::min(255, 50 + (i * 52973) % 256) as f32 / 255.0;
        self.blue = cmp::min(255, 50 + (i * 44111) % 256) as f32 / 255.0;
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
