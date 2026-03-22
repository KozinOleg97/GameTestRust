use crate::hex::coordinates::HexCoordinates;
use crate::hex::HexType;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Hex {
    coordinates: HexCoordinates,
    hex_type: HexType,
    danger_level: i32,
    location_id: Option<Uuid>,
    generation_seed: u64,
}

impl Hex {
    pub fn new(coordinates: HexCoordinates, hex_type: HexType) -> Self {
        Self {
            coordinates,
            hex_type,
            danger_level: 0,
            location_id: None,
            generation_seed: 0,
        }
    }

    // Getters
    pub fn coordinates(&self) -> &HexCoordinates {
        &self.coordinates
    }

    pub fn hex_type(&self) -> &HexType {
        &self.hex_type
    }

    pub fn danger_level(&self) -> i32 {
        self.danger_level
    }

    pub fn location_id(&self) -> Option<&Uuid> {
        self.location_id.as_ref()
    }

    pub fn generation_seed(&self) -> u64 {
        self.generation_seed
    }

    // Setters
    pub fn set_hex_type(&mut self, hex_type: HexType) {
        self.hex_type = hex_type;
    }

    pub fn set_danger_level(&mut self, danger_level: i32) {
        self.danger_level = danger_level;
    }

    pub fn set_location_id(&mut self, location_id: Option<Uuid>) {
        self.location_id = location_id;
    }

    pub fn set_generation_seed(&mut self, generation_seed: u64) {
        self.generation_seed = generation_seed;
    }

    // Convenience methods
    pub fn q(&self) -> i32 {
        self.coordinates.q()
    }

    pub fn r(&self) -> i32 {
        self.coordinates.r()
    }
}