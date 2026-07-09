use crate::game::{GameSettings, GameState, WorldGeneratedEvent};
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
    config: Res<GameSettings>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut writer: MessageWriter<WorldGeneratedEvent>,
) {
    let generator = ProceduralWorldGenerator::new(
        config.generation.map_width,
        config.generation.map_height,
        config.generation.generation_seed,
    );
    let hex_map = generator.generate_world();
    commands.insert_resource(hex_map);

    // Отправляем сигнал о завершении
    writer.write(WorldGeneratedEvent);

    next_state.set(GameState::Playing);
}
