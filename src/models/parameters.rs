extern crate rand;

use rand::Rng;

#[derive(Copy, Clone)]
pub struct Parameters {
    x: ParamDimensions,
    y: ParamDimensions,
}

#[derive(Copy, Clone)]
pub struct ParamDimensions {
    xx: f64,
    yy: f64,
    tt: f64,
    xy: f64,
    xt: f64,
    yt: f64,
    x: f64,
    y: f64,
    t: f64,
}

impl Parameters {
    pub fn new() -> Parameters {
        Parameters {
            x: ParamDimensions::new(),
            y: ParamDimensions::new(),
        }
    }

    pub fn get_x_dimensions(&self) -> ParamDimensions {
        self.x
    }

    pub fn get_y_dimensions(&self) -> ParamDimensions {
        self.y
    }

    pub fn reset_dimensions(&mut self) {
        self.x = ParamDimensions::new();
        self.y = ParamDimensions::new();
    }
}

impl ParamDimensions {
    pub fn new() -> ParamDimensions {
        let mut rng = rand::thread_rng();
        ParamDimensions {
            xx: rng.gen_range(-1, 2) as f64,
            yy: rng.gen_range(-1, 2) as f64,
            tt: rng.gen_range(-1, 2) as f64,
            xy: rng.gen_range(-1, 2) as f64,
            xt: rng.gen_range(-1, 2) as f64,
            yt: rng.gen_range(-1, 2) as f64,
            x: rng.gen_range(-1, 2) as f64,
            y: rng.gen_range(-1, 2) as f64,
            t: rng.gen_range(-1, 2) as f64,
        }
    }

    pub fn get_xx(&self) -> f64 {
        self.xx
    }

    pub fn get_yy(&self) -> f64 {
        self.yy
    }

    pub fn get_tt(&self) -> f64 {
        self.tt
    }

    pub fn get_xy(&self) -> f64 {
        self.xy
    }

    pub fn get_xt(&self) -> f64 {
        self.xt
    }

    pub fn get_yt(&self) -> f64 {
        self.yt
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_t(&self) -> f64 {
        self.t
    }
}
