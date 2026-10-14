use crate::game::MenuRequest;
use crate::simulation::SimulationRequest;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;

/// Действия меню — словарь экранов главного меню и паузы.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    StartGame,
    SaveSettings,
    Resume,
    OpenSettings,
    QuitToMainMenu,
}

/// Действия панели симуляции — словарь HUD управления временем.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SimulationAction {
    PauseToggle,
    SetSpeed(f64),
    JumpHours(u64),
    JumpDays(u64),
}

/// Всё, что интерфейс может попросить у игры.
///
/// Верхний уровень — один словарь на приложение (открыл файл и видишь всё), а
/// под-словари принадлежат своим экранам и растут независимо друг от друга.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum UiAction {
    Menu(MenuAction),
    Simulation(SimulationAction),
}

impl From<MenuAction> for UiAction {
    fn from(action: MenuAction) -> Self {
        Self::Menu(action)
    }
}

impl From<SimulationAction> for UiAction {
    fn from(action: SimulationAction) -> Self {
        Self::Simulation(action)
    }
}

/// Перевод действия интерфейса в запрос домена.
///
/// Реализации живут здесь, а не в домене: домен не должен знать про словарь UI,
/// а UI знать про запросы домена — его законное право.
impl From<MenuAction> for MenuRequest {
    fn from(action: MenuAction) -> Self {
        match action {
            MenuAction::StartGame => MenuRequest::StartGame,
            MenuAction::SaveSettings => MenuRequest::SaveSettings,
            MenuAction::Resume => MenuRequest::Resume,
            MenuAction::OpenSettings => MenuRequest::OpenSettings,
            MenuAction::QuitToMainMenu => MenuRequest::QuitToMainMenu,
        }
    }
}

impl From<SimulationAction> for SimulationRequest {
    fn from(action: SimulationAction) -> Self {
        match action {
            SimulationAction::PauseToggle => SimulationRequest::PauseToggle,
            SimulationAction::SetSpeed(scale) => SimulationRequest::SetSpeed(scale),
            SimulationAction::JumpHours(hours) => SimulationRequest::JumpHours(hours),
            SimulationAction::JumpDays(days) => SimulationRequest::JumpDays(days),
        }
    }
}

/// Единственное место, где интерфейс говорит с игрой.
///
/// Наблюдатель ничего не делает сам: он только переводит нажатие в запрос
/// владельца. Поэтому у него нет ни `SimClock`, ни состояний — и добавить новое
/// действие теперь значит добавить ветку, а не ещё один ресурс в подпись.
pub fn on_ui_activate(
    activate: On<Activate>,
    actions: Query<&UiAction>,
    mut menu_requests: MessageWriter<MenuRequest>,
    mut simulation_requests: MessageWriter<SimulationRequest>,
) {
    let Ok(action) = actions.get(activate.entity) else {
        return;
    };

    match *action {
        UiAction::Menu(action) => {
            menu_requests.write(action.into());
        }
        UiAction::Simulation(action) => {
            simulation_requests.write(action.into());
        }
    }
}
