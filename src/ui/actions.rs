use crate::game::{GameSessionState, PlayState};
use crate::simulation::{SimClock, TimeJumpRequest};
use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy_settings::SaveSettings;

/// Все действия, которые интерфейс может запросить у игры.
///
/// Один enum на всё приложение: экраны не заводят собственных наборов действий,
/// а единственный обработчик ниже — единственное место, где UI влияет на игру.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum UiAction {
    // Главное меню
    StartGame,
    SaveSettings,

    // Меню паузы
    Resume,
    OpenSettings,
    QuitToMainMenu,

    // Панель симуляции
    PauseToggle,
    SetSpeed(f64),
    JumpHours(u64),
    JumpDays(u64),
}

/// Обработчик всех действий интерфейса.
///
/// Вешается один раз на приложение, поэтому кнопкам не нужны собственные
/// обсерверы: кнопка несёт `UiAction`, а что с ним делать — решается здесь.
pub fn on_ui_activate(
    activate: On<Activate>,
    actions: Query<&UiAction>,
    mut commands: Commands,
    mut next_session: ResMut<NextState<GameSessionState>>,
    mut next_play: ResMut<NextState<PlayState>>,
    mut clock: ResMut<SimClock>,
    mut jump_request: ResMut<TimeJumpRequest>,
) {
    let Ok(action) = actions.get(activate.entity) else {
        return;
    };

    match *action {
        UiAction::StartGame => {
            next_session.set(GameSessionState::Loading);
            info!("Starting game session");
        }
        UiAction::SaveSettings => {
            commands.queue(SaveSettings::IfChanged);
            info!("Settings saved");
        }

        UiAction::Resume => next_play.set(PlayState::Playing),
        UiAction::OpenSettings => info!("Open settings (not implemented)"),
        UiAction::QuitToMainMenu => next_session.set(GameSessionState::MainMenu),

        UiAction::PauseToggle => {
            clock.paused = !clock.paused;
            info!("Simulation paused: {}", clock.paused);
        }
        UiAction::SetSpeed(scale) => clock.set_time_scale(scale),
        UiAction::JumpHours(hours) => {
            let ticks = (hours as f64 * 3600.0 / clock.tick_duration_safe()) as u64;
            jump_request.0 += ticks.max(1);
        }
        UiAction::JumpDays(days) => {
            let ticks = (days as f64 * 24.0 * 3600.0 / clock.tick_duration_safe()) as u64;
            jump_request.0 += ticks.max(1);
        }
    }
}
