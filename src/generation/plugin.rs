use crate::game::{GameConfig, GameState, WorldGeneratedEvent};
use crate::generation::ProceduralWorldGenerator;
use bevy::prelude::*;

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), generate_world);
        // app.add_systems(Startup, generate_world);
    }
}

fn generate_world(
    config: Res<GameConfig>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut writer: MessageWriter<WorldGeneratedEvent>,
) {
    let generator =
        ProceduralWorldGenerator::new(config.map_width, config.map_height, config.generation_seed);
    let hex_map = generator.generate_world();
    commands.insert_resource(hex_map);

    // Отправляем сигнал о завершении
    writer.write(WorldGeneratedEvent);

    next_state.set(GameState::Playing);
}
