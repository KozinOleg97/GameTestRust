use bevy::prelude::*;

use crate::hex::{ChunkLayout, HexChunk};

pub const DEFAULT_ELEVATION_STEP: f32 = 0.9;
pub const DEFAULT_SKIRT_HEIGHT: f32 = -10.0;

/// Размер гекса по умолчанию и корень из трёх объявлены в домене (`hex`):
/// они нужны не только отрисовке, но и `HexCoordinates::from_world_xz`.
pub use crate::hex::{DEFAULT_HEX_SIZE, SQRT_3};

pub fn hex_world_position(global_q: i32, global_r: i32) -> Vec3 {
    hex_world_position_with(DEFAULT_HEX_SIZE, global_q, global_r)
}

/// Собирает `Vec3` из доменной пары `(x, z)`: `hex` не знает про `Vec3`.
pub fn hex_world_position_with(hex_size: f32, global_q: i32, global_r: i32) -> Vec3 {
    let (x, z) = crate::hex::hex_world_xz_with(hex_size, global_q, global_r);

    Vec3::new(x, 0.0, z)
}

pub fn chunk_world_translation_with(hex_size: f32, layout: &ChunkLayout, chunk: &HexChunk) -> Vec3 {
    hex_world_position_with(hex_size, chunk.origin_q(layout), chunk.origin_r(layout))
}

/// Пересечение луча с горизонтальной плоскостью `y = 0`.
///
/// Плоскость — сознательное упрощение: рельеф поднимается до сотни единиц при
/// гексе меньше двух, и на склонах точка попадания уезжает к соседу. Заменой
/// будет марш луча по высотам из `ChunkData.elevations`.
pub fn ray_ground_point(origin: Vec3, direction: Vec3) -> Option<Vec3> {
    if direction.y.abs() < f32::EPSILON {
        return None;
    }

    let distance = -origin.y / direction.y;

    if distance <= 0.0 {
        return None;
    }

    Some(origin + direction * distance)
}

pub fn hex_corners(hex_size: f32) -> [Vec3; 6] {
    let mut corners = [Vec3::ZERO; 6];

    for (i, corner) in corners.iter_mut().enumerate() {
        let angle_rad = (60.0 * i as f32 + 30.0).to_radians();

        *corner = Vec3::new(hex_size * angle_rad.cos(), 0.0, hex_size * angle_rad.sin());
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
