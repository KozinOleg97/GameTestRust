use bevy::prelude::*;

use super::hex_math::hex_world_position_with;
use crate::game::GameSettings;
use crate::hex::{ChunkLayout, HexChunk};
use crate::rendering::{MeshBuildQueue, MeshBuildTask};

pub fn debug_draw_chunk_bounds(
    mut gizmos: Gizmos,
    chunks: Query<&HexChunk>,
    layout: Res<ChunkLayout>,
    config: Res<GameSettings>,
) {
    if !config.debug.chunk_bounds {
        return;
    }

    const DEBUG_HEIGHT: f32 = 10.0;

    let size = layout.size as i32;
    let hex_size = config.rendering.hex_size;

    for chunk in &chunks {
        let origin_q = layout.origin_q(chunk.chunk_x);
        let origin_r = layout.origin_r(chunk.chunk_y);

        let p00 = hex_world_position_with(hex_size, origin_q, origin_r);
        let p10 = hex_world_position_with(hex_size, origin_q + size, origin_r);
        let p01 = hex_world_position_with(hex_size, origin_q, origin_r + size);
        let p11 = hex_world_position_with(hex_size, origin_q + size, origin_r + size);

        let up = Vec3::new(0.0, DEBUG_HEIGHT, 0.0);

        let outline = [p00 + up, p10 + up, p11 + up, p01 + up, p00 + up];

        gizmos.linestrip(outline, Color::srgb(1.0, 0.0, 0.0));

        let center =
            hex_world_position_with(hex_size, origin_q + size / 2, origin_r + size / 2) + up;

        gizmos.linestrip(
            [
                center + Vec3::new(-2.0, 0.0, 0.0),
                center + Vec3::new(2.0, 0.0, 0.0),
            ],
            Color::srgb(1.0, 1.0, 0.0),
        );

        gizmos.linestrip(
            [
                center + Vec3::new(0.0, 0.0, -2.0),
                center + Vec3::new(0.0, 0.0, 2.0),
            ],
            Color::srgb(1.0, 1.0, 0.0),
        );
    }
}

pub fn log_when_all_chunks_meshed(
    mut logged: Local<bool>,
    chunks: Query<Option<&Mesh3d>, With<HexChunk>>,
    queue: Res<MeshBuildQueue>,
    tasks: Query<&MeshBuildTask>,
) {
    if *logged {
        return;
    }

    let total = chunks.iter().count();

    if total == 0 {
        return;
    }

    let with_mesh = chunks.iter().filter(|mesh| mesh.is_some()).count();

    if with_mesh == total && queue.is_empty() && tasks.is_empty() {
        info!(
            "All {} chunks have meshes. Rendering pipeline initialized.",
            total
        );
        *logged = true;
    }
}

pub fn debug_frustum_stats(
    keyboard: Res<ButtonInput<KeyCode>>,
    chunks: Query<&ViewVisibility, (With<HexChunk>, With<Mesh3d>)>,
) {
    if !keyboard.just_pressed(KeyCode::F10) {
        return;
    }

    let total = chunks.iter().count();
    let visible = chunks.iter().filter(|v| v.get()).count();

    info!("Frustum culling: visible {} / {} chunks", visible, total);
}
