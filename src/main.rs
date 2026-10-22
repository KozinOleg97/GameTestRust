use GameTestRust::game::GamePlugin;
use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowMode, WindowPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Mailbox,
                title: "Hex Game".to_string(),
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .run();
}
