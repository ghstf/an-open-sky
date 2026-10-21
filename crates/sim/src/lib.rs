use noise::{NoiseFn, OpenSimplex};

pub struct WorldGenerator {
    scale: f64,
    simplex: OpenSimplex,
}

impl WorldGenerator {
    pub fn new(scale: f64, seed: u32) -> Self {
        let simplex = OpenSimplex::new(seed);

        WorldGenerator { scale, simplex }
    }

    pub fn elevation(&self, x: f64, y: f64) -> f64 {
        self.simplex.get([x / self.scale, y / self.scale])
    }
}
