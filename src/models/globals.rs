use crate::config::T_START;
use glium::Display;
use glium_text::{FontTexture, TextSystem};

pub struct Globals {
    t: f64,
    speed_multiplier: f64,
    scale_factor: f64,
    point_size: usize,
    text_system_font_texture: (TextSystem, FontTexture),
    display: Display,
}

impl Globals {
    pub fn new(display: Display) -> Globals {
        Globals {
            t: T_START,
            speed_multiplier: 1.0,
            scale_factor: 0.25,
            point_size: 0,
            text_system_font_texture: (
                glium_text::TextSystem::new(&display),
                glium_text::FontTexture::new(
                    &display,
                    &include_bytes!("../font/sf-regular.otf")[..],
                    70,
                )
                .unwrap(),
            ),
            display,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn increase_t(&mut self, additional_value: f64) {
        self.t += additional_value;
    }

    pub fn reset_t(&mut self) {
        self.t = T_START;
    }

    pub fn speed_multiplier(&self) -> f64 {
        self.speed_multiplier
    }

    pub fn increase_speed_multiplier(&mut self, additional_value: f64) {
        self.speed_multiplier += additional_value;
    }

    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    pub fn point_size(&self) -> usize {
        self.point_size
    }

    pub fn increase_point_size(&mut self) {
        self.point_size = (self.point_size + 1) % 3;
    }

    pub fn text_system_font_texture(&self) -> &(TextSystem, FontTexture) {
        &self.text_system_font_texture
    }

    pub fn display(&self) -> &Display {
        &self.display
    }
}
