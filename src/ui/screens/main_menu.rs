use crate::game::GameSessionState;
use crate::ui::actions::MenuAction;
use crate::ui::kit::{spawn_button, spawn_screen_root, spawn_title};
use crate::ui::theme::UiTheme;
use bevy::prelude::*;

fn spawn_main_menu(mut commands: Commands, theme: Res<UiTheme>) {
    let theme = &*theme;

    info!("Spawned main menu");

    commands.spawn((
        Camera2d,
        Camera {
            order: 10,
            ..default()
        },
        DespawnOnExit(GameSessionState::MainMenu),
    ));

    spawn_screen_root(&mut commands, theme, GameSessionState::MainMenu, |parent| {
        spawn_title(parent, theme, "Hex Game");

        spawn_button(parent, theme, "Start Game", MenuAction::StartGame);
        spawn_button(parent, theme, "Save settings", MenuAction::SaveSettings);
    });
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameSessionState::MainMenu), spawn_main_menu);
    }
}
