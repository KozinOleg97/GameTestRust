pub mod debug;
pub mod hex_math;
pub mod hex_picking;
pub mod mesh_builder;
pub mod plugin;
pub mod systems;

pub use debug::{debug_draw_chunk_bounds, debug_frustum_stats, log_when_all_chunks_meshed};
pub use hex_math::{
    DEFAULT_ELEVATION_STEP, DEFAULT_HEX_SIZE, DEFAULT_SKIRT_HEIGHT, SHARED_CORNERS,
    chunk_world_translation_with, hex_corners, hex_world_position, hex_world_position_with,
    ray_ground_point,
};
pub use plugin::HexRenderingPlugin;
pub use systems::{
    MeshBuildQueue, MeshBuildTask, MeshGeneration, TerrainMaterial, apply_mesh_tasks_system,
    clear_mesh_queue, ensure_mesh_generation, process_mesh_build_queue_system,
    queue_dirty_chunks_system, setup_terrain_material,
};
