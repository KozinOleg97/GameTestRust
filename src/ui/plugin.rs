use crate::ui::main_menu::plugin::MainMenuPlugin;
use crate::ui::pause_menu::plugin::PauseMenuPlugin;



use bevy::app::App;
use bevy::prelude::{BackgroundColor, Button, Changed, Interaction, Plugin, Query, Update, With};
use crate::ui::interactions::InteractionPlugin;
use crate::ui::performance_overlay::plugin::PerformanceOverlayPlugin;
use crate::ui::style::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PerformanceOverlayPlugin)
            .add_plugins(MainMenuPlugin)
            .add_plugins(PauseMenuPlugin)
            .add_plugins(InteractionPlugin);
    }
}

