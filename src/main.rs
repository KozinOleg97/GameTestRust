use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowPlugin};
use game::GamePlugin;

mod camera;
mod game;
mod generation;
mod hex;
mod rendering;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                title: "Hex Game".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .run();
}
