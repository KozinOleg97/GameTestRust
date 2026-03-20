use noise::{NoiseFn, Perlin};

pub struct PerlinNoise {
    perlin: Perlin,
}

impl PerlinNoise {
    pub fn new(seed: u32) -> Self {
        return Self {
            perlin: Perlin::new(seed),
        };
    }

    pub fn noise(&self, x: f64, y: f64) -> f64 {
        self.perlin.get([x, y])
    }
}
