use noise::{Perlin, NoiseFn};

pub struct PerlinNoise {
    perlin: Perlin,
}

impl PerlinNoise {
    pub fn new(seed: u32) -> Self {
        Self {
            perlin: Perlin::new(seed),
        }
    }
    
    pub fn noise(&self, x: f64, y: f64) -> f64 {
        self.perlin.get([x, y])
    }
}