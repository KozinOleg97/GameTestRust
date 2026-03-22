use crate::hex::coordinates::{HexCoordinates, HEX_DIRECTIONS};
use crate::hex::Hex;

pub const HEX_SIZE: f32 = 25.0;
pub const SQRT_3: f32 = 1.73205080757; // sqrt(3)
pub const HEX_WIDTH: f32 = SQRT_3 * HEX_SIZE;
pub const HALF_WIDTH: f32 = HEX_WIDTH / 2.0;
pub const HEX_HEIGHT: f32 = 2.0 * HEX_SIZE;
pub const Y_PITCH: f32 = 1.5 * HEX_SIZE;

/// Calculates distance between two hexes
pub fn distance(a: &Hex, b: &Hex) -> i32 {
    distance_coords(a.coordinates(), b.coordinates())
}

/// Calculates distance between two coordinates
pub fn distance_coords(a: &HexCoordinates, b: &HexCoordinates) -> i32 {
    let x1 = a.q();
    let z1 = a.r();
    let x2 = b.q();
    let z2 = b.r();

    let y1 = -(x1 + z1);
    let y2 = -(x2 + z2);

    ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()) / 2
}

/// Checks if two hexes are neighbors
pub fn are_neighbors(a: &Hex, b: &Hex) -> bool {
    distance(a, b) == 1
}

/// Gets neighbor coordinates in a given direction
pub fn get_neighbor_coordinates(coordinates: &HexCoordinates, direction: usize) -> HexCoordinates {
    if direction >= 6 {
        panic!("Direction must be between 0 and 5");
    }

    let dir = HEX_DIRECTIONS[direction];
    HexCoordinates::new(coordinates.q() + dir.q(), coordinates.r() + dir.r())
}

/// Gets all neighbor coordinates for given coordinates
pub fn get_all_neighbor_coordinates(coordinates: &HexCoordinates) -> [HexCoordinates; 6] {
    let mut neighbors = [HexCoordinates::new(0, 0); 6];
    for i in 0..6 {
        neighbors[i] = get_neighbor_coordinates(coordinates, i);
    }
    neighbors
}

/// Converts axial coordinates to pixel coordinates
pub fn axial_to_pixel(coordinates: &HexCoordinates, hex_size: f32) -> (f32, f32) {
    let x = hex_size * (SQRT_3 * (coordinates.q() as f32 + 0.5 * ((coordinates.r() & 1) as f32)));
    let y = hex_size * 1.5 * coordinates.r() as f32;
    (x, y)
}

/// Gets all six neighbor coordinates
pub fn get_neighbor_coordinates_simple(coordinates: &HexCoordinates) -> [HexCoordinates; 6] {
    let q = coordinates.q();
    let r = coordinates.r();

    [
        HexCoordinates::new(q + 1, r),      // East
        HexCoordinates::new(q + 1, r - 1),  // Northeast
        HexCoordinates::new(q, r - 1),      // Northwest
        HexCoordinates::new(q - 1, r),      // West
        HexCoordinates::new(q - 1, r + 1),  // Southwest
        HexCoordinates::new(q, r + 1),       // Southeast
    ]
}