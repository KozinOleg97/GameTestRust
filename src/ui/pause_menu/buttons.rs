use bevy::prelude::Component;

#[derive(Component)]
pub enum PauseMenuAction {
    Resume,
    OpenSettings,
    QuitToMainMenu,
}
