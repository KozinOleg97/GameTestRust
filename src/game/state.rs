use bevy::prelude::*;

// Глобальное состояние сессии (существует ли игровой мир)
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameSessionState {
    #[default]
    MainMenu,
    Loading,
    Active, // Игра запущена 
}

// Подсостояние игрового процесса (работает только когда GameSessionState::Active)
#[derive(SubStates, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
#[source(GameSessionState = GameSessionState::Active)]
pub enum PlayState {
    #[default]
    Playing,
    Paused,
}