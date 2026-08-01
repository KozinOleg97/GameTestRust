use bevy::prelude::*;

use super::debug::debug_draw_chunk_bounds;
use super::systems::{
    apply_mesh_tasks_system, clear_mesh_queue, ensure_mesh_generation, process_mesh_build_queue_system,
    queue_dirty_chunks_system, setup_terrain_material, MeshBuildQueue,
    TerrainMaterial,
};
use crate::game::GameSessionState;
use crate::hex::{ChunkLayout, ChunkMap, MapBounds};
use crate::rendering::{debug_frustum_stats, log_when_all_chunks_meshed};

pub struct HexRenderingPlugin;

impl Plugin for HexRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkMap>()
            .init_resource::<ChunkLayout>()
            .init_resource::<MapBounds>()
            .init_resource::<MeshBuildQueue>()
            .init_resource::<TerrainMaterial>()
            .add_systems(OnEnter(GameSessionState::Loading), clear_mesh_queue)
            .add_systems(OnExit(GameSessionState::Active), clear_mesh_queue)
            .add_systems(
                Update,
                (
                    setup_terrain_material,
                    ensure_mesh_generation,
                    queue_dirty_chunks_system,
                    process_mesh_build_queue_system,
                )
                    .chain()
                    .run_if(in_state(GameSessionState::Active)),
            )
            .add_systems(
                PostUpdate,
                apply_mesh_tasks_system.run_if(in_state(GameSessionState::Active)),
            )
            .add_systems(
                Update,
                debug_draw_chunk_bounds.run_if(in_state(GameSessionState::Active)),
            )
            .add_systems(
                Update,
                log_when_all_chunks_meshed.run_if(in_state(GameSessionState::Active)),
            )
            .add_systems(
                Update,
                debug_frustum_stats.run_if(in_state(GameSessionState::Active)),
            );
    }
}
