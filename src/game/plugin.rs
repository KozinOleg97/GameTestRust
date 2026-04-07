// use super::state::GameState;
use super::config::GameConfig;
// use super::events::*;
// use super::systems::handle_pause;
use crate::camera::CameraPlugin;
use crate::game::{GameState, WorldGeneratedEvent};
use crate::generation::WorldGenerationPlugin;
use crate::rendering::{FullMeshRenderingPlugin, HexRenderingPlugin, RenderingMode};

use crate::game::settings::GameSettings;
use crate::game::settings_config::get_settings_config;
use crate::ui::UIPlugin;
use bevy::prelude::*;
use bevy_settings_lib::{
    PersistSetting, ReloadSetting, SettingsPlugin, SettingsPluginConfig, SettingsStorage,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        /// Конфигурация файла сохранений
        let config = get_settings_config();

        app.init_state::<GameState>()
            .insert_resource(GameConfig::default())
            .add_message::<WorldGeneratedEvent>()
            .add_plugins((
                CameraPlugin,
                WorldGenerationPlugin,
                HexRenderingPlugin {
                    mode: RenderingMode::FullMesh, // Chunked  FullMesh
                },
                UIPlugin,
            ))
            .add_plugins(SettingsPlugin::<GameSettings>::from_config(config))
            .add_systems(Startup, reload_settings)
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

fn save_initial_settings(mut commands: Commands, settings: Res<GameSettings>) {
    commands.trigger(PersistSetting::<GameSettings> {
        value: Some(settings.clone()),
    });
}

fn reload_settings(mut commands: Commands) {
    commands.trigger(ReloadSetting::<GameSettings> {
        _phantom: std::marker::PhantomData,
    });
}
