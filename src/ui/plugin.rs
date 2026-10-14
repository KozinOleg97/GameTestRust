use crate::ui::actions::on_ui_activate;
use crate::ui::hud::hex_info::HexInfoPlugin;
use crate::ui::hud::performance_overlay::PerformanceOverlayPlugin;
use crate::ui::hud::simulation::SimulationUIPlugin;
use crate::ui::interactions::InteractionPlugin;
use crate::ui::screens::main_menu::MainMenuPlugin;
use crate::ui::screens::pause_menu::PauseMenuPlugin;
use crate::ui::theme::UiTheme;
use bevy::app::App;
use bevy::prelude::Plugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Единый визуальный язык интерфейса и единый переводчик действий.
            .init_resource::<UiTheme>()
            .add_observer(on_ui_activate)
            .add_plugins((
                MainMenuPlugin,
                PauseMenuPlugin,
                SimulationUIPlugin,
                PerformanceOverlayPlugin,
                HexInfoPlugin,
                InteractionPlugin,
            ));
    }
}
