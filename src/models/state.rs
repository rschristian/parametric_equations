use crate::constants::T_START;

pub struct State {
    pub t: f64,
    pub speed_multiplier: f64,
    pub scale_factor: f64,
    pub point_size: usize,
}

impl State {
    pub fn new() -> State {
        State {
            t: T_START,
            speed_multiplier: 1.0,
            scale_factor: 0.25,
            point_size: 0,
        }
    }

    pub fn increase_t(&mut self, additional_value: f64) {
        self.t += additional_value;
    }

    pub fn reset_t(&mut self) {
        self.t = T_START;
    }

    pub fn increase_speed_multiplier(&mut self, additional_value: f64) {
        self.speed_multiplier += additional_value;
    }

    pub fn increase_point_size(&mut self) {
        self.point_size = (self.point_size + 1) % 3;
    }

    pub fn decrease_point_size(&mut self) {
        self.point_size = (self.point_size - 1) % 3;
    }
}
