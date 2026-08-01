use crate::game::{
    validate_game_settings, GameSessionState, GameSettings, PlayState, WorldGeneratedEvent,
};
use crate::generation::ProceduralWorldGenerator;
use crate::hex::{ChunkData, ChunkLayout, ChunkMap, HexChunk, MapBounds};
use crate::rendering::{hex_world_position_with, MeshGeneration};
use crate::ui::widgets::{spawn_menu_root, spawn_title};

use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkMap>()
            .init_resource::<ChunkLayout>()
            .init_resource::<MapBounds>()
            .add_systems(
                OnEnter(GameSessionState::Loading),
                (validate_game_settings, setup_loading_screen).chain(),
            )
            .add_systems(
                Update,
                poll_generation_task.run_if(in_state(GameSessionState::Loading)),
            )
            .add_systems(OnExit(GameSessionState::Active), cleanup_world_resources);
    }
}

#[derive(Component)]
struct GenerationTask {
    task: Task<Vec<(HexChunk, ChunkData)>>,
    seed: u64,
    layout: ChunkLayout,
    bounds: MapBounds,
    hex_size: f32,
}

fn setup_loading_screen(mut commands: Commands, config: Res<GameSettings>) {
    if config.debug.log_generation {
        info!("Entering Loading state...");
    }

    commands.spawn((
        Camera2d,
        Camera {
            order: -1,
            ..default()
        },
        DespawnOnExit(GameSessionState::Loading),
    ));

    spawn_menu_root(&mut commands, GameSessionState::Loading, |parent| {
        spawn_title(parent, "World generation...");
    });

    let generation = config.generation.clone();
    let rendering = config.rendering.clone();

    let layout = ChunkLayout::from_size(generation.chunk_size_usize());
    let bounds = MapBounds::new(generation.map_width, generation.map_height);

    let map_width = generation.map_width;
    let map_height = generation.map_height;
    let seed = generation.generation_seed;
    let chunk_size = generation.chunk_size_usize();

    let thread_pool = AsyncComputeTaskPool::get();

    let task = thread_pool.spawn(async move {
        let generator = ProceduralWorldGenerator::new(seed);

        // Ожидаемая сигнатура генератора:
        // generate_initial_chunks(map_width, map_height, chunk_size)
        generator.generate_initial_chunks(map_width, map_height, chunk_size)
    });

    commands.spawn((
        GenerationTask {
            task,
            seed,
            layout,
            bounds,
            hex_size: rendering.hex_size,
        },
        DespawnOnExit(GameSessionState::Loading),
    ));
}

#[allow(clippy::too_many_arguments)]
fn poll_generation_task(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut GenerationTask)>,
    mut chunk_map: ResMut<ChunkMap>,
    mut layout_resource: ResMut<ChunkLayout>,
    mut bounds_resource: ResMut<MapBounds>,
    mut next_session_state: ResMut<NextState<GameSessionState>>,
    mut next_play_state: ResMut<NextState<PlayState>>,
    mut writer: MessageWriter<WorldGeneratedEvent>,
) {
    for (entity, mut task) in &mut tasks {
        if let Some(generated_chunks) = future::block_on(future::poll_once(&mut task.task)) {
            let layout = task.layout;
            let bounds = task.bounds;
            let seed = task.seed;
            let hex_size = task.hex_size;

            *layout_resource = layout;
            *bounds_resource = bounds;

            chunk_map.clear();

            for (chunk, data) in generated_chunks {
                let origin = hex_world_position_with(
                    hex_size,
                    chunk.origin_q(&layout),
                    chunk.origin_r(&layout),
                );

                let chunk_entity = commands
                    .spawn((
                        chunk,
                        data,
                        MeshGeneration::default(),
                        Transform::from_translation(origin),
                        Visibility::default(),
                        DespawnOnExit(GameSessionState::Active),
                    ))
                    .id();

                chunk_map.register_chunk(chunk.chunk_x, chunk.chunk_y, chunk_entity);
            }

            commands.insert_resource(ProceduralWorldGenerator::new(seed));

            writer.write(WorldGeneratedEvent);

            next_session_state.set(GameSessionState::Active);
            next_play_state.set(PlayState::Playing);

            commands.entity(entity).despawn();

            info!(
                "World generation completed: {}x{} hexes, chunk size {}",
                bounds.width, bounds.height, layout.size
            );
        }
    }
}

fn cleanup_world_resources(
    mut commands: Commands,
    mut chunk_map: ResMut<ChunkMap>,
    mut layout: ResMut<ChunkLayout>,
    mut bounds: ResMut<MapBounds>,
) {
    chunk_map.clear();

    *layout = ChunkLayout::default();
    *bounds = MapBounds::default();

    commands.remove_resource::<ProceduralWorldGenerator>();

    info!("World resources cleaned up");
}
