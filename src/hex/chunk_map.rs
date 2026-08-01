use bevy::platform::collections::HashMap;
use bevy::prelude::*;

use crate::hex::{ChunkData, ChunkLayout, HexCoordinates, HexType};

/// Главный ресурс-менеджер чанков.
#[derive(Resource, Debug, Default)]
pub struct ChunkMap {
    chunks: HashMap<(i32, i32), Entity>,
}

impl ChunkMap {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn get_chunk_entity(&self, chunk_x: i32, chunk_y: i32) -> Option<Entity> {
        self.chunks.get(&(chunk_x, chunk_y)).copied()
    }

    pub fn register_chunk(&mut self, chunk_x: i32, chunk_y: i32, entity: Entity) {
        self.chunks.insert((chunk_x, chunk_y), entity);
    }

    pub fn unregister_chunk(&mut self, chunk_x: i32, chunk_y: i32) {
        self.chunks.remove(&(chunk_x, chunk_y));
    }

    #[inline]
    pub fn get_chunk_entity_by_hex(
        &self,
        layout: &ChunkLayout,
        global_q: i32,
        global_r: i32,
    ) -> Option<Entity> {
        let (chunk_x, chunk_y) = layout.chunk_coords(global_q, global_r);
        self.get_chunk_entity(chunk_x, chunk_y)
    }

    pub fn loaded_chunk_count(&self) -> usize {
        self.chunks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chunks.is_empty()
    }

    pub fn clear(&mut self) {
        self.chunks.clear();
    }

    pub fn iter_chunks(&self) -> impl Iterator<Item = (&(i32, i32), &Entity)> {
        self.chunks.iter()
    }
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct HexInfo {
    pub coords: HexCoordinates,
    pub biome: HexType,
    pub elevation: i8,
    pub chunk_entity: Entity,
}

pub fn get_hex_info(
    chunk_map: &ChunkMap,
    layout: &ChunkLayout,
    chunks_query: &Query<&ChunkData>,
    global_q: i32,
    global_r: i32,
) -> Option<HexInfo> {
    let chunk_entity = chunk_map.get_chunk_entity_by_hex(layout, global_q, global_r)?;
    let chunk_data = chunks_query.get(chunk_entity).ok()?;

    let (local_q, local_r) = layout.local_coords(global_q, global_r);
    let idx = chunk_data.index(local_q, local_r);

    Some(HexInfo {
        coords: HexCoordinates::new(global_q, global_r),
        biome: chunk_data.biomes[idx],
        elevation: chunk_data.elevations[idx],
        chunk_entity,
    })
}

pub fn modify_hex_data<F>(
    chunk_map: &ChunkMap,
    layout: &ChunkLayout,
    chunks_query: &mut Query<&mut ChunkData>,
    global_q: i32,
    global_r: i32,
    modifier: F,
) -> bool
where
    F: FnOnce(&mut ChunkData, usize),
{
    let Some(chunk_entity) = chunk_map.get_chunk_entity_by_hex(layout, global_q, global_r) else {
        return false;
    };

    let Ok(mut chunk_data) = chunks_query.get_mut(chunk_entity) else {
        return false;
    };

    let (local_q, local_r) = layout.local_coords(global_q, global_r);
    let idx = chunk_data.index(local_q, local_r);

    modifier(&mut *chunk_data, idx);

    true
}

pub fn set_hex_data(
    chunk_map: &ChunkMap,
    layout: &ChunkLayout,
    chunks_query: &mut Query<&mut ChunkData>,
    global_q: i32,
    global_r: i32,
    biome: HexType,
    elevation: i8,
) -> bool {
    modify_hex_data(
        chunk_map,
        layout,
        chunks_query,
        global_q,
        global_r,
        |data, idx| {
            data.biomes[idx] = biome;
            data.elevations[idx] = elevation;
        },
    )
}
