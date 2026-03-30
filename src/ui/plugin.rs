use crate::ui::main_menu::main_menu::MainMenuPlugin;
use crate::ui::performance_overlay::performance_overlay::PerformanceOverlayPlugin;
use bevy::app::App;
use bevy::prelude::Plugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PerformanceOverlayPlugin)
            .add_plugins(MainMenuPlugin);
    }
}
