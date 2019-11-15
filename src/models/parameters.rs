extern crate rand;

use rand::Rng;

pub struct Parameters {
    x: ParamDimensions,
    y: ParamDimensions,
}

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

    pub fn x_dimensions(&self) -> &ParamDimensions {
        &self.x
    }

    pub fn y_dimensions(&self) -> &ParamDimensions {
        &self.y
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

    pub fn xx(&self) -> f64 {
        self.xx
    }

    pub fn yy(&self) -> f64 {
        self.yy
    }

    pub fn tt(&self) -> f64 {
        self.tt
    }

    pub fn xy(&self) -> f64 {
        self.xy
    }

    pub fn xt(&self) -> f64 {
        self.xt
    }

    pub fn yt(&self) -> f64 {
        self.yt
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn t(&self) -> f64 {
        self.t
    }
}
