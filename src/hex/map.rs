use crate::hex::Hex;
use crate::hex::coordinates::HexCoordinates;
use std::collections::HashMap;
use bevy::prelude::*;

#[derive(Resource)]
pub struct HexMap {
    hex_grid: HashMap<HexCoordinates, Hex>,
    width: i32,
    height: i32,
}

impl HexMap {
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Width and height must be positive");
        }
        
        Self {
            hex_grid: HashMap::new(),
            width,
            height,
        }
    }
    
    pub fn add_hex(&mut self, hex: Hex) {
        let coordinates = *hex.coordinates();
        let q = coordinates.q();
        let r = coordinates.r();
        
        if !self.is_within_bounds(q, r) {
            panic!("Hex coordinates out of bounds: ({}, {})", q, r);
        }
        
        self.hex_grid.insert(coordinates, hex);
    }
    
    pub fn get_hex(&self, coordinates: &HexCoordinates) -> Option<&Hex> {
        self.hex_grid.get(coordinates)
    }
    
    pub fn get_hex_mut(&mut self, coordinates: &HexCoordinates) -> Option<&mut Hex> {
        self.hex_grid.get_mut(coordinates)
    }
    
    pub fn get_hex_by_coords(&self, q: i32, r: i32) -> Option<&Hex> {
        self.hex_grid.get(&HexCoordinates::new(q, r))
    }
    
    pub fn has_hex(&self, coordinates: &HexCoordinates) -> bool {
        self.hex_grid.contains_key(coordinates)
    }
    
    pub fn remove_hex(&mut self, coordinates: &HexCoordinates) {
        self.hex_grid.remove(coordinates);
    }
    
    pub fn get_all_hexes(&self) -> Vec<&Hex> {
        self.hex_grid.values().collect()
    }
    
    pub fn size(&self) -> usize {
        self.hex_grid.len()
    }
    
    pub fn clear(&mut self) {
        self.hex_grid.clear();
    }
    
    pub fn width(&self) -> i32 {
        self.width
    }
    
    pub fn height(&self) -> i32 {
        self.height
    }
    
    fn is_within_bounds(&self, q: i32, r: i32) -> bool {
        q >= 0 && q < self.width && r >= 0 && r < self.height
    }
}