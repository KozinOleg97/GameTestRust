use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowPlugin};
use GameTestRust::game::GamePlugin;

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
