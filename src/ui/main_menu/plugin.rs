use crate::game::GameState;
use crate::ui::main_menu::buttons::MainMenuAction;
use crate::ui::style::*;
use crate::ui::widgets::{spawn_menu_root, spawn_text_button, spawn_title};
use bevy::prelude::*;
use bevy_settings::SaveSettingsSync;
use std::thread::spawn;

fn spawn_main_menu(mut commands: Commands) {
    info!("Spawned main menu");
    commands.spawn((Camera2d, DespawnOnExit(GameState::MainMenu)));

    spawn_menu_root(&mut commands, GameState::MainMenu, |parent| {
        spawn_title(parent, "Hex Game");

        spawn_text_button(parent, "Start Game", MainMenuAction::StartGame);
        spawn_text_button(parent, "Save settings", MainMenuAction::SaveSettings);
    });
}

/// Централизованная обработка всех нажатий кнопок главного меню
fn main_menu_action(
    mut interaction_query: Query<
        (&Interaction, &MainMenuAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    for (interaction, action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                MainMenuAction::StartGame => {
                    next_state.set(GameState::Loading);
                }
                MainMenuAction::SaveSettings => {
                    commands.queue(SaveSettingsSync::IfChanged);
                    info!("Settings saved");
                }
            }
        }
    }
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Спавн UI при входе в состояние меню
            .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            // Системы, работающие только когда мы в главном меню
            .add_systems(
                Update,
                (main_menu_action).run_if(in_state(GameState::MainMenu)),
            );
    }
}
