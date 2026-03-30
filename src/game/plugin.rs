// use super::state::GameState;
use super::config::GameConfig;
// use super::events::*;
// use super::systems::handle_pause;
use crate::camera::CameraPlugin;
use crate::game::{GameState, WorldGeneratedEvent};
use crate::generation::WorldGenerationPlugin;
use crate::rendering::HexRenderingPlugin;
use bevy::ecs::error::info;

use crate::ui::UIPlugin;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(GameConfig::default())
            .add_message::<WorldGeneratedEvent>()
            .add_plugins((
                CameraPlugin,
                WorldGenerationPlugin,
                HexRenderingPlugin,
                UIPlugin,
            ))
            .add_systems(Update, handle_pause.run_if(in_state(GameState::Playing)))
            .add_systems(Update, handle_resume.run_if(in_state(GameState::Paused)))
            .add_systems(Update, start_game.run_if(in_state(GameState::MainMenu)));
    }
}

pub fn handle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if *current_state.get() == GameState::Playing {
            next_state.set(GameState::Paused);
            info!("Game is paused!");
        }
    }
}

fn handle_resume(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if *current_state.get() == GameState::Paused {
            next_state.set(GameState::Playing);
            info!("Game is playing");
        }
    }
}

fn start_game(mut next_state: ResMut<NextState<GameState>>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Enter) {
        // или любая другая кнопка
        next_state.set(GameState::Loading);
    }
}
