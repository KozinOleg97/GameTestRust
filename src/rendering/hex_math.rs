use bevy::prelude::*;

use crate::hex::{ChunkLayout, HexChunk};

pub const DEFAULT_HEX_SIZE: f32 = 1.0;
pub const DEFAULT_ELEVATION_STEP: f32 = 0.9;
pub const DEFAULT_SKIRT_HEIGHT: f32 = -10.0;

pub const SQRT_3: f32 = 1.7320508075688772;

pub fn hex_world_position(global_q: i32, global_r: i32) -> Vec3 {
    hex_world_position_with(DEFAULT_HEX_SIZE, global_q, global_r)
}

pub fn hex_world_position_with(hex_size: f32, global_q: i32, global_r: i32) -> Vec3 {
    let x = hex_size * SQRT_3 * (global_q as f32 + global_r as f32 / 2.0);
    let z = hex_size * 1.5 * global_r as f32;

    Vec3::new(x, 0.0, z)
}

pub fn chunk_world_translation_with(hex_size: f32, layout: &ChunkLayout, chunk: &HexChunk) -> Vec3 {
    hex_world_position_with(hex_size, chunk.origin_q(layout), chunk.origin_r(layout))
}

pub fn hex_corners(hex_size: f32) -> [Vec3; 6] {
    let mut corners = [Vec3::ZERO; 6];

    for i in 0..6 {
        let angle_rad = (60.0 * i as f32 + 30.0).to_radians();

        corners[i] = Vec3::new(hex_size * angle_rad.cos(), 0.0, hex_size * angle_rad.sin());
    }

    corners
}

/// Индексы углов гекса, образующих грань с соседом.
///
/// Порядок соответствует HEX_DIRECTIONS:
/// 0: East
/// 1: Southeast
/// 2: Southwest
/// 3: West
/// 4: Northwest
/// 5: Northeast
pub const SHARED_CORNERS: [[usize; 2]; 6] = [
    [5, 0], // East
    [0, 1], // Southeast
    [1, 2], // Southwest
    [2, 3], // West
    [3, 4], // Northwest
    [4, 5], // Northeast
];
