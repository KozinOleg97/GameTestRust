use crate::game::{GameSessionState, GameSettings, PlayState, WorldGeneratedEvent};
use crate::generation::ProceduralWorldGenerator;
use bevy::prelude::*;

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        // Запускаем генерацию, когда глобальная сессия переходит в состояние загрузки
        app.add_systems(OnEnter(GameSessionState::Loading), generate_world);
    }
}

fn generate_world(
    config: Res<GameSettings>,
    mut commands: Commands,
    mut next_session_state: ResMut<NextState<GameSessionState>>,
    mut next_play_state: ResMut<NextState<PlayState>>,
    mut writer: MessageWriter<WorldGeneratedEvent>,
) {
    info!("Generating world...");

    let generator = ProceduralWorldGenerator::new(
        config.generation.map_width,
        config.generation.map_height,
        config.generation.generation_seed,
    );

    let hex_map = generator.generate_world();
    commands.insert_resource(hex_map);

    // Отправляем сигнал о завершении
    writer.write(WorldGeneratedEvent);

    // Переходим в активную игровую сессию и сразу начинаем играть
    next_session_state.set(GameSessionState::Active);
    next_play_state.set(PlayState::Playing);
}
