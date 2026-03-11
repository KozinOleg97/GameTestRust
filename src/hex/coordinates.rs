use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexCoordinates {
    q: i32,
    r: i32,
}

impl HexCoordinates {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }
    
    pub fn q(&self) -> i32 {
        self.q
    }
    
    pub fn r(&self) -> i32 {
        self.r
    }
}

impl Display for HexCoordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.q, self.r)
    }
}

// Constants for hex directions
pub const HEX_DIRECTIONS: [HexCoordinates; 6] = [
    HexCoordinates { q: 1, r: 0 },   // East
    HexCoordinates { q: 1, r: -1 },    // Northeast
    HexCoordinates { q: 0, r: -1 },   // Northwest
    HexCoordinates { q: -1, r: 0 },  // West
    HexCoordinates { q: -1, r: 1 }, // Southwest
    HexCoordinates { q: 0, r: 1 },   // Southeast
];