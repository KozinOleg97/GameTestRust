use bevy::prelude::*;
use bevy_settings::SaveSettings;

use super::{GameSessionState, PlayState};

/// Запросы меню к игре.
///
/// Тип живёт в домене, а не в UI: интерфейс зависит от игры, игра про интерфейс
/// не знает. Позже это же сообщение сможет послать горячая клавиша, сценарий или
/// сетевой запрос — источник запроса домену безразличен.
#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuRequest {
    StartGame,
    SaveSettings,
    Resume,
    OpenSettings,
    QuitToMainMenu,
}

/// Обработка запросов меню.
///
/// Живёт в домене, поэтому UI больше не трогает состояния и настройки напрямую.
pub fn handle_menu_requests(
    mut requests: MessageReader<MenuRequest>,
    mut commands: Commands,
    mut next_session: ResMut<NextState<GameSessionState>>,
    mut next_play: ResMut<NextState<PlayState>>,
) {
    for request in requests.read() {
        match request {
            MenuRequest::StartGame => {
                next_session.set(GameSessionState::Loading);
                info!("Starting game session");
            }
            MenuRequest::SaveSettings => {
                commands.queue(SaveSettings::IfChanged);
                info!("Settings saved");
            }
            MenuRequest::Resume => next_play.set(PlayState::Playing),
            MenuRequest::OpenSettings => info!("Open settings (not implemented)"),
            MenuRequest::QuitToMainMenu => next_session.set(GameSessionState::MainMenu),
        }
    }
}
