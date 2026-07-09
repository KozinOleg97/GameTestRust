use crate::game::{GameSessionState, PlayState};
use crate::ui::pause_menu::buttons::PauseMenuAction;
use crate::ui::style::*;
use crate::ui::widgets::{spawn_menu_root, spawn_text_button, spawn_title};
use bevy::prelude::*;

#[derive(Component)]
pub struct PauseMenuUI;

fn spawn_pause_menu(mut commands: Commands) {
    info!("Spawning pause menu");

    spawn_menu_root(&mut commands, PlayState::Paused, |parent| {
        spawn_title(parent, "Paused");

        spawn_text_button(parent, "Resume", PauseMenuAction::Resume);
        spawn_text_button(parent, "Open settings", PauseMenuAction::OpenSettings);
        spawn_text_button(parent, "Main menu", PauseMenuAction::QuitToMainMenu);
    });
}

fn pause_menu_action(
    mut query: Query<(&Interaction, &PauseMenuAction), (Changed<Interaction>, With<Button>)>,
    mut next_session_state: ResMut<NextState<GameSessionState>>,
    mut next_play_state: ResMut<NextState<PlayState>>,
) {
    for (interaction, action) in &mut query {
        if *interaction == Interaction::Pressed {
            match action {
                PauseMenuAction::Resume => {
                    next_play_state.set(PlayState::Playing);
                }
                PauseMenuAction::OpenSettings => {
                    info!("Open settings (not implemented)");
                    // Здесь можно отправить событие для открытия окна настроек
                }
                PauseMenuAction::QuitToMainMenu => {
                    next_session_state.set(GameSessionState::MainMenu);
                }
            }
        }
    }
}

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayState::Paused), spawn_pause_menu)
            .add_systems(
                Update,
                pause_menu_action.run_if(in_state(PlayState::Paused)),
            );
    }
}
