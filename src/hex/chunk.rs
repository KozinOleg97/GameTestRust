use bevy::prelude::*;

use super::HexType;

pub const DEFAULT_CHUNK_SIZE: usize = 32;
pub const MIN_CHUNK_SIZE: usize = 8;
pub const MAX_CHUNK_SIZE: usize = 128;

pub fn is_valid_chunk_size(size: usize) -> bool {
    size >= MIN_CHUNK_SIZE && size <= MAX_CHUNK_SIZE && size.is_power_of_two()
}

pub fn normalize_chunk_size(size: usize) -> usize {
    if is_valid_chunk_size(size) {
        size
    } else {
        DEFAULT_CHUNK_SIZE
    }
}

// -----------------------------------------------------------------------------
// Chunk layout
// -----------------------------------------------------------------------------

/// Текущая раскладка чанков.
///
/// Создаётся из настроек:
/// ```rust
/// ChunkLayout::from_size(32)
/// ```
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkLayout {
    pub size: usize,
    pub shift: u32,
    pub mask: i32,
    pub area: usize,
}

impl Default for ChunkLayout {
    fn default() -> Self {
        Self::from_size(DEFAULT_CHUNK_SIZE)
    }
}

impl ChunkLayout {
    pub fn from_size(size: usize) -> Self {
        let size = normalize_chunk_size(size);

        let shift = size.trailing_zeros();
        let mask = size as i32 - 1;
        let area = size * size;

        Self {
            size,
            shift,
            mask,
            area,
        }
    }

    #[inline]
    pub fn chunk_coords(&self, global_q: i32, global_r: i32) -> (i32, i32) {
        (global_q >> self.shift, global_r >> self.shift)
    }

    #[inline]
    pub fn local_coords(&self, global_q: i32, global_r: i32) -> (usize, usize) {
        (
            (global_q & self.mask) as usize,
            (global_r & self.mask) as usize,
        )
    }

    #[inline]
    pub fn index(&self, local_q: usize, local_r: usize) -> usize {
        local_r * self.size + local_q
    }

    #[inline]
    pub fn origin_q(&self, chunk_x: i32) -> i32 {
        chunk_x * self.size as i32
    }

    #[inline]
    pub fn origin_r(&self, chunk_y: i32) -> i32 {
        chunk_y * self.size as i32
    }

    #[inline]
    pub fn contains_local(&self, local_q: i32, local_r: i32) -> bool {
        local_q >= 0 && local_q < self.size as i32 && local_r >= 0 && local_r < self.size as i32
    }
}

// -----------------------------------------------------------------------------
// Map bounds
// -----------------------------------------------------------------------------

/// Границы карты.
///
/// Карта всегда начинается с (0, 0)
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MapBounds {
    pub width: i32,
    pub height: i32,
}

impl MapBounds {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width: width.max(0),
            height: height.max(0),
        }
    }

    #[inline]
    pub fn contains_hex(&self, q: i32, r: i32) -> bool {
        q >= 0 && q < self.width && r >= 0 && r < self.height
    }

    pub fn contains_chunk(&self, layout: &ChunkLayout, chunk_x: i32, chunk_y: i32) -> bool {
        if self.width <= 0 || self.height <= 0 {
            return false;
        }

        let chunk_min_q = layout.origin_q(chunk_x);
        let chunk_max_q = chunk_min_q + layout.size as i32 - 1;

        let chunk_min_r = layout.origin_r(chunk_y);
        let chunk_max_r = chunk_min_r + layout.size as i32 - 1;

        let map_max_q = self.width - 1;
        let map_max_r = self.height - 1;

        chunk_max_q >= 0 && chunk_min_q <= map_max_q && chunk_max_r >= 0 && chunk_min_r <= map_max_r
    }
}

// -----------------------------------------------------------------------------
// Hex chunk component
// -----------------------------------------------------------------------------

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[require(Transform, Visibility)]
pub struct HexChunk {
    pub chunk_x: i32,
    pub chunk_y: i32,
}

impl HexChunk {
    pub fn new(chunk_x: i32, chunk_y: i32) -> Self {
        Self { chunk_x, chunk_y }
    }

    #[inline]
    pub fn origin_q(&self, layout: &ChunkLayout) -> i32 {
        layout.origin_q(self.chunk_x)
    }

    #[inline]
    pub fn origin_r(&self, layout: &ChunkLayout) -> i32 {
        layout.origin_r(self.chunk_y)
    }
}

// -----------------------------------------------------------------------------
// Chunk data
// -----------------------------------------------------------------------------

#[derive(Component, Debug, Clone)]
pub struct ChunkData {
    pub size: usize,
    pub biomes: Vec<HexType>,
    pub elevations: Vec<i8>,
}

impl Default for ChunkData {
    fn default() -> Self {
        Self::new(DEFAULT_CHUNK_SIZE)
    }
}

impl ChunkData {
    pub fn new(size: usize) -> Self {
        let size = normalize_chunk_size(size);
        let area = size * size;

        Self {
            size,
            biomes: vec![HexType::Empty; area],
            elevations: vec![0; area],
        }
    }

    pub fn with_data(size: usize, biomes: Vec<HexType>, elevations: Vec<i8>) -> Self {
        let size = normalize_chunk_size(size);
        let area = size * size;

        if biomes.len() == area && elevations.len() == area {
            Self {
                size,
                biomes,
                elevations,
            }
        } else {
            Self::new(size)
        }
    }

    #[inline]
    pub fn area(&self) -> usize {
        self.size * self.size
    }

    #[inline]
    pub fn index(&self, local_q: usize, local_r: usize) -> usize {
        local_r * self.size + local_q
    }

    #[inline]
    pub fn set_hex(&mut self, local_q: usize, local_r: usize, biome: HexType, elevation: i8) {
        let idx = self.index(local_q, local_r);
        self.biomes[idx] = biome;
        self.elevations[idx] = elevation;
    }

    pub fn is_empty(&self) -> bool {
        self.biomes.iter().all(|b| b.is_empty())
    }
}
