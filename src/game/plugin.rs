use bevy::log;

use crate::camera::CameraPlugin;
use crate::game::{GameSessionState, PlayState, WorldGeneratedEvent};
use crate::generation::WorldGenerationPlugin;
use crate::rendering::{FullMeshRenderingPlugin, HexRenderingPlugin, RenderingMode};

use crate::ui::UIPlugin;
use bevy::prelude::*;
use bevy_settings::{SettingsPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameSessionState>()
            .add_sub_state::<PlayState>()
            .add_message::<WorldGeneratedEvent>()
            .add_plugins((
                CameraPlugin,
                WorldGenerationPlugin,
                HexRenderingPlugin {
                    mode: RenderingMode::FullMesh,
                },
                UIPlugin,
            ))
            .add_plugins(SettingsPlugin::new("org.bevy.examples.settings"))
            // Обработка паузы/возобновления
            .add_systems(
                Update,
                handle_pause.run_if(in_state(GameSessionState::Active)),
            )
            // Старт игры из главного меню
            .add_systems(
                Update,
                start_game.run_if(in_state(GameSessionState::MainMenu)),
            );
    }
}

pub fn handle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_play_state: Res<State<PlayState>>,
    mut next_play_state: ResMut<NextState<PlayState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if *current_play_state.get() == PlayState::Playing {
            next_play_state.set(PlayState::Paused);
            info!("Game is paused!");
        } else if *current_play_state.get() == PlayState::Paused {
            next_play_state.set(PlayState::Playing);
            info!("Game is playing");
        }
    }
}

fn start_game(
    mut next_session_state: ResMut<NextState<GameSessionState>>,
    mut next_play_state: ResMut<NextState<PlayState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        next_session_state.set(GameSessionState::Active);
        next_play_state.set(PlayState::Playing);
    }
}
