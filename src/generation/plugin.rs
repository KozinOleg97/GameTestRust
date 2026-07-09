use crate::game::{GameSessionState, GameSettings, PlayState, WorldGeneratedEvent};
use crate::generation::ProceduralWorldGenerator;

use crate::hex::HexMap;
use crate::ui::widgets::{spawn_menu_root, spawn_title};
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app
            // 1. Спавн экрана загрузки и запуск фоновой задачи
            .add_systems(OnEnter(GameSessionState::Loading), setup_loading_screen)
            // 2. Каждый кадр проверяем, готова ли карта
            .add_systems(
                Update,
                poll_generation_task.run_if(in_state(GameSessionState::Loading)),
            );
    }
}

/// Компонент-обертка для фоновой задачи генерации
#[derive(Component)]
struct GenerationTask(Task<HexMap>);

// -----------------------------------------------------------------------------
// 1. Спавн экрана загрузки и запуск фоновой задачи
// -----------------------------------------------------------------------------
fn setup_loading_screen(mut commands: Commands, config: Res<GameSettings>) {
    info!("Entering Loading state...");

    // Камера для отрисовки UI загрузки
    commands.spawn((Camera2d, DespawnOnExit(GameSessionState::Loading)));

    // UI экрана загрузки
    spawn_menu_root(&mut commands, GameSessionState::Loading, |parent| {
        spawn_title(parent, "World generation...");
    });

    // генерация в фоновом потоке
    let thread_pool = AsyncComputeTaskPool::get();
    let width = config.generation.map_width;
    let height = config.generation.map_height;
    let seed = config.generation.generation_seed;

    let task = thread_pool.spawn(async move {
        let generator = ProceduralWorldGenerator::new(width, height, seed);
        generator.generate_world() // Возвращает HexMap
    });

    // Сохраняем задачу в ECS
    commands.spawn((
        GenerationTask(task),
        DespawnOnExit(GameSessionState::Loading),
    ));
}

// -----------------------------------------------------------------------------
// 2. Проверка завершения задачи
// -----------------------------------------------------------------------------
fn poll_generation_task(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut GenerationTask)>,
    mut next_session_state: ResMut<NextState<GameSessionState>>,
    mut next_play_state: ResMut<NextState<PlayState>>,
    mut writer: MessageWriter<WorldGeneratedEvent>,
) {
    for (entity, mut task) in &mut tasks {
        if task.0.is_finished() {
            // Забираем результат
            let hex_map = future::block_on(&mut task.0);

            // Вставляем ресурс и отправляем евент для камеры/рендеринга
            commands.insert_resource(hex_map);
            writer.write(WorldGeneratedEvent);

            // Переходим в активную сессию и начинаем
            next_session_state.set(GameSessionState::Active);
            next_play_state.set(PlayState::Playing);

            commands.entity(entity).despawn();
            info!("World generation completed!");
        }
    }
}
