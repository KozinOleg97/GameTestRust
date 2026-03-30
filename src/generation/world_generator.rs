use bevy::log::info;
use crate::generation::PerlinNoise;
use crate::hex::{Hex, HexCoordinates, HexMap, HexType};

pub struct ProceduralWorldGenerator {
    width: i32,
    height: i32,
    seed: u64,
    // Noise generators
    continental_noise: PerlinNoise,
    terrain_noise: PerlinNoise,
    moisture_noise: PerlinNoise,
    temperature_noise: PerlinNoise,
    mountain_noise: PerlinNoise,
    aridity_noise: PerlinNoise,
}

impl ProceduralWorldGenerator {
    pub fn new(width: i32, height: i32, seed: u64) -> Self {
        Self {
            width,
            height,
            seed,
            continental_noise: PerlinNoise::new(seed as u32),
            terrain_noise: PerlinNoise::new((seed + 1) as u32),
            moisture_noise: PerlinNoise::new((seed + 2) as u32),
            temperature_noise: PerlinNoise::new((seed + 3) as u32),
            mountain_noise: PerlinNoise::new((seed + 4) as u32),
            aridity_noise: PerlinNoise::new((seed + 5) as u32),
        }
    }

    pub fn generate_world(&self) -> HexMap {
        info!("Generating world...");

        let mut hex_map = HexMap::new(self.width, self.height);

        // Generate hexes for the entire map
        for q in 0..self.width {
            for r in 0..self.height {
                let coordinates = HexCoordinates::new(q, r);
                let hex_type = self.determine_hex_type(q, r);
                let hex = Hex::new(coordinates, hex_type);
                hex_map.set_hex(q, r, hex)
            }
        }

        info!("Generated world with {} hexes", hex_map.size());

        hex_map
    }

    fn determine_hex_type(&self, q: i32, r: i32) -> HexType {
        let nx = q as f64 / self.width as f64;
        let ny = r as f64 / self.height as f64;

        // Get noise values
        let height = self.continental_noise.noise(nx * 2.0, ny * 2.0) as f32;
        let moisture = self.moisture_noise.noise(nx * 3.0, ny * 3.0) as f32;
        let temperature = self.temperature_noise.noise(nx * 2.5, ny * 2.5) as f32;
        let aridity = self.aridity_noise.noise(nx * 1.5, ny * 1.5) as f32;

        // Normalize noise values to 0-1 range
        let height = (height + 1.0) / 2.0;
        let moisture = (moisture + 1.0) / 2.0;
        let temperature = (temperature + 1.0) / 2.0;
        let aridity = (aridity + 1.0) / 2.0;

        // Determine hex type based on noise values
        if height < 0.2 {
            HexType::Ocean
        } else if height < 0.25 {
            HexType::Coast
        } else if height > 0.8 && self.mountain_noise.noise(nx * 4.0, ny * 4.0) as f32 > 0.5 {
            HexType::Mountains
        } else if moisture < 0.3 && aridity > 0.7 {
            HexType::Desert
        } else if moisture > 0.7 && height < 0.5 {
            HexType::Swamp
        } else if temperature < 0.3 && height > 0.5 {
            HexType::Mountains
        } else if moisture > 0.6 {
            HexType::Forest
        } else {
            HexType::Plains
        }
    }
}
