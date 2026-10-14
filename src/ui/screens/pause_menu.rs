use crate::game::PlayState;
use crate::ui::actions::MenuAction;
use crate::ui::kit::{spawn_button, spawn_screen_root, spawn_title};
use crate::ui::theme::UiTheme;
use bevy::prelude::*;

fn spawn_pause_menu(mut commands: Commands, theme: Res<UiTheme>) {
    let theme = &*theme;

    info!("Spawning pause menu");

    spawn_screen_root(&mut commands, theme, PlayState::Paused, |parent| {
        spawn_title(parent, theme, "Paused");

        spawn_button(parent, theme, "Resume", MenuAction::Resume);
        spawn_button(parent, theme, "Open settings", MenuAction::OpenSettings);
        spawn_button(parent, theme, "Main menu", MenuAction::QuitToMainMenu);
    });
}

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayState::Paused), spawn_pause_menu);
    }
}
