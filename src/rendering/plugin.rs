use bevy::prelude::*;

use super::debug::debug_draw_chunk_bounds;
use super::hex_picking::{draw_hex_selection, select_hex_on_click, update_hovered_hex};
use super::systems::{
    MeshBuildQueue, TerrainMaterial, apply_mesh_tasks_system, clear_mesh_queue,
    ensure_mesh_generation, process_mesh_build_queue_system, queue_dirty_chunks_system,
    setup_terrain_material,
};
use crate::game::{GameSessionState, PlayState};
use crate::hex::{ChunkLayout, ChunkMap, HoveredHex, MapBounds, SelectedHex};
use crate::pointer::update_pointer_blocked;
use crate::rendering::{debug_frustum_stats, log_when_all_chunks_meshed};

pub struct HexRenderingPlugin;

impl Plugin for HexRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkMap>()
            .init_resource::<ChunkLayout>()
            .init_resource::<MapBounds>()
            .init_resource::<MeshBuildQueue>()
            .init_resource::<TerrainMaterial>()
            // Состояние выбора гекса принадлежит домену, но инициализируется здесь:
            // это единственный плагин, который знает про камеру и курсор.
            .init_resource::<HoveredHex>()
            .init_resource::<SelectedHex>()
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
            // Наведение и выбор работают только когда идёт игра: в меню и на паузе
            // клик по интерфейсу не должен выбирать гекс за панелью.
            .add_systems(
                Update,
                (update_hovered_hex, select_hex_on_click)
                    .chain()
                    .after(update_pointer_blocked)
                    .run_if(in_state(GameSessionState::Active))
                    .run_if(in_state(PlayState::Playing)),
            )
            // Контуры видны всегда в игре, включая паузу: выделение не должно
            // исчезать от того, что игрок открыл меню.
            .add_systems(
                Update,
                draw_hex_selection.run_if(in_state(GameSessionState::Active)),
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
