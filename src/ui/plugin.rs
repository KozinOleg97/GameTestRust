use crate::ui::main_menu::plugin::MainMenuPlugin;
use crate::ui::pause_menu::plugin::PauseMenuPlugin;

use crate::ui::interactions::InteractionPlugin;
use crate::ui::performance_overlay::plugin::PerformanceOverlayPlugin;
use crate::ui::simulation_ui::plugin::SimulationUIPlugin;
use bevy::app::App;
use bevy::prelude::Plugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PerformanceOverlayPlugin)
            .add_plugins(MainMenuPlugin)
            .add_plugins(PauseMenuPlugin)
            .add_plugins(InteractionPlugin)
            .add_plugins(SimulationUIPlugin);
    }
}
