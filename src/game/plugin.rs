use crate::camera::CameraPlugin;
use crate::game::{
    GameSessionState, MenuRequest, PlayState, WorldGeneratedEvent, handle_menu_requests,
};
use crate::generation::WorldGenerationPlugin;
use crate::rendering::HexRenderingPlugin;

use crate::hex::ChunkMap;
use crate::simulation::clock::SimClock;
use crate::simulation::plugin::SimulationPlugin;
use crate::ui::UIPlugin;
use bevy::prelude::*;
use bevy_settings::SettingsPlugin;

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
            .add_message::<MenuRequest>()
            // --- Ресурсы ---
            // ChunkMap инициализируется пустым для защиты от гонок
            .init_resource::<ChunkMap>()
            // --- Подключаемые плагины ---
            .add_plugins((
                SettingsPlugin::new("org.bevy.examples.settings"),
                CameraPlugin,
                WorldGenerationPlugin,
                HexRenderingPlugin,
                UIPlugin,
                SimulationPlugin,
            ))
            // --- Глобальные игровые системы ---
            // Запросы меню обрабатываются всегда: они нужны и в главном меню,
            // и на паузе, и во время игры.
            .add_systems(Update, handle_menu_requests)
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
    mut sim_clock: ResMut<SimClock>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_play_state.get() {
            PlayState::Playing => {
                next_play_state.set(PlayState::Paused);
                sim_clock.paused = true;
                info!("Game is paused!");
            }
            PlayState::Paused => {
                next_play_state.set(PlayState::Playing);
                sim_clock.paused = false;
                info!("Game is playing");
            }
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
