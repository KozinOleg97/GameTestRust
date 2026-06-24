use bevy::prelude::Component;

/// Действия, которые можно совершить из главного меню
#[derive(Component)]
pub enum MainMenuAction {
    StartGame,
    SaveSettings,
}