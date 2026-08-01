use bevy::log;

use crate::camera::CameraPlugin;
use crate::game::{GameSessionState, PlayState, WorldGeneratedEvent};
use crate::generation::WorldGenerationPlugin;
use crate::rendering::{HexRenderingPlugin};

use crate::ui::UIPlugin;
use bevy::prelude::*;
use bevy_settings::{SettingsPlugin};
use crate::hex::ChunkMap;

/// Главный плагин игры.
/// Связывает все подсистемы: генерацию, рендеринг, камеру и UI.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // --- Состояния и события ---
            .init_state::<GameSessionState>()
            .add_sub_state::<PlayState>()
            .add_message::<WorldGeneratedEvent>()

            // --- Ресурсы ---
            // ChunkMap инициализируется пустым для защиты от гонок
            .init_resource::<ChunkMap>()

            // --- Подключаемые плагины ---
            .add_plugins((
                CameraPlugin,
                WorldGenerationPlugin,
                HexRenderingPlugin,
                UIPlugin,
                SettingsPlugin::new("org.bevy.examples.settings"),
            ))

            // --- Глобальные игровые системы ---
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

/// Обработка паузы (Escape)
pub fn handle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_play_state: Res<State<PlayState>>,
    mut next_play_state: ResMut<NextState<PlayState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_play_state.get() {
            PlayState::Playing => {
                next_play_state.set(PlayState::Paused);
                info!("Game is paused!");
            }
            PlayState::Paused => {
                next_play_state.set(PlayState::Playing);
                info!("Game is playing");
            }
            _ => {}
        }
    }
}

/// Старт игры из главного меню (Enter)
fn start_game(
    mut next_session_state: ResMut<NextState<GameSessionState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        // Переходим в состояние загрузки, где WorldGenerationPlugin
        // начнёт генерацию мира в фоновом потоке
        next_session_state.set(GameSessionState::Loading);
        info!("Starting game...");
    }
}
